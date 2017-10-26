// This library relies on the Rust nightly compiler's `i128_type` feature.
// If that's not okay for you, disable the u128-support feature. (Pass
// --no-default-features for example.)
#![cfg_attr(feature = "u128-support", feature(i128_type))]

// `clippy` is a code linting tool for improving code quality by catching
// common mistakes or strange code patterns. If the `clippy` feature is
// provided, it is enabled and all compiler warnings are prohibited.
#![cfg_attr(feature = "clippy", deny(warnings))]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", allow(inline_always))]
#![cfg_attr(feature = "clippy", allow(too_many_arguments))]
#![cfg_attr(feature = "clippy", allow(unreadable_literal))]

// The compiler provides `test` (on nightly) for benchmarking tools, but
// it's hidden behind a feature flag. Enable it if we're testing.
#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

extern crate rand;
extern crate byteorder;

#[cfg(test)]
pub mod tests;

pub mod bls12_381;

#[cfg(feature = "unstable-wnaf")]
pub mod wnaf;

use std::fmt;
use std::error::Error;
use std::io::{self, Read, Write};

/// An "engine" is a collection of types (fields, elliptic curve groups, etc.)
/// with well-defined relationships. In particular, the G1/G2 curve groups are
/// of prime order `r`, and are equipped with a bilinear pairing function.
pub trait Engine: Sized {
    /// This is the scalar field of the G1/G2 groups.
    type Fr: PrimeField;

    /// The projective representation of an element in G1.
    type G1: CurveProjective<Engine=Self, Base=Self::Fq, Scalar=Self::Fr, Affine=Self::G1Affine> + From<Self::G1Affine>;

    /// The affine representation of an element in G1.
    type G1Affine: CurveAffine<Engine=Self, Base=Self::Fq, Scalar=Self::Fr, Projective=Self::G1, Pair=Self::G2Affine, PairingResult=Self::Fqk> + From<Self::G1>;

    /// The projective representation of an element in G2.
    type G2: CurveProjective<Engine=Self, Base=Self::Fqe, Scalar=Self::Fr, Affine=Self::G2Affine> + From<Self::G2Affine>;

    /// The affine representation of an element in G2.
    type G2Affine: CurveAffine<Engine=Self, Base=Self::Fqe, Scalar=Self::Fr, Projective=Self::G2, Pair=Self::G1Affine, PairingResult=Self::Fqk> + From<Self::G2>;

    /// The base field that hosts G1.
    type Fq: PrimeField + SqrtField;

    /// The extension field that hosts G2.
    type Fqe: SqrtField;

    /// The extension field that hosts the target group of the pairing.
    type Fqk: Field;

    /// Perform a miller loop with some number of (G1, G2) pairs.
    fn miller_loop<'a, I>(i: I) -> Self::Fqk
        where I: IntoIterator<Item=&'a (
                                    &'a <Self::G1Affine as CurveAffine>::Prepared,
                                    &'a <Self::G2Affine as CurveAffine>::Prepared
                               )>;

    /// Perform final exponentiation of the result of a miller loop.
    fn final_exponentiation(&Self::Fqk) -> Option<Self::Fqk>;

    /// Performs a complete pairing operation `(p, q)`.
    fn pairing<G1, G2>(p: G1, q: G2) -> Self::Fqk
        where G1: Into<Self::G1Affine>,
              G2: Into<Self::G2Affine>
    {
        Self::final_exponentiation(&Self::miller_loop(
            [(
                &(p.into().prepare()),
                &(q.into().prepare())
            )].into_iter()
        )).unwrap()
    }
}

