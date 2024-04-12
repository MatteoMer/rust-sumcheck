use ark_ff::Field;
use ark_poly::{
    multivariate::{SparsePolynomial, SparseTerm},
    univariate::DensePolynomial,
    DenseMVPolynomial, DenseUVPolynomial, Polynomial,
};

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

        // Initialize the coefficients for the univariate polynomial.
        let mut coefficients: Vec<F> = vec![F::zero(); max_degree + 1];

        // Iterate over all combinations for the unfixed variables.
        for combo in 0..(1 << (num_vars - fixed_vars.len())) {
            let mut point: Vec<F> = fixed_vars.to_vec(); // Start with the fixed values
            point.resize(num_vars, F::zero()); // Extend the point vector to the full size

            // Set the binary values for the unfixed variables.
            for j in fixed_vars.len()..num_vars {
                if j != variable_to_keep {
                    point[j] = F::from(((combo >> (j - fixed_vars.len())) & 1) as u64);
                }
            }

            // Evaluate the polynomial for each power of variable_to_keep and add to coefficients
            for power in 0..=max_degree {
                point[variable_to_keep] = F::from(power as u64);
                coefficients[power] = coefficients[power] + poly.evaluate(&point);
            }
        }
        DensePolynomial::from_coefficients_vec(coefficients)
    }
}
