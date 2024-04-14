use ark_ff::Field;
use ark_poly::{
    multivariate::{SparsePolynomial, SparseTerm, Term},
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

    // attribution: https://github.com/punwai/sumcheck/blob/main/src/main.rs
    pub fn construct_uni_poly(
        g: &SparsePolynomial<F, SparseTerm>,
        r_i: &[F],
        round: usize,
    ) -> DensePolynomial<F> {
        let mut coefficients = vec![F::zero(); g.degree() + 1];
        let v = g.num_vars();

        // number of inputs to generate, we substract round because it's the nb of already known
        // inputs at the round; at round 1 we will have r_i.len() = 1
        for i in 0..2i32.pow((v - round - 1) as u32) {
            let mut inputs: Vec<F> = vec![];
            // adding inputs from previous rounds
            inputs.extend(r_i);
            // adding round variable
            inputs.push(F::zero());
            // generating inputs for the rest of the variables
            let mut counter = i;
            for _ in 0..(v - round - 1) {
                if counter % 2 == 0 {
                    inputs.push(0.into());
                } else {
                    inputs.push(1.into());
                }
                counter /= 2;
            }

            //computing polynomial coef from evaluation
            for (c, t) in g.terms.clone().into_iter() {
                let mut c_acc = F::one();
                let mut which = 0;

                for (&var, pow) in t.vars().iter().zip(t.powers()) {
                    if var == round {
                        which = pow;
                    } else {
                        c_acc *= inputs[var].pow([pow as u64]);
                    }
                }
                coefficients[which] += c * c_acc;
            }
        }

        DensePolynomial::from_coefficients_vec(coefficients)
    }
}