/// Projective representation of an elliptic curve point guaranteed to be
/// in the correct prime order subgroup.
pub trait CurveProjective: PartialEq +
                           Eq +
                           Sized +
                           Copy +
                           Clone +
                           Send +
                           Sync +
                           fmt::Debug +
                           fmt::Display +
                           rand::Rand +
                           'static
{
    type Engine: Engine;
    type Scalar: PrimeField;
    type Base: SqrtField;
    type Affine: CurveAffine<Projective=Self, Scalar=Self::Scalar>;

    /// Returns the additive identity.
    fn zero() -> Self;

    /// Returns a fixed generator of unknown exponent.
    fn one() -> Self;

    /// Determines if this point is the point at infinity.
    fn is_zero(&self) -> bool;

    /// Normalizes a slice of projective elements so that
    /// conversion to affine is cheap.
    fn batch_normalization(v: &mut [Self]);

    /// Checks if the point is already "normalized" so that
    /// cheap affine conversion is possible.
    fn is_normalized(&self) -> bool;

    /// Doubles this element.
    fn double(&mut self);

    /// Adds another element to this element.
    fn add_assign(&mut self, other: &Self);

    /// Subtracts another element from this element.
    fn sub_assign(&mut self, other: &Self) {
        let mut tmp = *other;
        tmp.negate();
        self.add_assign(&tmp);
    }

    /// Adds an affine element to this element.
    fn add_assign_mixed(&mut self, other: &Self::Affine);

    /// Negates this element.
    fn negate(&mut self);

    /// Performs scalar multiplication of this element.
    fn mul_assign<S: Into<<Self::Scalar as PrimeField>::Repr>>(&mut self, other: S);

    /// Converts this element into its affine representation.
    fn into_affine(&self) -> Self::Affine;

    /// Recommends a wNAF window table size given a scalar. Returns `None` if normal
    /// scalar multiplication is encouraged. If `Some` is returned, it will be between
    /// 2 and 22, inclusive.
    fn recommended_wnaf_for_scalar(scalar: <Self::Scalar as PrimeField>::Repr) -> Option<usize>;

    /// Recommends a wNAF window size given the number of scalars you intend to multiply
    /// a base by. Always returns a number between 2 and 22, inclusive.
    fn recommended_wnaf_for_num_scalars(num_scalars: usize) -> usize;
}

/// Affine representation of an elliptic curve point guaranteed to be
/// in the correct prime order subgroup.
pub trait CurveAffine: Copy +
                       Clone +
                       Sized +
                       Send +
                       Sync +
                       fmt::Debug +
                       fmt::Display +
                       PartialEq +
                       Eq +
                       'static
{
    type Engine: Engine;
    type Scalar: PrimeField;
    type Base: SqrtField;
    type Projective: CurveProjective<Affine=Self, Scalar=Self::Scalar>;
    type Prepared: Clone + Send + Sync + 'static;
    type Uncompressed: EncodedPoint<Affine=Self>;
    type Compressed: EncodedPoint<Affine=Self>;
    type Pair: CurveAffine<Pair=Self>;
    type PairingResult: Field;
    type FieldSerial;

    fn serial(&self)->Self::FieldSerial;

    fn from_serial(Self::FieldSerial)->Self;

    /// Returns the additive identity.
    fn zero() -> Self;

    /// Returns a fixed generator of unknown exponent.
    fn one() -> Self;

    /// Determines if this point represents the point at infinity; the
    /// additive identity.
    fn is_zero(&self) -> bool;

    /// Negates this element.
    fn negate(&mut self);

    /// Performs scalar multiplication of this element with mixed addition.
    fn mul<S: Into<<Self::Scalar as PrimeField>::Repr>>(&self, other: S) -> Self::Projective;

    /// Prepares this element for pairing purposes.
    fn prepare(&self) -> Self::Prepared;

    /// Perform a pairing
    fn pairing_with(&self, other: &Self::Pair) -> Self::PairingResult;

    /// Converts this element into its affine representation.
    fn into_projective(&self) -> Self::Projective;

    /// Converts this element into its compressed encoding, so long as it's not
    /// the point at infinity.
    fn into_compressed(&self) -> Self::Compressed {
        <Self::Compressed as EncodedPoint>::from_affine(*self)
    }

    /// Converts this element into its uncompressed encoding, so long as it's not
    /// the point at infinity.
    fn into_uncompressed(&self) -> Self::Uncompressed {
        <Self::Uncompressed as EncodedPoint>::from_affine(*self)
    }
}

