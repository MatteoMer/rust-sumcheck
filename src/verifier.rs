use ark_ff::Field;
use ark_poly::{univariate::DensePolynomial, DenseMVPolynomial, Polynomial};
use rand::Rng;
use std::marker::PhantomData;

pub struct Verifier<F, P>
where
    F: Field,
    P: DenseMVPolynomial<F>,
{
    g: P,
    _phantom: PhantomData<F>,
}

impl<F, P> Verifier<F, P>
where
    F: Field + From<i32>,
    P: DenseMVPolynomial<F>,
{
    pub fn send_random_challenge(&self) -> F {
        let mut rng = rand::thread_rng();

        let r: u8 = rng.gen();
        F::from(2)
    }

    // get degree of a variable of the multivariate polynomial
    fn get_variable_degree(&self, variable: usize) -> usize {
        let terms = &self.g.terms();
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

    pub fn check_claim(&self, g_j: &DensePolynomial<F>, c_j: F, round: usize) -> bool {
        // check if g_j(0) + g_j(1) = c_j
        let eval_zero = g_j.evaluate(&F::zero());
        let eval_one = g_j.evaluate(&F::one());
        println!("{:?}", g_j.to_vec());
        println!("{}+{}={}", eval_zero, eval_one, c_j);
        if eval_zero + eval_one != c_j {
            return false;
        }

        // check if deg(g_j) <= deg_j(g)
        let deg_g = self.get_variable_degree(round);
        let deg_g_j = g_j.degree();
        if deg_g_j > deg_g {
            return false;
        }

        true
    }
    pub fn new(g: &P) -> Self {
        Self {
            g: g.clone(),
            _phantom: PhantomData,
        }
    }
}
