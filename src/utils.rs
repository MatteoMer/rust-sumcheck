use ark_ff::Field;
use ark_poly::multivariate::Term;
use ark_poly::DenseMVPolynomial;

// get degree of a variable of the multivariate polynomial
pub fn get_variable_degree<F, P>(g: &P, variable: usize) -> usize
where
    F: Field,
    P: DenseMVPolynomial<F>,
{
    let mut max = 0usize;

    for (_c, t) in g.terms().iter() {
        for (&var, pow) in t.vars().iter().zip(t.powers()) {
            if var == variable && pow > max {
                max = pow
            }
        }
    }

    max
}
