use crate::{transcript::SumcheckTranscript, utils::get_variable_degree};
use ark_ff::{Field, Zero};
use ark_poly::{
    multivariate::{SparsePolynomial, SparseTerm},
    univariate::DensePolynomial,
    DenseMVPolynomial, DenseUVPolynomial, Polynomial,
};
use merlin::Transcript;
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
        F::from(r)
    }

    pub fn check_claim(&self, g_j: &DensePolynomial<F>, c_j: F, round: usize) -> bool {
        // check if g_j(0) + g_j(1) = c_j
        let eval_zero = g_j.evaluate(&F::zero());
        let eval_one = g_j.evaluate(&F::one());
        if eval_zero + eval_one != c_j {
            return false;
        }

        // check if deg(g_j) <= deg_j(g)
        let deg_g = get_variable_degree(&self.g, round);
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

    pub fn verify_non_interactive_proof(
        &self,
        g: &SparsePolynomial<F, SparseTerm>,
        h: &F,
        degree: usize,
        proof: &Vec<Vec<F>>,
    ) -> bool {
        let mut c_i = *h;
        let mut r: Vec<F> = Vec::new();
        let mut r_i: F = F::zero();
        let mut g_i: DensePolynomial<F> = DensePolynomial::zero();

        let mut transcript = Transcript::new(b"Sumcheck transcript");

        assert_eq!(proof.len(), degree);

        for i in 0..proof.len() {
            let coeffs = &proof[i];
            g_i = DensePolynomial::from_coefficients_vec(coeffs.to_vec());

            if g_i.degree() > degree {
                panic!(
                    "[sumcheck-non-interactive] verification failed at round: {}",
                    i
                );
            }

            assert_eq!(g_i.evaluate(&F::zero()) + g_i.evaluate(&F::one()), c_i);

            transcript.append_poly(b"g_i", &g_i);

            let mut bytes = [0u8; 32];
            transcript.challenge_bytes(b"challenge_i", &mut bytes);
            r_i = F::from_random_bytes(&bytes).unwrap();
            r.push(r_i);
            c_i = g_i.evaluate(&r_i);
        }
        assert_eq!(g_i.evaluate(&r_i), g.evaluate(&r));

        true
    }
}
