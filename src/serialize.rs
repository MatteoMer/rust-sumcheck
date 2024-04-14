use ark_ff::Field;
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial};

pub trait Serialize<F> {
    fn serialize(&self) -> Vec<F>;
}

impl<F: Field> Serialize<F> for DensePolynomial<F> {
    fn serialize(&self) -> Vec<F> {
        self.coeffs().to_vec()
    }
}
