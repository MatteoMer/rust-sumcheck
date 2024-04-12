mod interactive;
mod prover;
mod verifier;

use ark_bls12_381::Fq;
use ark_poly::{
    polynomial::multivariate::{SparsePolynomial, SparseTerm, Term},
    DenseMVPolynomial,
};
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

    interactive_protocol(&p, &v);
}
