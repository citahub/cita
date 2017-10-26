use pairing::*;
use std::sync::Arc;

mod generator;
pub use self::generator::*;
mod prover;
pub use self::prover::*;
mod verifier;
pub use self::verifier::*;

use ::Error;
use std::io::{self, Write, Read};
use multiexp::{Source, SourceBuilder};

#[derive(Debug)]
pub struct Proof<E: Engine> {
    pub a: E::G1Affine,
    pub b: E::G2Affine,
    pub c: E::G1Affine
}

impl<E:Engine> Proof<E>{
    pub fn serial(&self)->(<E::G1Affine as CurveAffine>::FieldSerial,<E::G2Affine as CurveAffine>::FieldSerial,<E::G1Affine as CurveAffine>::FieldSerial){
        (self.a.serial(),self.b.serial(),self.c.serial())
    }

    pub fn from_serial(serial:(<E::G1Affine as CurveAffine>::FieldSerial,<E::G2Affine as CurveAffine>::FieldSerial,<E::G1Affine as CurveAffine>::FieldSerial))->Self{
        Proof{
            a:<E::G1Affine as CurveAffine>::from_serial(serial.0),
            b:<E::G2Affine as CurveAffine>::from_serial(serial.1),
            c:<E::G1Affine as CurveAffine>::from_serial(serial.2)
        }
    }
}

pub struct PreparedVerifyingKey<E: Engine> {
    alpha_g1_beta_g2: E::Fqk,
    neg_gamma_g2: <E::G2Affine as CurveAffine>::Prepared,
    neg_delta_g2: <E::G2Affine as CurveAffine>::Prepared,
    ic: Vec<E::G1Affine>
}

pub struct VerifyingKey<E: Engine> {
    // alpha in g1 for verifying and for creating A/C elements of
    // proof. Never the point at infinity.
    alpha_g1: E::G1Affine,

    // beta in g1 and g2 for verifying and for creating B/C elements
    // of proof. Never the point at infinity.
    beta_g1: E::G1Affine,
    beta_g2: E::G2Affine,

    // gamma in g2 for verifying. Never the point at infinity.
    gamma_g2: E::G2Affine,

    // delta in g1/g2 for verifying and proving, essentially the magic
    // trapdoor that forces the prover to evaluate the C element of the
    // proof with only components from the CRS. Never the point at
    // infinity.
    delta_g1: E::G1Affine,
    delta_g2: E::G2Affine,

    // Elements of the form (beta * u_i(tau) + alpha v_i(tau) + w_i(tau)) / gamma
    // for all public inputs. Because all public inputs have a "soundness
    // of input consistency" constraint, this is the same size as the
    // number of inputs, and never contains points at infinity.
    ic: Vec<E::G1Affine>
}

impl<E: Engine> Clone for VerifyingKey<E> {
    fn clone(&self) -> VerifyingKey<E> {
        VerifyingKey {
            alpha_g1: self.alpha_g1.clone(),
            beta_g1: self.beta_g1.clone(),
            beta_g2: self.beta_g2.clone(),
            gamma_g2: self.gamma_g2.clone(),
            delta_g1: self.delta_g1.clone(),
            delta_g2: self.delta_g2.clone(),
            ic: self.ic.clone()
        }
    } 
}

impl<E: Engine> PartialEq for VerifyingKey<E> {
    fn eq(&self, other: &VerifyingKey<E>) -> bool {
        self.alpha_g1 == other.alpha_g1 &&
        self.beta_g1 == other.beta_g1 &&
        self.beta_g2 == other.beta_g2 &&
        self.gamma_g2 == other.gamma_g2 &&
        self.delta_g1 == other.delta_g1 &&
        self.delta_g2 == other.delta_g2 &&
        self.ic == other.ic
    }
}

fn read_nonzero<R: Read, G: CurveAffine>(reader: &mut R) -> Result<G, Error> {
    let mut repr = G::Uncompressed::empty();
    reader.read_exact(repr.as_mut())?;

    let affine = repr.into_affine_unchecked(); // TODO

    match affine {
        Ok(affine) => {
            if affine.is_zero() {
                Err(Error::UnexpectedIdentity)
            } else {
                Ok(affine)
            }
        },
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e).into())
    }
}

