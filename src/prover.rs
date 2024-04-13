use ark_ff::Field;
use ark_poly::{
    multivariate::{SparsePolynomial, SparseTerm},
    univariate::DensePolynomial,
    DenseMVPolynomial, DenseUVPolynomial, Evaluations, Polynomial,
};

use crate::utils::get_variable_degree;

pub struct Prover<F, P>
where
    P: DenseMVPolynomial<F>,
    F: Field + From<i32>,
{
    pub g: P,
    pub h: F,
}

impl<F> Prover<F, SparsePolynomial<F, SparseTerm>>
where
    F: Field + From<i32>,
{
    fn sum_evaluation(g: &SparsePolynomial<F, SparseTerm>) -> Option<F> {
        let v = g.num_vars();
        let mut sum = F::zero();

        // all {0,1} combination
        for i in 0..(1 << v) {
            let point: Vec<F> = (0..v)
                .map(|d| {
                    if (i >> d) & 1 == 1 {
                        F::from(1)
                    } else {
                        F::from(0)
                    }
                })
                .collect();

            // Evaluates the polynomial at the generated point, summing the results.
            sum = sum.add(&g.evaluate(&point));
        }
        Some(sum)
    }
    pub fn new(g: &SparsePolynomial<F, SparseTerm>) -> Option<Self> {
        Some(Self {
            g: g.clone(),
            h: Prover::sum_evaluation(g)?,
        })
    }

    pub fn construct_univariate(
        g: &SparsePolynomial<F, SparseTerm>,
        r_i: &[F],
        round: usize,
    ) -> DensePolynomial<F> {
        let v = g.num_vars();
        let d = get_variable_degree(g, round);

        // Initialize the coefficients for the univariate polynomial.
        let mut coefficients: Vec<F> = vec![F::zero(); v + 1];

        // Iterate over all combinations for the unfixed variables.
        for combo in 0..(1 << (v - r_i.len())) {
            let mut point: Vec<F> = r_i.to_vec(); // Start with the fixed values
            point.resize(v, F::zero()); // Extend the point vector to the full size

            // Set the binary values for the unfixed variables.
            for j in r_i.len()..v {
                if j != round {
                    point[j] = F::from(((combo >> (j - r_i.len())) & 1) as u64);
                }
            }

            point[round] = F::zero(); // Set the variable to keep to 0
            coefficients[0] += g.evaluate(&point);

            point[round] = F::one(); // Set the variable to keep to 1
            coefficients[1] += g.evaluate(&point);
        }
        DensePolynomial::from_coefficients_vec(coefficients)
    }

    pub fn construct_univariate_eval(
        g: &SparsePolynomial<F, SparseTerm>,
        r_i: &[F],
        round: usize,
    ) -> Evaluations<F> {
        let v = g.num_vars();
        let mut coefficients: Vec<F> = vec![F::zero(); v + 1];

        for i in 0..(v + 1) {
            let mut point: Vec<F> = vec![F::zero(); v];
            for j in 0..v {
                point[j] = F::from(((i >> (j - r_i.len())) & 1) as u64);
            }
            point.pop();
            point.insert(0, F::zero());

            coefficients[i] = g.evaluate(&point);
            point[0] = F::one();
            coefficients[i] += g.evaluate(&point);
        }

        Evaluations
    }
}
