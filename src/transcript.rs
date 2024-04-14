use ark_ff::Field;
use ark_poly::{univariate::DensePolynomial as UniPolynomial, DenseUVPolynomial};
use merlin::Transcript;

pub trait SumcheckTranscript<F>
where
    F: Field,
{
    fn append_poly(&mut self, label: &'static [u8], poly: &UniPolynomial<F>);
    fn append_scalar(&mut self, label: &'static [u8], scalar: &F);
}

impl<F: Field> SumcheckTranscript<F> for Transcript {
    fn append_scalar(&mut self, label: &'static [u8], scalar: &F) {
        let mut buf = vec![];
        scalar.serialize_compressed(&mut buf).unwrap();
        self.append_message(label, &buf);
    }

    fn append_poly(&mut self, label: &'static [u8], poly: &UniPolynomial<F>) {
        self.append_message(label, b"append_poly_begin");

        for i in 0..poly.coeffs().len() {
            self.append_scalar(label, &poly.coeffs[i])
        }

        self.append_message(label, b"append_poly_end");
    }
}
