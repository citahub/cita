use pairing::*;
use domain::{Scalar, EvaluationDomain};
use ::{
    ConstraintSystem,
    PublicConstraintSystem,
    Circuit,
    Input,
    Index,
    Error,
    Variable,
    LinearCombination
};
use multiexp::*;
use super::{ParameterSource, Proof};
use rand::Rng;
use std::sync::Arc;
use futures::Future;
use futures_cpupool::CpuPool;

pub fn create_random_proof<E, C, R, P: ParameterSource<E>>(
    circuit: C,
    params: P,
    rng: &mut R
) -> Result<Proof<E>, Error>
    where E: Engine, C: Circuit<E>, R: Rng
{
    let r = rng.gen();
    let s = rng.gen();

    create_proof::<E, C, P>(circuit, params, r, s)
}

pub fn create_proof<E, C, P: ParameterSource<E>>(
    circuit: C,
    mut params: P,
    r: E::Fr,
    s: E::Fr
) -> Result<Proof<E>, Error>
    where E: Engine, C: Circuit<E>
{
    struct ProvingAssignment<E: Engine> {
        // Density of queries
        a_aux_density: DensityTracker,
        b_input_density: DensityTracker,
        b_aux_density: DensityTracker,

        // Evaluations of A, B, C polynomials
        a: Vec<Scalar<E>>,
        b: Vec<Scalar<E>>,
        c: Vec<Scalar<E>>,

        // Assignments of variables
        input_assignment: Vec<E::Fr>,
        aux_assignment: Vec<E::Fr>
    }

    impl<E: Engine> PublicConstraintSystem<E> for ProvingAssignment<E> {
        fn alloc_input<F: FnOnce() -> Result<E::Fr, Error>>(&mut self, value: F) -> Result<Variable, Error> {
            self.input_assignment.push(value()?);
            self.b_input_density.add_element();

            Ok(Variable(Index::Input(self.input_assignment.len() - 1)))
        }
    }

    impl<E: Engine> ConstraintSystem<E> for ProvingAssignment<E> {
        fn alloc<F: FnOnce() -> Result<E::Fr, Error>>(&mut self, value: F) -> Result<Variable, Error> {
            self.aux_assignment.push(value()?);
            self.a_aux_density.add_element();
            self.b_aux_density.add_element();

            Ok(Variable(Index::Aux(self.aux_assignment.len() - 1)))
        }

        fn enforce(
            &mut self,
            a: LinearCombination<E>,
            b: LinearCombination<E>,
            c: LinearCombination<E>
        )
        {
            self.a.push(Scalar(a.eval(None, Some(&mut self.a_aux_density), &self.input_assignment, &self.aux_assignment)));
            self.b.push(Scalar(b.eval(Some(&mut self.b_input_density), Some(&mut self.b_aux_density), &self.input_assignment, &self.aux_assignment)));
            self.c.push(Scalar(c.eval(None, None, &self.input_assignment, &self.aux_assignment)));
        }
    }

    let mut prover = ProvingAssignment {
        a_aux_density: DensityTracker::new(),
        b_input_density: DensityTracker::new(),
        b_aux_density: DensityTracker::new(),
        a: vec![],
        b: vec![],
        c: vec![],
        input_assignment: vec![],
        aux_assignment: vec![]
    };

    prover.alloc_input(|| Ok(E::Fr::one()))?;

    circuit.synthesize(&mut prover)?.synthesize(&mut prover)?;

    // Input consistency constraints: x * 0 = 0
    for i in 0..prover.input_assignment.len() {
        prover.enforce(LinearCombination::zero() + Variable(Index::Input(i)),
                       LinearCombination::zero(),
                       LinearCombination::zero());
    }

    let cpupool = CpuPool::new_num_cpus();

    let vk = params.get_vk(prover.input_assignment.len())?;

    let h = {
        let mut a = EvaluationDomain::from_coeffs(prover.a)?;
        let mut b = EvaluationDomain::from_coeffs(prover.b)?;
        let mut c = EvaluationDomain::from_coeffs(prover.c)?;
        a.ifft();
        a.coset_fft();
        b.ifft();
        b.coset_fft();
        c.ifft();
        c.coset_fft();

        a.mul_assign(&b);
        drop(b);
        a.sub_assign(&c);
        drop(c);
        a.divide_by_z_on_coset();
        a.icoset_fft();
        let mut a = a.into_coeffs();
        let a_len = a.len() - 1;
        a.truncate(a_len);
        // TODO: parallelize if it's even helpful
        let a = Arc::new(a.into_iter().map(|s| s.0.into_repr()).collect::<Vec<_>>());

        multiexp(&cpupool, params.get_h(a.len())?, FullDensity, a)
    };

    // TODO: parallelize if it's even helpful
    let input_assignment = Arc::new(prover.input_assignment.into_iter().map(|s| s.into_repr()).collect::<Vec<_>>());
    let aux_assignment = Arc::new(prover.aux_assignment.into_iter().map(|s| s.into_repr()).collect::<Vec<_>>());

    let l = multiexp(&cpupool, params.get_l(aux_assignment.len())?, FullDensity, aux_assignment.clone());

    let a_aux_density_total = prover.a_aux_density.get_total_density();

    let (a_inputs_source, a_aux_source) = params.get_a(input_assignment.len(), a_aux_density_total)?;

    let a_inputs = multiexp(&cpupool, a_inputs_source, FullDensity, input_assignment.clone());
    let a_aux = multiexp(&cpupool, a_aux_source, Arc::new(prover.a_aux_density), aux_assignment.clone());

    let b_input_density = Arc::new(prover.b_input_density);
    let b_input_density_total = b_input_density.get_total_density();
    let b_aux_density = Arc::new(prover.b_aux_density);
    let b_aux_density_total = b_aux_density.get_total_density();

    let (b_g1_inputs_source, b_g1_aux_source) = params.get_b_g1(b_input_density_total, b_aux_density_total)?;

    let b_g1_inputs = multiexp(&cpupool, b_g1_inputs_source, b_input_density.clone(), input_assignment.clone());
    let b_g1_aux = multiexp(&cpupool, b_g1_aux_source, b_aux_density.clone(), aux_assignment.clone());

    let (b_g2_inputs_source, b_g2_aux_source) = params.get_b_g2(b_input_density_total, b_aux_density_total)?;
    
    let b_g2_inputs = multiexp(&cpupool, b_g2_inputs_source, b_input_density, input_assignment.clone());
    let b_g2_aux = multiexp(&cpupool, b_g2_aux_source, b_aux_density, aux_assignment);

    drop(input_assignment);

    let mut g_a = vk.delta_g1.mul(r);
    g_a.add_assign_mixed(&vk.alpha_g1);
    let mut g_b = vk.delta_g2.mul(s);
    g_b.add_assign_mixed(&vk.beta_g2);
    let mut g_c;
    {
        let mut rs = r;
        rs.mul_assign(&s);

        g_c = vk.delta_g1.mul(rs);
        g_c.add_assign(&vk.alpha_g1.mul(s));
        g_c.add_assign(&vk.beta_g1.mul(r));
    }
    let mut a_answer = a_inputs.wait()?;
    a_answer.add_assign(&a_aux.wait()?);
    g_a.add_assign(&a_answer);
    a_answer.mul_assign(s);
    g_c.add_assign(&a_answer);

    let mut b1_answer = b_g1_inputs.wait()?;
    b1_answer.add_assign(&b_g1_aux.wait()?);
    let mut b2_answer = b_g2_inputs.wait()?;
    b2_answer.add_assign(&b_g2_aux.wait()?);

    g_b.add_assign(&b2_answer);
    b1_answer.mul_assign(r);
    g_c.add_assign(&b1_answer);
    g_c.add_assign(&h.wait()?);
    g_c.add_assign(&l.wait()?);

    Ok(Proof {
        a: g_a.into_affine(),
        b: g_b.into_affine(),
        c: g_c.into_affine()
    })
}