/// An encoded elliptic curve point, which should essentially wrap a `[u8; N]`.
pub trait EncodedPoint: Sized +
                        Send +
                        Sync +
                        AsRef<[u8]> +
                        AsMut<[u8]> +
                        Clone +
                        Copy +
                        'static
{
    type Affine: CurveAffine;

    /// Creates an empty representation.
    fn empty() -> Self;

    /// Returns the number of bytes consumed by this representation.
    fn size() -> usize;

    /// Converts an `EncodedPoint` into a `CurveAffine` element,
    /// if the encoding represents a valid element.
    fn into_affine(&self) -> Result<Self::Affine, GroupDecodingError>;

    /// Converts an `EncodedPoint` into a `CurveAffine` element,
    /// without guaranteeing that the encoding represents a valid
    /// element. This is useful when the caller knows the encoding is
    /// valid already.
    ///
    /// If the encoding is invalid, this can break API invariants,
    /// so caution is strongly encouraged.
    fn into_affine_unchecked(&self) -> Result<Self::Affine, GroupDecodingError>;

    /// Creates an `EncodedPoint` from an affine point, as long as the
    /// point is not the point at infinity.
    fn from_affine(affine: Self::Affine) -> Self;
}

/// This trait represents an element of a field.
pub trait Field: Sized +
                 Eq +
                 Copy +
                 Clone +
                 Send +
                 Sync +
                 fmt::Debug +
                 fmt::Display +
                 'static +
                 rand::Rand
{
    /// Returns the zero element of the field, the additive identity.
    fn zero() -> Self;

    /// Returns the one element of the field, the multiplicative identity.
    fn one() -> Self;

    /// Returns true iff this element is zero.
    fn is_zero(&self) -> bool;

    /// Squares this element.
    fn square(&mut self);

    /// Doubles this element.
    fn double(&mut self);

    /// Negates this element.
    fn negate(&mut self);

    /// Adds another element to this element.
    fn add_assign(&mut self, other: &Self);

    /// Subtracts another element from this element.
    fn sub_assign(&mut self, other: &Self);

    /// Multiplies another element by this element.
    fn mul_assign(&mut self, other: &Self);

    /// Computes the multiplicative inverse of this element, if nonzero.
    fn inverse(&self) -> Option<Self>;

    /// Exponentiates this element by a power of the base prime modulus via
    /// the Frobenius automorphism.
    fn frobenius_map(&mut self, power: usize);

    /// Exponentiates this element by a number represented with `u64` limbs,
    /// least significant digit first.
    fn pow<S: AsRef<[u64]>>(&self, exp: S) -> Self
    {
        let mut res = Self::one();

        let mut found_one = false;

        for i in BitIterator::new(exp) {
            if found_one {
                res.square();
            } else {
                found_one = i;
            }

            if i {
                res.mul_assign(self);
            }
        }

        res
    }
}

/// This trait represents an element of a field that has a square root operation described for it.
pub trait SqrtField: Field
{
    /// Returns the square root of the field element, if it is
    /// quadratic residue.
    fn sqrt(&self) -> Option<Self>;
}

