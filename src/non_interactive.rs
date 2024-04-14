// Non-interactive version of the protocol

use crate::{prover::Prover, serialize::Serialize, transcript::SumcheckTranscript};
use ark_ff::Field;
use ark_poly::{
    multivariate::{SparsePolynomial, SparseTerm},
    DenseMVPolynomial,
};
use merlin::Transcript;

// The prover wants to convince a verifier that he knows some h such that h is the sum of every
// evaluation of a polynomial g over the boolean hypercube
// Description of the protocol: https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.pdf
// Chapter 4.1

pub fn non_interactive_protocol<F>(p: &Prover<F, SparsePolynomial<F, SparseTerm>>) -> Vec<Vec<F>>
where
    F: Field + From<i32>,
{
    let nb_rounds = p.g.num_vars();
    let mut r: Vec<F> = Vec::new();
    let mut transcript = Transcript::new(b"Sumcheck transcript");
    let mut proof: Vec<Vec<F>> = Vec::new();

    // first round
    let g_1 = Prover::construct_uni_poly(&p.g, &r, 0);

    transcript.append_poly(b"g_i", &g_1);

    let mut bytes = [0u8; 32];
    transcript.challenge_bytes(b"challenge_i", &mut bytes);
    let r_i = F::from_random_bytes(&bytes).unwrap();
    r.push(r_i);
    proof.push(g_1.serialize());

    // j-th rounds
    for round in 1..nb_rounds - 1 {
        let g_i = Prover::construct_uni_poly(&p.g, &r, round);
        transcript.append_poly(b"g_i", &g_i);

        let mut bytes = [0u8; 32];
        transcript.challenge_bytes(b"challenge_i", &mut bytes);

        let r_i = F::from_random_bytes(&bytes).unwrap();
        r.push(r_i);
        proof.push(g_i.serialize());
    }

    // last round
    let g_v = Prover::construct_uni_poly(&p.g, &r, nb_rounds - 1);
    transcript.append_poly(b"g_i", &g_v);

    let mut bytes = [0u8; 32];
    transcript.challenge_bytes(b"challenge_i", &mut bytes);

    let r_i = F::from_random_bytes(&bytes).unwrap();
    r.push(r_i);

    proof.push(g_v.serialize());

    proof
}
