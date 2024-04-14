mod interactive;
mod non_interactive;
mod prover;
mod serialize;
mod transcript;
mod utils;
mod verifier;

use ark_bls12_381::Fq;
use ark_poly::{
    polynomial::multivariate::{SparsePolynomial, SparseTerm, Term},
    DenseMVPolynomial, Polynomial,
};
use non_interactive::non_interactive_protocol;
use verifier::Verifier;

use crate::interactive::interactive_protocol;
use prover::Prover;

fn main() {
    // g(x_0, x_1, x_2) = 2*x_0^3 + x_0*x_2 + x_1*x_2
    let g = SparsePolynomial::from_coefficients_vec(
        3,
        vec![
            (Fq::from(2), SparseTerm::new(vec![(0, 3)])),
            (Fq::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),
            (Fq::from(1), SparseTerm::new(vec![(1, 1), (2, 1)])),
        ],
    );

    let p = Prover::new(&g).unwrap();
    let v = Verifier::new(&g);

    println!("[sumcheck] starting interactive protocol");
    let mut valid = interactive_protocol(&p, &v);
    if !valid {
        panic!("[sumcheck] interactive protocol is not valid");
    }
    println!("[sumcheck] interactive protocol is valid");

    println!("[sumcheck] starting non-interactive protocol");
    let proof = non_interactive_protocol(&p);
    valid = v.verify_non_interactive_proof(&p.g, &p.h, p.g.degree(), &proof);
    if !valid {
        panic!("[sumcheck] interactive protocol is not valid");
    }
    println!("[sumcheck] non-interactive protocol is valid");
}