/// This trait represents a wrapper around a biginteger which can encode any element of a particular
/// prime field. It is a smart wrapper around a sequence of `u64` limbs, least-significant digit
/// first.
pub trait PrimeFieldRepr: Sized +
                          Copy +
                          Clone +
                          Eq +
                          Ord +
                          Send +
                          Sync +
                          Default +
                          fmt::Debug +
                          fmt::Display +
                          'static +
                          rand::Rand +
                          AsRef<[u64]> +
                          AsMut<[u64]> +
                          From<u64>
{
    /// Subtract another represetation from this one, returning the borrow bit.
    fn sub_noborrow(&mut self, other: &Self) -> bool;

    /// Add another representation to this one, returning the carry bit.
    fn add_nocarry(&mut self, other: &Self) -> bool;

    /// Compute the number of bits needed to encode this number. Always a
    /// multiple of 64.
    fn num_bits(&self) -> u32;

    /// Returns true iff this number is zero.
    fn is_zero(&self) -> bool;

    /// Returns true iff this number is odd.
    fn is_odd(&self) -> bool;

    /// Returns true iff this number is even.
    fn is_even(&self) -> bool;

    /// Performs a rightwise bitshift of this number, effectively dividing
    /// it by 2.
    fn div2(&mut self);

    /// Performs a rightwise bitshift of this number by some amount.
    fn divn(&mut self, amt: u32);

    /// Performs a leftwise bitshift of this number, effectively multiplying
    /// it by 2. Overflow is ignored.
    fn mul2(&mut self);

    /// Performs a leftwise bitshift of this number by some amount.
    fn muln(&mut self, amt: u32);

    /// Writes this `PrimeFieldRepr` as a big endian integer. Always writes
    /// `(num_bits` / 8) bytes.
    fn write_be<W: Write>(&self, mut writer: W) -> io::Result<()> {
        use byteorder::{WriteBytesExt, BigEndian};

        for digit in self.as_ref().iter().rev() {
            writer.write_u64::<BigEndian>(*digit)?;
        }

        Ok(())
    }

    /// Reads a big endian integer occupying (`num_bits` / 8) bytes into this
    /// representation.
    fn read_be<R: Read>(&mut self, mut reader: R) -> io::Result<()> {
        use byteorder::{ReadBytesExt, BigEndian};

        for digit in self.as_mut().iter_mut().rev() {
            *digit = reader.read_u64::<BigEndian>()?;
        }

        Ok(())
    }
}

/// An error that may occur when trying to interpret a `PrimeFieldRepr` as a
/// `PrimeField` element.
#[derive(Debug)]
pub enum PrimeFieldDecodingError {
    /// The encoded value is not in the field
    NotInField(String)
}

impl Error for PrimeFieldDecodingError {
    fn description(&self) -> &str {
        match *self {
            PrimeFieldDecodingError::NotInField(..) => "not an element of the field"
        }
    }
}

impl fmt::Display for PrimeFieldDecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            PrimeFieldDecodingError::NotInField(ref repr) => {
                write!(f, "{} is not an element of the field", repr)
            }
        }
    }
}

/// An error that may occur when trying to decode an `EncodedPoint`.
#[derive(Debug)]
pub enum GroupDecodingError {
    /// The coordinate(s) do not lie on the curve.
    NotOnCurve,
    /// The element is not part of the r-order subgroup.
    NotInSubgroup,
    /// One of the coordinates could not be decoded
    CoordinateDecodingError(&'static str, PrimeFieldDecodingError),
    /// The compression mode of the encoded element was not as expected
    UnexpectedCompressionMode,
    /// The encoding contained bits that should not have been set
    UnexpectedInformation
}

impl Error for GroupDecodingError {
    fn description(&self) -> &str {
        match *self {
            GroupDecodingError::NotOnCurve => "coordinate(s) do not lie on the curve",
            GroupDecodingError::NotInSubgroup => "the element is not part of an r-order subgroup",
            GroupDecodingError::CoordinateDecodingError(..) => "coordinate(s) could not be decoded",
            GroupDecodingError::UnexpectedCompressionMode => "encoding has unexpected compression mode",
            GroupDecodingError::UnexpectedInformation => "encoding has unexpected information"
        }
    }
}

impl fmt::Display for GroupDecodingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            GroupDecodingError::CoordinateDecodingError(description, ref err) => {
                write!(f, "{} decoding error: {}", description, err)
            },
            _ => {
                write!(f, "{}", self.description())
            }
        }
    }
}

