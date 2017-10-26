extern crate pairing;
extern crate rand;
extern crate bit_vec;
extern crate futures;
extern crate futures_cpupool;
extern crate num_cpus;
extern crate crossbeam;

use pairing::{Engine, Field};
use std::ops::{Add, Sub};
use std::io;

pub mod multicore;
pub mod domain;
pub mod groth16;

pub mod multiexp;
// TODO: remove this from public API?
pub use self::multiexp::{DensityTracker, FullDensity, multiexp};

#[derive(Debug)]
pub enum Error {
    PolynomialDegreeTooLarge,
    MalformedVerifyingKey,
    AssignmentMissing,
    UnexpectedIdentity,
    UnconstrainedVariable(Variable),
    IoError(io::Error)
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IoError(e)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Variable(Index);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Index {
    Input(usize),
    Aux(usize)
}

pub struct LinearCombination<E: Engine>(Vec<(Index, E::Fr)>);

impl<E: Engine> Clone for LinearCombination<E> {
    fn clone(&self) -> LinearCombination<E> {
        LinearCombination(self.0.clone())
    }
}

impl<E: Engine> LinearCombination<E> {
    pub fn zero() -> LinearCombination<E> {
        LinearCombination(vec![])
    }

    pub fn eval(
        self,
        mut input_density: Option<&mut DensityTracker>,
        mut aux_density: Option<&mut DensityTracker>,
        input_assignment: &[E::Fr],
        aux_assignment: &[E::Fr]
    ) -> E::Fr
    {
        let mut acc = E::Fr::zero();

        for (index, coeff) in self.0.into_iter() {
            let mut tmp;

            match index {
                Index::Input(i) => {
                    tmp = input_assignment[i];
                    if let Some(ref mut v) = input_density {
                        v.inc(i);
                    }
                },
                Index::Aux(i) => {
                    tmp = aux_assignment[i];
                    if let Some(ref mut v) = aux_density {
                        v.inc(i);
                    }
                }
            }

            if coeff == E::Fr::one() {
               acc.add_assign(&tmp);
            } else {
               tmp.mul_assign(&coeff);
               acc.add_assign(&tmp);
            }
        }

        acc
    }
}

impl<E: Engine> Add<Variable> for LinearCombination<E> {
    type Output = LinearCombination<E>;

    fn add(self, other: Variable) -> LinearCombination<E> {
        self + (E::Fr::one(), other)
    }
}

impl<E: Engine> Sub<Variable> for LinearCombination<E> {
    type Output = LinearCombination<E>;

    fn sub(self, other: Variable) -> LinearCombination<E> {
        self - (E::Fr::one(), other)
    }
}

impl<E: Engine> Add<(E::Fr, Variable)> for LinearCombination<E> {
    type Output = LinearCombination<E>;

    fn add(mut self, (coeff, var): (E::Fr, Variable)) -> LinearCombination<E> {
        let mut must_insert = true;

        for &mut (ref index, ref mut fr) in &mut self.0 {
            if *index == var.0 {
                fr.add_assign(&coeff);
                must_insert = false;
                break;
            }
        }

        if must_insert {
            self.0.push((var.0, coeff));
        }

        self
    }
}

impl<E: Engine> Sub<(E::Fr, Variable)> for LinearCombination<E> {
    type Output = LinearCombination<E>;

    fn sub(self, (mut coeff, var): (E::Fr, Variable)) -> LinearCombination<E> {
        coeff.negate();

        self + (coeff, var)
    }
}

impl<'a, E: Engine> Add<&'a LinearCombination<E>> for LinearCombination<E> {
    type Output = LinearCombination<E>;

    fn add(mut self, other: &'a LinearCombination<E>) -> LinearCombination<E> {
        for &(k, v) in other.0.iter() {
            self = self + (v, Variable(k));
        }

        self
    }
}

impl<'a, E: Engine> Sub<&'a LinearCombination<E>> for LinearCombination<E> {
    type Output = LinearCombination<E>;

    fn sub(mut self, other: &'a LinearCombination<E>) -> LinearCombination<E> {
        for &(k, v) in other.0.iter() {
            self = self - (v, Variable(k));
        }

        self
    }
}

pub trait Circuit<E: Engine> {
    type InputMap: Input<E>;

    /// Synthesize the circuit into a rank-1 quadratic constraint system
    #[must_use]
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<Self::InputMap, Error>;
}

pub trait Input<E: Engine> {
    /// Synthesize the circuit, except with additional access to public input
    /// variables
    fn synthesize<CS: PublicConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), Error>;
}

pub trait PublicConstraintSystem<E: Engine>: ConstraintSystem<E> {
    /// Allocate a public input that the verifier knows. The provided function is used to
    /// determine the assignment of the variable.
    fn alloc_input<F: FnOnce() -> Result<E::Fr, Error>>(&mut self, f: F) -> Result<Variable, Error>;
}

pub trait ConstraintSystem<E: Engine> {
    /// Return the "one" input variable
    fn one() -> Variable {
        Variable(Index::Input(0))
    }

    /// Allocate a private variable in the constraint system. The provided function is used to
    /// determine the assignment of the variable.
    fn alloc<F: FnOnce() -> Result<E::Fr, Error>>(&mut self, f: F) -> Result<Variable, Error>;

    /// Enforce that `A` * `B` = `C`.
    fn enforce(
        &mut self,
        a: LinearCombination<E>,
        b: LinearCombination<E>,
        c: LinearCombination<E>
    );
}
