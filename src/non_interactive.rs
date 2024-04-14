use ark_ff::Field;
use ark_poly::{multivariate::SparseTerm, univariate::SparsePolynomial};

use crate::{prover::Prover, verifier::Verifier};

pub fn build_non_interactive_proof<F>(
    p: &Prover<F, SparsePolynomial<F, SparseTerm>>,
    v: &Verifier<F, SparsePolynomial<F, SparseTerm>>,
) where
    F: Field + From<i32>,
{
}
