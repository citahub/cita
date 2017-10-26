use pairing::*;
use ::{
    Input,
    Error,
    LinearCombination,
    Index,
    Variable,
    ConstraintSystem,
    PublicConstraintSystem
};
use super::{Proof, VerifyingKey, PreparedVerifyingKey};

/// This is the constraint system synthesizer that is made available to
/// callers of the verification function when they wish to perform
/// allocations. In that context, allocation of inputs is not allowed.
pub struct VerifierInput<'a, E: Engine> {
    acc: E::G1,
    ic: &'a [E::G1Affine],
    insufficient_inputs: bool,
    num_inputs: usize,
    num_aux: usize
}

impl<'a, E: Engine> ConstraintSystem<E> for VerifierInput<'a, E> {
    fn alloc<F: FnOnce() -> Result<E::Fr, Error>>(&mut self, f: F) -> Result<Variable, Error> {
        // Run the function for calculating the allocation but ignore the output,
        // since we don't care about the assignment of auxillary variables during
        // verification.
        let _ = f();

        let index = self.num_aux;
        self.num_aux += 1;

        Ok(Variable(Index::Aux(index)))
    }

    fn enforce(
        &mut self,
        _: LinearCombination<E>,
        _: LinearCombination<E>,
        _: LinearCombination<E>
    )
    {
        // Do nothing; we don't care about the constraint system
        // in this context.
    }
}

/// This is intended to be a wrapper around VerifierInput that is kept
/// private and used for input allocation.
struct InputAllocator<T>(T);

impl<'a, 'b, E: Engine> ConstraintSystem<E> for InputAllocator<&'a mut VerifierInput<'b, E>> {
    fn alloc<F: FnOnce() -> Result<E::Fr, Error>>(&mut self, value: F) -> Result<Variable, Error> {
        self.0.alloc(value)
    }

    fn enforce(
        &mut self,
        _: LinearCombination<E>,
        _: LinearCombination<E>,
        _: LinearCombination<E>
    )
    {
        // Do nothing; we don't care about the constraint system
        // in this context.
    }
}

impl<'a, 'b, E: Engine> PublicConstraintSystem<E> for InputAllocator<&'a mut VerifierInput<'b, E>> {
    fn alloc_input<F: FnOnce() -> Result<E::Fr, Error>>(&mut self, value: F) -> Result<Variable, Error> {
        if self.0.ic.len() == 0 {
            self.0.insufficient_inputs = true;
        } else {
            self.0.acc.add_assign(&self.0.ic[0].mul(value()?));
            self.0.ic = &self.0.ic[1..];
        }

        let index = self.0.num_inputs;
        self.0.num_inputs += 1;

        Ok(Variable(Index::Input(index)))
    }
}

pub fn verify_proof<'a, E, C, F>(
    pvk: &'a PreparedVerifyingKey<E>,
    proof: &Proof<E>,
    circuit: F
) -> Result<bool, Error>
    where E: Engine, C: Input<E>, F: FnOnce(&mut VerifierInput<'a, E>) -> Result<C, Error>
{
    let mut witness = VerifierInput::<E> {
        acc: pvk.ic[0].into_projective(),
        ic: &pvk.ic[1..],
        insufficient_inputs: false,
        num_inputs: 1,
        num_aux: 0
    };

    circuit(&mut witness)?.synthesize(&mut InputAllocator(&mut witness))?;

    if witness.ic.len() != 0 || witness.insufficient_inputs {
        return Err(Error::MalformedVerifyingKey);
    }

    // The original verification equation is:
    // A * B = alpha * beta + inputs * gamma + C * delta
    // ... however, we rearrange it so that it is:
    // A * B - inputs * gamma - C * delta = alpha * beta
    // or equivalently:
    // A * B + inputs * (-gamma) + C * (-delta) = alpha * beta
    // which allows us to do a single final exponentiation.

    Ok(E::final_exponentiation(
        &E::miller_loop([
            (&proof.a.prepare(), &proof.b.prepare()),
            (&witness.acc.into_affine().prepare(), &pvk.neg_gamma_g2),
            (&proof.c.prepare(), &pvk.neg_delta_g2)
        ].into_iter())
    ).unwrap() == pvk.alpha_g1_beta_g2)
}

pub fn prepare_verifying_key<E: Engine>(
    vk: &VerifyingKey<E>
) -> PreparedVerifyingKey<E>
{
    let mut gamma = vk.gamma_g2;
    gamma.negate();
    let mut delta = vk.delta_g2;
    delta.negate();

    PreparedVerifyingKey {
        alpha_g1_beta_g2: E::pairing(vk.alpha_g1, vk.beta_g2),
        neg_gamma_g2: gamma.prepare(),
        neg_delta_g2: delta.prepare(),
        ic: vk.ic.clone()
    }
}