/// This represents an element of a prime field.
pub trait PrimeField: Field
{
    /// The prime field can be converted back and forth into this biginteger
    /// representation.
    type Repr: PrimeFieldRepr + From<Self>;

    /// Interpret a string of numbers as a (congruent) prime field element.
    /// Does not accept unnecessary leading zeroes or a blank string.
    fn from_str(s: &str) -> Option<Self> {
        if s.is_empty() {
            return None;
        }

        if s == "0" {
            return Some(Self::zero());
        }

        let mut res = Self::zero();

        let ten = Self::from_repr(Self::Repr::from(10)).unwrap();

        let mut first_digit = true;

        for c in s.chars() {
            match c.to_digit(10) {
                Some(c) => {
                    if first_digit {
                        if c == 0 {
                            return None;
                        }

                        first_digit = false;
                    }

                    res.mul_assign(&ten);
                    res.add_assign(&Self::from_repr(Self::Repr::from(c as u64)).unwrap());
                },
                None => {
                    return None;
                }
            }
        }

        Some(res)
    }

    /// Convert this prime field element into a biginteger representation.
    fn from_repr(Self::Repr) -> Result<Self, PrimeFieldDecodingError>;

    /// Convert a biginteger representation into a prime field element, if
    /// the number is an element of the field.
    fn into_repr(&self) -> Self::Repr;

    /// Returns the field characteristic; the modulus.
    fn char() -> Self::Repr;

    /// Returns how many bits are needed to represent an element of this
    /// field.
    fn num_bits() -> u32;

    /// Returns how many bits of information can be reliably stored in the
    /// field element.
    fn capacity() -> u32;

    /// Returns the multiplicative generator of `char()` - 1 order. This element
    /// must also be quadratic nonresidue.
    fn multiplicative_generator() -> Self;

    /// Returns s such that 2^s * t = `char()` - 1 with t odd.
    fn s() -> u32;

    /// Returns the 2^s root of unity computed by exponentiating the `multiplicative_generator()`
    /// by t.
    fn root_of_unity() -> Self;
}

pub struct BitIterator<E> {
    t: E,
    n: usize
}

impl<E: AsRef<[u64]>> BitIterator<E> {
    pub fn new(t: E) -> Self {
        let n = t.as_ref().len() * 64;

        BitIterator {
            t: t,
            n: n
        }
    }
}

impl<E: AsRef<[u64]>> Iterator for BitIterator<E> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.n == 0 {
            None
        } else {
            self.n -= 1;
            let part = self.n / 64;
            let bit = self.n - (64 * part);

            Some(self.t.as_ref()[part] & (1 << bit) > 0)
        }
    }
}

#[test]
fn test_bit_iterator() {
    let mut a = BitIterator::new([0xa953d79b83f6ab59, 0x6dea2059e200bd39]);
    let expected = "01101101111010100010000001011001111000100000000010111101001110011010100101010011110101111001101110000011111101101010101101011001";

    for e in expected.chars() {
        assert!(a.next().unwrap() == (e == '1'));
    }

    assert!(a.next().is_none());

    let expected = "1010010101111110101010000101101011101000011101110101001000011001100100100011011010001011011011010001011011101100110100111011010010110001000011110100110001100110011101101000101100011100100100100100001010011101010111110011101011000011101000111011011101011001";

    let mut a = BitIterator::new([0x429d5f3ac3a3b759, 0xb10f4c66768b1c92, 0x92368b6d16ecd3b4, 0xa57ea85ae8775219]);

    for e in expected.chars() {
        assert!(a.next().unwrap() == (e == '1'));
    }

    assert!(a.next().is_none());
}

use self::arith::*;

#[cfg(feature = "u128-support")]
mod arith {

    /// Calculate a - b - borrow, returning the result and modifying
    /// the borrow value.
    #[inline(always)]
    pub(crate) fn sbb(a: u64, b: u64, borrow: &mut u64) -> u64 {
        let tmp = (1u128 << 64) + (a as u128) - (b as u128) - (*borrow as u128);

        *borrow = if tmp >> 64 == 0 { 1 } else { 0 };

        tmp as u64
    }