impl<E: Engine> VerifyingKey<E> {
    fn size(num_ic: usize) -> usize {
        let mut acc = 0;
        acc += <E::G1Affine as CurveAffine>::Uncompressed::size(); // alpha_g1
        acc += <E::G1Affine as CurveAffine>::Uncompressed::size(); // beta_g1
        acc += <E::G1Affine as CurveAffine>::Uncompressed::size(); // delta_g1
        acc += <E::G1Affine as CurveAffine>::Uncompressed::size() * num_ic; // IC
        acc += <E::G2Affine as CurveAffine>::Uncompressed::size(); // beta_g2
        acc += <E::G2Affine as CurveAffine>::Uncompressed::size(); // gamma_g2
        acc += <E::G2Affine as CurveAffine>::Uncompressed::size(); // delta_g2

        acc
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        writer.write_all(self.alpha_g1.into_uncompressed().as_ref())?;
        writer.write_all(self.beta_g1.into_uncompressed().as_ref())?;
        writer.write_all(self.beta_g2.into_uncompressed().as_ref())?;
        writer.write_all(self.gamma_g2.into_uncompressed().as_ref())?;
        writer.write_all(self.delta_g1.into_uncompressed().as_ref())?;
        writer.write_all(self.delta_g2.into_uncompressed().as_ref())?;
        for ic in &self.ic {
            writer.write_all(ic.into_uncompressed().as_ref())?;
        }

        Ok(())
    }

    pub fn read<R: Read>(reader: &mut R, num_ic: usize) -> Result<VerifyingKey<E>, Error> {
        let alpha_g1 = read_nonzero(reader)?;
        let beta_g1 = read_nonzero(reader)?;
        let beta_g2 = read_nonzero(reader)?;
        let gamma_g2 = read_nonzero(reader)?;
        let delta_g1 = read_nonzero(reader)?;
        let delta_g2 = read_nonzero(reader)?;

        let mut ic = vec![];
        for _ in 0..num_ic {
            ic.push(read_nonzero(reader)?);
        }

        Ok(VerifyingKey {
            alpha_g1: alpha_g1,
            beta_g1: beta_g1,
            beta_g2: beta_g2,
            gamma_g2: gamma_g2,
            delta_g1: delta_g1,
            delta_g2: delta_g2,
            ic: ic
        })
    }
}

pub struct Parameters<E: Engine> {
    pub vk: VerifyingKey<E>,

    // Elements of the form ((tau^i * t(tau)) / delta) for i between 0 and 
    // m-2 inclusive. Never contains points at infinity.
    h: Arc<Vec<E::G1Affine>>,

    // Elements of the form (beta * u_i(tau) + alpha v_i(tau) + w_i(tau)) / delta
    // for all auxillary inputs. Variables can never be unconstrained, so this
    // never contains points at infinity.
    l: Arc<Vec<E::G1Affine>>,

    // QAP "A" polynomials evaluated at tau in the Lagrange basis. Never contains
    // points at infinity: polynomials that evaluate to zero are omitted from
    // the CRS and the prover can deterministically skip their evaluation.
    a: Arc<Vec<E::G1Affine>>,

    // QAP "B" polynomials evaluated at tau in the Lagrange basis. Needed in
    // G1 and G2 for C/B queries, respectively. Never contains points at
    // infinity for the same reason as the "A" polynomials.
    b_g1: Arc<Vec<E::G1Affine>>,
    b_g2: Arc<Vec<E::G2Affine>>
}

impl<E: Engine> Parameters<E> {
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        self.vk.write(writer)?;

        for e in &*self.h {
            writer.write_all(e.into_uncompressed().as_ref())?;
        }

        for e in &*self.l {
            writer.write_all(e.into_uncompressed().as_ref())?;
        }

        for e in &*self.a {
            writer.write_all(e.into_uncompressed().as_ref())?;
        }

        for e in &*self.b_g1 {
            writer.write_all(e.into_uncompressed().as_ref())?;
        }

        for e in &*self.b_g2 {
            writer.write_all(e.into_uncompressed().as_ref())?;
        }

        Ok(())
    }
}

