// Interactive version of the protocol

use ark_ff::Field;
use ark_poly::{
    multivariate::{SparsePolynomial, SparseTerm},
    DenseMVPolynomial, Polynomial,
};

use crate::{prover::Prover, verifier::Verifier};

// The prover wants to convince a verifier that he knows some h such that h is the sum of every
// evaluation of a polynomial g over the boolean hypercube
// Description of the protocol: https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf
// Chapter 4.1

pub fn interactive_protocol<F>(
    p: &Prover<F, SparsePolynomial<F, SparseTerm>>,
    v: &Verifier<F, SparsePolynomial<F, SparseTerm>>,
) -> bool
where
    F: Field + From<i32>,
{
    let nb_rounds = p.g.num_vars();
    let mut r: Vec<F> = Vec::new();

    // first round
    let g_1 = Prover::construct_uni_poly(&p.g, &r, 0);
    if !v.check_claim(&g_1, p.h, 0) {
        panic!("[sumcheck-interactive] claimed failed at first round");
    }
    let mut r_i = v.send_random_challenge();
    r.push(r_i);

    let mut c_i = g_1.evaluate(&r_i);

    // j-th rounds
    for round in 1..nb_rounds - 1 {
        let g_i = Prover::construct_uni_poly(&p.g, &r, round);

        if !v.check_claim(&g_i, c_i, round) {
            panic!("[sumcheck-interactive] claimed failed at round {}", round);
        }
        r_i = v.send_random_challenge();
        r.push(r_i);
        c_i = g_i.evaluate(&r_i);
    }

    // last round
    let g_v = Prover::construct_uni_poly(&p.g, &r, nb_rounds - 1);
    if !v.check_claim(&g_v, c_i, nb_rounds - 1) {
        panic!("[sumcheck-interactive] claimed failed at last round");
    }
    r_i = v.send_random_challenge();
    r.push(r_i);
    if p.g.evaluate(&r) != g_v.evaluate(&r_i) {
        panic!("[sumcheck-interactive] claimed failed at last evaluation");
    }

    true
}