    /// Calculate a + b + carry, returning the sum and modifying the
    /// carry value.
    #[inline(always)]
    pub(crate) fn adc(a: u64, b: u64, carry: &mut u64) -> u64 {
        let tmp = (a as u128) + (b as u128) + (*carry as u128);

        *carry = (tmp >> 64) as u64;

        tmp as u64
    }

    /// Calculate a + (b * c) + carry, returning the least significant digit
    /// and setting carry to the most significant digit.
    #[inline(always)]
    pub(crate) fn mac_with_carry(a: u64, b: u64, c: u64, carry: &mut u64) -> u64 {
        let tmp = (a as u128) + (b as u128) * (c as u128) + (*carry as u128);

        *carry = (tmp >> 64) as u64;

        tmp as u64
    }
}

#[cfg(not(feature = "u128-support"))]
mod arith {
    #[inline(always)]
    fn split_u64(i: u64) -> (u64, u64) {
        (i >> 32, i & 0xFFFFFFFF)
    }

    #[inline(always)]
    fn combine_u64(hi: u64, lo: u64) -> u64 {
        (hi << 32) | lo
    }

    #[inline(always)]
    pub(crate) fn sbb(a: u64, b: u64, borrow: &mut u64) -> u64 {
        let (a_hi, a_lo) = split_u64(a);
        let (b_hi, b_lo) = split_u64(b);
        let (b, r0) = split_u64((1 << 32) + a_lo - b_lo - *borrow);
        let (b, r1) = split_u64((1 << 32) + a_hi - b_hi - ((b == 0) as u64));

        *borrow = (b == 0) as u64;

        combine_u64(r1, r0)
    }

    #[inline(always)]
    pub(crate) fn adc(a: u64, b: u64, carry: &mut u64) -> u64 {
        let (a_hi, a_lo) = split_u64(a);
        let (b_hi, b_lo) = split_u64(b);
        let (carry_hi, carry_lo) = split_u64(*carry);

        let (t, r0) = split_u64(a_lo + b_lo + carry_lo);
        let (t, r1) = split_u64(t + a_hi + b_hi + carry_hi);

        *carry = t;

        combine_u64(r1, r0)
    }

    #[inline(always)]
    pub(crate) fn mac_with_carry(a: u64, b: u64, c: u64, carry: &mut u64) -> u64 {
        /*
                                [  b_hi  |  b_lo  ]
                                [  c_hi  |  c_lo  ] *
        -------------------------------------------
                                [  b_lo  *  c_lo  ] <-- w
                       [  b_hi  *  c_lo  ]          <-- x
                       [  b_lo  *  c_hi  ]          <-- y
             [   b_hi  *  c_lo  ]                   <-- z
                                [  a_hi  |  a_lo  ]
                                [  C_hi  |  C_lo  ]
        */

        let (a_hi, a_lo) = split_u64(a);
        let (b_hi, b_lo) = split_u64(b);
        let (c_hi, c_lo) = split_u64(c);
        let (carry_hi, carry_lo) = split_u64(*carry);

        let (w_hi, w_lo) = split_u64(b_lo * c_lo);
        let (x_hi, x_lo) = split_u64(b_hi * c_lo);
        let (y_hi, y_lo) = split_u64(b_lo * c_hi);
        let (z_hi, z_lo) = split_u64(b_hi * c_hi);

        let (t, r0) = split_u64(w_lo + a_lo + carry_lo);
        let (t, r1) = split_u64(t + w_hi + x_lo + y_lo + a_hi + carry_hi);
        let (t, r2) = split_u64(t + x_hi + y_hi + z_lo);
        let (_, r3) = split_u64(t + z_hi);

        *carry = combine_u64(r3, r2);

        combine_u64(r1, r0)
    }
}
