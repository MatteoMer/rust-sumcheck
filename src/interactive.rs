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
    let mut r_i: Vec<F> = Vec::new();

    // first round
    let g_1 = Prover::construct_univariate(&p.g, &r_i, 0);
    if !v.check_claim(&g_1, p.h, 0) {
        panic!("claimed failed at first round");
    }
    let mut r = v.send_random_challenge();
    r_i.push(r);

    println!("g_1:{:?}\nr_1:{}", g_1.to_vec(), r);
    let mut c_i = g_1.evaluate(&r);

    // j-th rounds
    for round in 1..nb_rounds - 1 {
        let g_i = Prover::construct_univariate(&p.g, &r_i, round);

        if !v.check_claim(&g_i, c_i, round) {
            panic!("claimed failed at round {}", round + 1);
        }
        r = v.send_random_challenge();
        r_i.push(r);
        c_i = g_i.evaluate(&r);
    }

    // last round
    let g_v = Prover::construct_univariate(&p.g, &r_i, nb_rounds - 1);
    if !v.check_claim(&g_v, c_i, nb_rounds - 1) {
        panic!("claimed failed at last round");
    }
    r = v.send_random_challenge();
    r_i.push(r);
    if p.g.evaluate(&r_i) != g_v.evaluate(&r) {
        panic!("claimed failed at last evaluation");
    }

    true
}
