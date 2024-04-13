use ark_ff::Field;
use ark_poly::DenseMVPolynomial;

// get degree of a variable of the multivariate polynomial
pub fn get_variable_degree<F, P>(g: &P, variable: usize) -> usize
where
    F: Field,
    P: DenseMVPolynomial<F>,
{
    let terms = g.terms();
    let mut max = 0usize;

    for i in 0..terms.len() {
        // there's probably a better way to get the degree lol
        if terms[i].1.len() != 0 && terms[i].1[0].0 == variable {
            let deg = terms[i].1[0].1;
            if deg > max {
                max = deg;
            }
        }
    }
    max
}