pub trait ParameterSource<E: Engine> {
    type G1Builder: SourceBuilder<E::G1Affine>;
    type G2Builder: SourceBuilder<E::G2Affine>;

    fn get_vk(&mut self, num_ic: usize) -> Result<VerifyingKey<E>, Error>;
    fn get_h(&mut self, num_h: usize) -> Result<Self::G1Builder, Error>;
    fn get_l(&mut self, num_l: usize) -> Result<Self::G1Builder, Error>;
    fn get_a(&mut self, num_inputs: usize, num_aux: usize) -> Result<(Self::G1Builder, Self::G1Builder), Error>;
    fn get_b_g1(&mut self, num_inputs: usize, num_aux: usize) -> Result<(Self::G1Builder, Self::G1Builder), Error>;
    fn get_b_g2(&mut self, num_inputs: usize, num_aux: usize) -> Result<(Self::G2Builder, Self::G2Builder), Error>;
}

impl<'a, E: Engine> ParameterSource<E> for &'a Parameters<E> {
    type G1Builder = (Arc<Vec<E::G1Affine>>, usize);
    type G2Builder = (Arc<Vec<E::G2Affine>>, usize);

    fn get_vk(&mut self, num_ic: usize) -> Result<VerifyingKey<E>, Error> {
        assert_eq!(self.vk.ic.len(), num_ic);

        Ok(self.vk.clone())
    }

    fn get_h(&mut self, num_h: usize) -> Result<Self::G1Builder, Error> {
        assert_eq!(self.h.len(), num_h);

        Ok((self.h.clone(), 0))
    }

    fn get_l(&mut self, num_l: usize) -> Result<Self::G1Builder, Error> {
        assert_eq!(self.l.len(), num_l);

        Ok((self.l.clone(), 0))
    }

    fn get_a(&mut self, num_inputs: usize, num_aux: usize) -> Result<(Self::G1Builder, Self::G1Builder), Error> {
        assert_eq!(self.a.len(), num_inputs + num_aux);

        Ok(((self.a.clone(), 0), (self.a.clone(), num_inputs)))
    }

    fn get_b_g1(&mut self, num_inputs: usize, num_aux: usize) -> Result<(Self::G1Builder, Self::G1Builder), Error> {
        assert_eq!(self.b_g1.len(), num_inputs + num_aux);

        Ok(((self.b_g1.clone(), 0), (self.b_g1.clone(), num_inputs)))
    }

    fn get_b_g2(&mut self, num_inputs: usize, num_aux: usize) -> Result<(Self::G2Builder, Self::G2Builder), Error> {
        assert_eq!(self.b_g2.len(), num_inputs + num_aux);

        Ok(((self.b_g2.clone(), 0), (self.b_g2.clone(), num_inputs)))
    }
}

use std::fs::File;
use std::io::{Seek, SeekFrom};

pub struct ProverStream {
    path: String,
    cursor: u64,
    fh: Option<File>
}

impl Clone for ProverStream {
    fn clone(&self) -> ProverStream {
        ProverStream {
            path: self.path.clone(),
            cursor: self.cursor,
            fh: None
        }
    }
}

impl ProverStream {
    pub fn new(path: &str) -> Result<ProverStream, io::Error> {
        Ok(ProverStream {
            path: path.to_string(),
            cursor: 0,
            fh: None
        })
    }

    fn open_if_needed(&mut self) -> Result<(), Error> {
        if self.fh.is_none() {
            let mut fh = File::open(&self.path)?;
            fh.seek(SeekFrom::Start(self.cursor))?;

            self.fh = Some(fh);
        }

        Ok(())
    }
}

impl<G: CurveAffine> Source<G> for ProverStream {
    fn add_assign_mixed(&mut self, to: &mut <G as CurveAffine>::Projective) -> Result<(), Error> {
        self.open_if_needed()?;

        let r: G = read_nonzero(self.fh.as_mut().unwrap())?;

        self.cursor += G::Uncompressed::size() as u64;
        
        to.add_assign_mixed(&r);

        Ok(())
    }
    fn skip(&mut self, amt: usize) -> Result<(), Error> {
        self.open_if_needed()?;
        
        let size_to_skip = amt * G::Uncompressed::size();

        self.cursor += size_to_skip as u64;

        self.fh.as_mut().unwrap().seek(SeekFrom::Current(size_to_skip as i64))?;

        Ok(())
    }
}

impl<G: CurveAffine> SourceBuilder<G> for ProverStream {
    type Source = Self;

    fn new(self) -> Self::Source {
        self
    }
}

impl<E: Engine> ParameterSource<E> for ProverStream {
    type G1Builder = ProverStream;
    type G2Builder = ProverStream;

    fn get_vk(&mut self, num_ic: usize) -> Result<VerifyingKey<E>, Error> {
        self.open_if_needed()?;

        let vk = VerifyingKey::read(self.fh.as_mut().unwrap(), num_ic)?;

        self.cursor += VerifyingKey::<E>::size(num_ic) as u64;

        Ok(vk)
    }
    fn get_h(&mut self, num_h: usize) -> Result<Self::G1Builder, Error> {
        self.open_if_needed()?;

        let res = self.clone();

        let amount_to_seek = num_h * <E::G1Affine as CurveAffine>::Uncompressed::size();

        self.fh.as_mut().unwrap().seek(SeekFrom::Current(amount_to_seek as i64))?;
        self.cursor += amount_to_seek as u64;

        Ok(res)
    }
    fn get_l(&mut self, num_l: usize) -> Result<Self::G1Builder, Error> {
        self.open_if_needed()?;

        let res = self.clone();

        let amount_to_seek = num_l * <E::G1Affine as CurveAffine>::Uncompressed::size();

        self.fh.as_mut().unwrap().seek(SeekFrom::Current(amount_to_seek as i64))?;
        self.cursor += amount_to_seek as u64;

        Ok(res)
    }
    fn get_a(&mut self, num_inputs: usize, num_aux: usize) -> Result<(Self::G1Builder, Self::G1Builder), Error> {
        self.open_if_needed()?;

        let res1 = self.clone();

        let amount_to_seek = num_inputs * <E::G1Affine as CurveAffine>::Uncompressed::size();

        self.fh.as_mut().unwrap().seek(SeekFrom::Current(amount_to_seek as i64))?;
        self.cursor += amount_to_seek as u64;

        let res2 = self.clone();

        let amount_to_seek = num_aux * <E::G1Affine as CurveAffine>::Uncompressed::size();

        self.fh.as_mut().unwrap().seek(SeekFrom::Current(amount_to_seek as i64))?;
        self.cursor += amount_to_seek as u64;

        Ok((res1, res2))
    }
    fn get_b_g1(&mut self, num_inputs: usize, num_aux: usize) -> Result<(Self::G1Builder, Self::G1Builder), Error> {
        self.open_if_needed()?;

        let res1 = self.clone();

        let amount_to_seek = num_inputs * <E::G1Affine as CurveAffine>::Uncompressed::size();

        self.fh.as_mut().unwrap().seek(SeekFrom::Current(amount_to_seek as i64))?;
        self.cursor += amount_to_seek as u64;

        let res2 = self.clone();

        let amount_to_seek = num_aux * <E::G1Affine as CurveAffine>::Uncompressed::size();

        self.fh.as_mut().unwrap().seek(SeekFrom::Current(amount_to_seek as i64))?;
        self.cursor += amount_to_seek as u64;

        Ok((res1, res2))
    }
    fn get_b_g2(&mut self, num_inputs: usize, num_aux: usize) -> Result<(Self::G2Builder, Self::G2Builder), Error> {
        self.open_if_needed()?;

        let res1 = self.clone();

        let amount_to_seek = num_inputs * <E::G2Affine as CurveAffine>::Uncompressed::size();

        self.fh.as_mut().unwrap().seek(SeekFrom::Current(amount_to_seek as i64))?;
        self.cursor += amount_to_seek as u64;

        let res2 = self.clone();

        let amount_to_seek = num_aux * <E::G2Affine as CurveAffine>::Uncompressed::size();

        self.fh.as_mut().unwrap().seek(SeekFrom::Current(amount_to_seek as i64))?;
        self.cursor += amount_to_seek as u64;

        Ok((res1, res2))
    }
}
