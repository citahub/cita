use ::{Field, PrimeField, SqrtField, PrimeFieldRepr, PrimeFieldDecodingError};

// r = 52435875175126190479447740508185965837690552500527637822603658699938581184513
const MODULUS: FrRepr = FrRepr([0xffffffff00000001, 0x53bda402fffe5bfe, 0x3339d80809a1d805, 0x73eda753299d7d48]);

// The number of bits needed to represent the modulus.
const MODULUS_BITS: u32 = 255;

// The number of bits that must be shaved from the beginning of
// the representation when randomly sampling.
const REPR_SHAVE_BITS: u32 = 1;

// R = 2**256 % r
const R: FrRepr = FrRepr([0x1fffffffe, 0x5884b7fa00034802, 0x998c4fefecbc4ff5, 0x1824b159acc5056f]);

// R2 = R^2 % r
const R2: FrRepr = FrRepr([0xc999e990f3f29c6d, 0x2b6cedcb87925c23, 0x5d314967254398f, 0x748d9d99f59ff11]);

// INV = -(r^{-1} mod r) mod r
const INV: u64 = 0xfffffffeffffffff;

// GENERATOR = 7 (multiplicative generator of r-1 order, that is also quadratic nonresidue)
const GENERATOR: FrRepr = FrRepr([0xefffffff1, 0x17e363d300189c0f, 0xff9c57876f8457b0, 0x351332208fc5a8c4]);

// 2^s * t = MODULUS - 1 with t odd
const S: u32 = 32;

// 2^s root of unity computed by GENERATOR^t
const ROOT_OF_UNITY: FrRepr = FrRepr([0xb9b58d8c5f0e466a, 0x5b1b4c801819d7ec, 0xaf53ae352a31e64, 0x5bf3adda19e9b27b]);

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct FrRepr(pub [u64; 4]);

impl FrRepr{
    pub fn serial(&self)->[u64;4]{
        self.0
    }

    pub fn from_serial(serial:[u64;4])->FrRepr{
        FrRepr(serial)
    }

    pub fn bits(&self)->Vec<bool>{
        let mut v = Vec::with_capacity(256);
        for num in self.0.into_iter(){
            let mut num = *num;
            for _ in 0..64{
                v.push(num&1==1);
                num>>=1;
            }
        }
        v
    }

    pub fn from_bits(bits:Vec<bool>)->Self{
        assert_eq!(bits.len(),256);
        let mut j = 0;
        let mut v:[u64;4] = [0;4];
        for i in 0..4{
            for _ in 0..64{
                v[i]<<=1;
                v[i]|=bits[j] as u64;
                j+=1;
            }
        }
        FrRepr(v)
    }
}

impl ::rand::Rand for FrRepr {
    #[inline(always)]
    fn rand<R: ::rand::Rng>(rng: &mut R) -> Self {
        FrRepr(rng.gen())
    }
}

impl ::std::fmt::Display for FrRepr
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        try!(write!(f, "0x"));
        for i in self.0.iter().rev() {
            try!(write!(f, "{:016x}", *i));
        }

        Ok(())
    }
}

impl AsRef<[u64]> for FrRepr {
    #[inline(always)]
    fn as_ref(&self) -> &[u64] {
        &self.0
    }
}

impl AsMut<[u64]> for FrRepr {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u64] {
        &mut self.0
    }
}

impl From<u64> for FrRepr {
    #[inline(always)]
    fn from(val: u64) -> FrRepr {
        let mut repr = Self::default();
        repr.0[0] = val;
        repr
    }
}

impl Ord for FrRepr {
    #[inline(always)]
    fn cmp(&self, other: &FrRepr) -> ::std::cmp::Ordering {
        for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if a < b {
                return ::std::cmp::Ordering::Less
            } else if a > b {
                return ::std::cmp::Ordering::Greater
            }
        }

        ::std::cmp::Ordering::Equal
    }
}

impl PartialOrd for FrRepr {
    #[inline(always)]
    fn partial_cmp(&self, other: &FrRepr) -> Option<::std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PrimeFieldRepr for FrRepr {
    #[inline(always)]
    fn is_odd(&self) -> bool {
        self.0[0] & 1 == 1
    }

    #[inline(always)]
    fn is_even(&self) -> bool {
        !self.is_odd()
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self.0.iter().all(|&e| e == 0)
    }

    #[inline(always)]
    fn divn(&mut self, mut n: u32) {
        if n >= 64 * 4 {
            *self = Self::from(0);
            return;
        }

        while n >= 64 {
            let mut t = 0;
            for i in self.0.iter_mut().rev() {
                ::std::mem::swap(&mut t, i);
            }
            n -= 64;
        }

        if n > 0 {
            let mut t = 0;
            for i in self.0.iter_mut().rev() {
                let t2 = *i << (64 - n);
                *i >>= n;
                *i |= t;
                t = t2;
            }
        }
    }

    #[inline(always)]
    fn div2(&mut self) {
        let mut t = 0;
        for i in self.0.iter_mut().rev() {
            let t2 = *i << 63;
            *i >>= 1;
            *i |= t;
            t = t2;
        }
    }

    #[inline(always)]
    fn mul2(&mut self) {
        let mut last = 0;
        for i in &mut self.0 {
            let tmp = *i >> 63;
            *i <<= 1;
            *i |= last;
            last = tmp;
        }
    }

    #[inline(always)]
    fn muln(&mut self, mut n: u32) {
        if n >= 64 * 4 {
            *self = Self::from(0);
            return;
        }

        while n >= 64 {
            let mut t = 0;
            for i in &mut self.0 {
                ::std::mem::swap(&mut t, i);
            }
            n -= 64;
        }

        if n > 0 {
            let mut t = 0;
            for i in &mut self.0 {
                let t2 = *i >> (64 - n);
                *i <<= n;
                *i |= t;
                t = t2;
            }
        }
    }

    #[inline(always)]
    fn num_bits(&self) -> u32 {
        let mut ret = (4 as u32) * 64;
        for i in self.0.iter().rev() {
            let leading = i.leading_zeros();
            ret -= leading;
            if leading != 64 {
                break;
            }
        }

        ret
    }

    #[inline(always)]
    fn add_nocarry(&mut self, other: &FrRepr) -> bool {
        let mut carry = 0;

        for (a, b) in self.0.iter_mut().zip(other.0.iter()) {
            *a = ::adc(*a, *b, &mut carry);
        }

        carry != 0
    }

    #[inline(always)]
    fn sub_noborrow(&mut self, other: &FrRepr) -> bool {
        let mut borrow = 0;

        for (a, b) in self.0.iter_mut().zip(other.0.iter()) {
            *a = ::sbb(*a, *b, &mut borrow);
        }

        borrow != 0
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Fr(FrRepr);

impl Fr{
    pub fn serial(&self)->[u64;4]{
        self.0.serial()
    }

    pub fn from_serial(serial:[u64;4])->Self{
        Fr(FrRepr::from_serial(serial))
    }
}

impl ::std::fmt::Display for Fr
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Fr({})", self.into_repr())
    }
}

impl ::rand::Rand for Fr {
    fn rand<R: ::rand::Rng>(rng: &mut R) -> Self {
        loop {
            let mut tmp = Fr(FrRepr::rand(rng));
            tmp.0.divn(REPR_SHAVE_BITS);
            if tmp.is_valid() {
                return tmp
            }
        }
    }
}

impl From<Fr> for FrRepr {
    fn from(e: Fr) -> FrRepr {
        e.into_repr()
    }
}

impl PrimeField for Fr {
    type Repr = FrRepr;

    fn from_repr(r: FrRepr) -> Result<Fr, PrimeFieldDecodingError> {
        let mut r = Fr(r);
        if r.is_valid() {
            r.mul_assign(&Fr(R2));

            Ok(r)
        } else {
            Err(PrimeFieldDecodingError::NotInField(format!("{}", r.0)))
        }
    }

    fn into_repr(&self) -> FrRepr {
        let mut r = *self;
        r.mont_reduce((self.0).0[0], (self.0).0[1],
                      (self.0).0[2], (self.0).0[3],
                      0, 0, 0, 0);
        r.0
    }

    fn char() -> FrRepr {
        MODULUS
    }

    fn num_bits() -> u32 {
        MODULUS_BITS
    }

    fn capacity() -> u32 {
        Self::num_bits() - 1
    }

    fn multiplicative_generator() -> Self {
        Fr(GENERATOR)
    }

    fn s() -> u32 {
        S
    }

    fn root_of_unity() -> Self {
        Fr(ROOT_OF_UNITY)
    }
}

impl Field for Fr {
    #[inline]
    fn zero() -> Self {
        Fr(FrRepr::from(0))
    }

    #[inline]
    fn one() -> Self {
        Fr(R)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[inline]
    fn add_assign(&mut self, other: &Fr) {
        // This cannot exceed the backing capacity.
        self.0.add_nocarry(&other.0);

        // However, it may need to be reduced.
        self.reduce();
    }

    #[inline]
    fn double(&mut self) {
        // This cannot exceed the backing capacity.
        self.0.mul2();

        // However, it may need to be reduced.
        self.reduce();
    }

    #[inline]
    fn sub_assign(&mut self, other: &Fr) {
        // If `other` is larger than `self`, we'll need to add the modulus to self first.
        if other.0 > self.0 {
            self.0.add_nocarry(&MODULUS);
        }

        self.0.sub_noborrow(&other.0);
    }

    #[inline]
    fn negate(&mut self) {
        if !self.is_zero() {
            let mut tmp = MODULUS;
            tmp.sub_noborrow(&self.0);
            self.0 = tmp;
        }
    }

    fn inverse(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            // Guajardo Kumar Paar Pelzl
            // Efficient Software-Implementation of Finite Fields with Applications to Cryptography
            // Algorithm 16 (BEA for Inversion in Fp)

            let one = FrRepr::from(1);

            let mut u = self.0;
            let mut v = MODULUS;
            let mut b = Fr(R2); // Avoids unnecessary reduction step.
            let mut c = Self::zero();

            while u != one && v != one {
                while u.is_even() {
                    u.div2();

                    if b.0.is_even() {
                        b.0.div2();
                    } else {
                        b.0.add_nocarry(&MODULUS);
                        b.0.div2();
                    }
                }

                while v.is_even() {
                    v.div2();

                    if c.0.is_even() {
                        c.0.div2();
                    } else {
                        c.0.add_nocarry(&MODULUS);
                        c.0.div2();
                    }
                }

                if v < u {
                    u.sub_noborrow(&v);
                    b.sub_assign(&c);
                } else {
                    v.sub_noborrow(&u);
                    c.sub_assign(&b);
                }
            }

            if u == one {
                Some(b)
            } else {
                Some(c)
            }
        }
    }

    #[inline(always)]
    fn frobenius_map(&mut self, _: usize) {
        // This has no effect in a prime field.
    }

    #[inline]
    fn mul_assign(&mut self, other: &Fr)
    {
        let mut carry = 0;
        let r0 = ::mac_with_carry(0, (self.0).0[0], (other.0).0[0], &mut carry);
        let r1 = ::mac_with_carry(0, (self.0).0[0], (other.0).0[1], &mut carry);
        let r2 = ::mac_with_carry(0, (self.0).0[0], (other.0).0[2], &mut carry);
        let r3 = ::mac_with_carry(0, (self.0).0[0], (other.0).0[3], &mut carry);
        let r4 = carry;
        let mut carry = 0;
        let r1 = ::mac_with_carry(r1, (self.0).0[1], (other.0).0[0], &mut carry);
        let r2 = ::mac_with_carry(r2, (self.0).0[1], (other.0).0[1], &mut carry);
        let r3 = ::mac_with_carry(r3, (self.0).0[1], (other.0).0[2], &mut carry);
        let r4 = ::mac_with_carry(r4, (self.0).0[1], (other.0).0[3], &mut carry);
        let r5 = carry;
        let mut carry = 0;
        let r2 = ::mac_with_carry(r2, (self.0).0[2], (other.0).0[0], &mut carry);
        let r3 = ::mac_with_carry(r3, (self.0).0[2], (other.0).0[1], &mut carry);
        let r4 = ::mac_with_carry(r4, (self.0).0[2], (other.0).0[2], &mut carry);
        let r5 = ::mac_with_carry(r5, (self.0).0[2], (other.0).0[3], &mut carry);
        let r6 = carry;
        let mut carry = 0;
        let r3 = ::mac_with_carry(r3, (self.0).0[3], (other.0).0[0], &mut carry);
        let r4 = ::mac_with_carry(r4, (self.0).0[3], (other.0).0[1], &mut carry);
        let r5 = ::mac_with_carry(r5, (self.0).0[3], (other.0).0[2], &mut carry);
        let r6 = ::mac_with_carry(r6, (self.0).0[3], (other.0).0[3], &mut carry);
        let r7 = carry;
        self.mont_reduce(r0, r1, r2, r3, r4, r5, r6, r7);
    }

    #[inline]
    fn square(&mut self)
    {
        let mut carry = 0;
        let r1 = ::mac_with_carry(0, (self.0).0[0], (self.0).0[1], &mut carry);
        let r2 = ::mac_with_carry(0, (self.0).0[0], (self.0).0[2], &mut carry);
        let r3 = ::mac_with_carry(0, (self.0).0[0], (self.0).0[3], &mut carry);
        let r4 = carry;
        let mut carry = 0;
        let r3 = ::mac_with_carry(r3, (self.0).0[1], (self.0).0[2], &mut carry);
        let r4 = ::mac_with_carry(r4, (self.0).0[1], (self.0).0[3], &mut carry);
        let r5 = carry;
        let mut carry = 0;
        let r5 = ::mac_with_carry(r5, (self.0).0[2], (self.0).0[3], &mut carry);
        let r6 = carry;

        let r7 = r6 >> 63;
        let r6 = (r6 << 1) | (r5 >> 63);
        let r5 = (r5 << 1) | (r4 >> 63);
        let r4 = (r4 << 1) | (r3 >> 63);
        let r3 = (r3 << 1) | (r2 >> 63);
        let r2 = (r2 << 1) | (r1 >> 63);
        let r1 = r1 << 1;

        let mut carry = 0;
        let r0 = ::mac_with_carry(0, (self.0).0[0], (self.0).0[0], &mut carry);
        let r1 = ::adc(r1, 0, &mut carry);
        let r2 = ::mac_with_carry(r2, (self.0).0[1], (self.0).0[1], &mut carry);
        let r3 = ::adc(r3, 0, &mut carry);
        let r4 = ::mac_with_carry(r4, (self.0).0[2], (self.0).0[2], &mut carry);
        let r5 = ::adc(r5, 0, &mut carry);
        let r6 = ::mac_with_carry(r6, (self.0).0[3], (self.0).0[3], &mut carry);
        let r7 = ::adc(r7, 0, &mut carry);
        self.mont_reduce(r0, r1, r2, r3, r4, r5, r6, r7);
    }
}

impl Fr {
    /// Determines if the element is really in the field. This is only used
    /// internally.
    #[inline(always)]
    fn is_valid(&self) -> bool {
        self.0 < MODULUS
    }

    /// Subtracts the modulus from this element if this element is not in the
    /// field. Only used internally.
    #[inline(always)]
    fn reduce(&mut self) {
        if !self.is_valid() {
            self.0.sub_noborrow(&MODULUS);
        }
    }

    #[inline(always)]
    fn mont_reduce(
        &mut self,
        r0: u64,
        mut r1: u64,
        mut r2: u64,
        mut r3: u64,
        mut r4: u64,
        mut r5: u64,
        mut r6: u64,
        mut r7: u64
    )
    {
        // The Montgomery reduction here is based on Algorithm 14.32 in
        // Handbook of Applied Cryptography
        // <http://cacr.uwaterloo.ca/hac/about/chap14.pdf>.

        let k = r0.wrapping_mul(INV);
        let mut carry = 0;
        ::mac_with_carry(r0, k, MODULUS.0[0], &mut carry);
        r1 = ::mac_with_carry(r1, k, MODULUS.0[1], &mut carry);
        r2 = ::mac_with_carry(r2, k, MODULUS.0[2], &mut carry);
        r3 = ::mac_with_carry(r3, k, MODULUS.0[3], &mut carry);
        r4 = ::adc(r4, 0, &mut carry);
        let carry2 = carry;
        let k = r1.wrapping_mul(INV);
        let mut carry = 0;
        ::mac_with_carry(r1, k, MODULUS.0[0], &mut carry);
        r2 = ::mac_with_carry(r2, k, MODULUS.0[1], &mut carry);
        r3 = ::mac_with_carry(r3, k, MODULUS.0[2], &mut carry);
        r4 = ::mac_with_carry(r4, k, MODULUS.0[3], &mut carry);
        r5 = ::adc(r5, carry2, &mut carry);
        let carry2 = carry;
        let k = r2.wrapping_mul(INV);
        let mut carry = 0;
        ::mac_with_carry(r2, k, MODULUS.0[0], &mut carry);
        r3 = ::mac_with_carry(r3, k, MODULUS.0[1], &mut carry);
        r4 = ::mac_with_carry(r4, k, MODULUS.0[2], &mut carry);
        r5 = ::mac_with_carry(r5, k, MODULUS.0[3], &mut carry);
        r6 = ::adc(r6, carry2, &mut carry);
        let carry2 = carry;
        let k = r3.wrapping_mul(INV);
        let mut carry = 0;
        ::mac_with_carry(r3, k, MODULUS.0[0], &mut carry);
        r4 = ::mac_with_carry(r4, k, MODULUS.0[1], &mut carry);
        r5 = ::mac_with_carry(r5, k, MODULUS.0[2], &mut carry);
        r6 = ::mac_with_carry(r6, k, MODULUS.0[3], &mut carry);
        r7 = ::adc(r7, carry2, &mut carry);
        (self.0).0[0] = r4;
        (self.0).0[1] = r5;
        (self.0).0[2] = r6;
        (self.0).0[3] = r7;
        self.reduce();
    }
}

impl SqrtField for Fr {
    fn sqrt(&self) -> Option<Self> {
        // Tonelli-Shank's algorithm for q mod 16 = 1
        // https://eprint.iacr.org/2012/685.pdf (page 12, algorithm 5)

        if self.is_zero() {
            return Some(*self);
        }

        // if self^((r - 1) // 2) != 1
        if self.pow([0x7fffffff80000000, 0xa9ded2017fff2dff, 0x199cec0404d0ec02, 0x39f6d3a994cebea4]) != Self::one() {
            None
        } else {
            let mut c = Fr(ROOT_OF_UNITY);
            // r = self^((t + 1) // 2)
            let mut r = self.pow([0x7fff2dff80000000, 0x4d0ec02a9ded201, 0x94cebea4199cec04, 0x39f6d3a9]);
            // t = self^t
            let mut t = self.pow([0xfffe5bfeffffffff, 0x9a1d80553bda402, 0x299d7d483339d808, 0x73eda753]);
            let mut m = S;

            while t != Self::one() {
                let mut i = 1;
                {
                    let mut t2i = t;
                    t2i.square();
                    loop {
                        if t2i == Self::one() {
                            break;
                        }
                        t2i.square();
                        i += 1;
                    }
                }

                for _ in 0..(m - i - 1) {
                    c.square();
                }
                r.mul_assign(&c);
                c.square();
                t.mul_assign(&c);
                m = i;
            }

            Some(r)
        }
    }
}

#[cfg(test)]
use rand::{SeedableRng, XorShiftRng, Rand};

#[test]
fn test_fr_repr_ordering() {
    fn assert_equality(a: FrRepr, b: FrRepr) {
        assert_eq!(a, b);
        assert!(a.cmp(&b) == ::std::cmp::Ordering::Equal);
    }

    fn assert_lt(a: FrRepr, b: FrRepr) {
        assert!(a < b);
        assert!(b > a);
    }

    assert_equality(FrRepr([9999, 9999, 9999, 9999]), FrRepr([9999, 9999, 9999, 9999]));
    assert_equality(FrRepr([9999, 9998, 9999, 9999]), FrRepr([9999, 9998, 9999, 9999]));
    assert_equality(FrRepr([9999, 9999, 9999, 9997]), FrRepr([9999, 9999, 9999, 9997]));
    assert_lt(FrRepr([9999, 9997, 9999, 9998]), FrRepr([9999, 9997, 9999, 9999]));
    assert_lt(FrRepr([9999, 9997, 9998, 9999]), FrRepr([9999, 9997, 9999, 9999]));
    assert_lt(FrRepr([9, 9999, 9999, 9997]), FrRepr([9999, 9999, 9999, 9997]));
}

#[test]
fn test_fr_repr_from() {
    assert_eq!(FrRepr::from(100), FrRepr([100, 0, 0, 0]));
}

#[test]
fn test_fr_repr_is_odd() {
    assert!(!FrRepr::from(0).is_odd());
    assert!(FrRepr::from(0).is_even());
    assert!(FrRepr::from(1).is_odd());
    assert!(!FrRepr::from(1).is_even());
    assert!(!FrRepr::from(324834872).is_odd());
    assert!(FrRepr::from(324834872).is_even());
    assert!(FrRepr::from(324834873).is_odd());
    assert!(!FrRepr::from(324834873).is_even());
}

#[test]
fn test_fr_repr_is_zero() {
    assert!(FrRepr::from(0).is_zero());
    assert!(!FrRepr::from(1).is_zero());
    assert!(!FrRepr([0, 0, 1, 0]).is_zero());
}

#[test]
fn test_fr_repr_div2() {
    let mut a = FrRepr([0xbd2920b19c972321, 0x174ed0466a3be37e, 0xd468d5e3b551f0b5, 0xcb67c072733beefc]);
    a.div2();
    assert_eq!(a, FrRepr([0x5e949058ce4b9190, 0x8ba76823351df1bf, 0x6a346af1daa8f85a, 0x65b3e039399df77e]));
    for _ in 0..10 {
        a.div2();
    }
    assert_eq!(a, FrRepr([0x6fd7a524163392e4, 0x16a2e9da08cd477c, 0xdf9a8d1abc76aa3e, 0x196cf80e4e677d]));
    for _ in 0..200 {
        a.div2();
    }
    assert_eq!(a, FrRepr([0x196cf80e4e67, 0x0, 0x0, 0x0]));
    for _ in 0..40 {
        a.div2();
    }
    assert_eq!(a, FrRepr([0x19, 0x0, 0x0, 0x0]));
    for _ in 0..4 {
        a.div2();
    }
    assert_eq!(a, FrRepr([0x1, 0x0, 0x0, 0x0]));
    a.div2();
    assert!(a.is_zero());
}

#[test]
fn test_fr_repr_divn() {
    let mut a = FrRepr([0xb33fbaec482a283f, 0x997de0d3a88cb3df, 0x9af62d2a9a0e5525, 0x36003ab08de70da1]);
    a.divn(0);
    assert_eq!(
        a,
        FrRepr([0xb33fbaec482a283f, 0x997de0d3a88cb3df, 0x9af62d2a9a0e5525, 0x36003ab08de70da1])
    );
    a.divn(1);
    assert_eq!(
        a,
        FrRepr([0xd99fdd762415141f, 0xccbef069d44659ef, 0xcd7b16954d072a92, 0x1b001d5846f386d0])
    );
    a.divn(50);
    assert_eq!(
        a,
        FrRepr([0xbc1a7511967bf667, 0xc5a55341caa4b32f, 0x75611bce1b4335e, 0x6c0])
    );
    a.divn(130);
    assert_eq!(
        a,
        FrRepr([0x1d5846f386d0cd7, 0x1b0, 0x0, 0x0])
    );
    a.divn(64);
    assert_eq!(
        a,
        FrRepr([0x1b0, 0x0, 0x0, 0x0])
    );
}

#[test]
fn test_fr_repr_mul2() {
    let mut a = FrRepr::from(23712937547);
    a.mul2();
    assert_eq!(a, FrRepr([0xb0acd6c96, 0x0, 0x0, 0x0]));
    for _ in 0..60 {
        a.mul2();
    }
    assert_eq!(a, FrRepr([0x6000000000000000, 0xb0acd6c9, 0x0, 0x0]));
    for _ in 0..128 {
        a.mul2();
    }
    assert_eq!(a, FrRepr([0x0, 0x0, 0x6000000000000000, 0xb0acd6c9]));
    for _ in 0..60 {
        a.mul2();
    }
    assert_eq!(a, FrRepr([0x0, 0x0, 0x0, 0x9600000000000000]));
    for _ in 0..7 {
        a.mul2();
    }
    assert!(a.is_zero());
}

#[test]
fn test_fr_repr_num_bits() {
    let mut a = FrRepr::from(0);
    assert_eq!(0, a.num_bits());
    a = FrRepr::from(1);
    for i in 1..257 {
        assert_eq!(i, a.num_bits());
        a.mul2();
    }
    assert_eq!(0, a.num_bits());
}

#[test]
fn test_fr_repr_sub_noborrow() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let mut t = FrRepr([0x8e62a7e85264e2c3, 0xb23d34c1941d3ca, 0x5976930b7502dd15, 0x600f3fb517bf5495]);
    t.sub_noborrow(&FrRepr([0xd64f669809cbc6a4, 0xfa76cb9d90cf7637, 0xfefb0df9038d43b3, 0x298a30c744b31acf]));
    assert!(t == FrRepr([0xb813415048991c1f, 0x10ad07ae88725d92, 0x5a7b851271759961, 0x36850eedd30c39c5]));

    for _ in 0..1000 {
        let mut a = FrRepr::rand(&mut rng);
        a.0[3] >>= 30;
        let mut b = a;
        for _ in 0..10 {
            b.mul2();
        }
        let mut c = b;
        for _ in 0..10 {
            c.mul2();
        }

        assert!(a < b);
        assert!(b < c);

        let mut csub_ba = c;
        csub_ba.sub_noborrow(&b);
        csub_ba.sub_noborrow(&a);

        let mut csub_ab = c;
        csub_ab.sub_noborrow(&a);
        csub_ab.sub_noborrow(&b);

        assert_eq!(csub_ab, csub_ba);
    }

    // Subtracting r+1 from r should produce a borrow
    let mut qplusone = FrRepr([0xffffffff00000001, 0x53bda402fffe5bfe, 0x3339d80809a1d805, 0x73eda753299d7d48]);
    assert!(qplusone.sub_noborrow(&FrRepr([0xffffffff00000002, 0x53bda402fffe5bfe, 0x3339d80809a1d805, 0x73eda753299d7d48])));

    // Subtracting x from x should produce no borrow
    let mut x = FrRepr([0xffffffff00000001, 0x53bda402fffe5bfe, 0x3339d80809a1d805, 0x73eda753299d7d48]);
    assert!(!x.sub_noborrow(&FrRepr([0xffffffff00000001, 0x53bda402fffe5bfe, 0x3339d80809a1d805, 0x73eda753299d7d48])))
}

#[test]
fn test_fr_repr_add_nocarry() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let mut t = FrRepr([0xd64f669809cbc6a4, 0xfa76cb9d90cf7637, 0xfefb0df9038d43b3, 0x298a30c744b31acf]);
    t.add_nocarry(&FrRepr([0x8e62a7e85264e2c3, 0xb23d34c1941d3ca, 0x5976930b7502dd15, 0x600f3fb517bf5495]));
    assert_eq!(t, FrRepr([0x64b20e805c30a967, 0x59a9ee9aa114a02, 0x5871a104789020c9, 0x8999707c5c726f65]));

    // Test for the associativity of addition.
    for _ in 0..1000 {
        let mut a = FrRepr::rand(&mut rng);
        let mut b = FrRepr::rand(&mut rng);
        let mut c = FrRepr::rand(&mut rng);

        // Unset the first few bits, so that overflow won't occur.
        a.0[3] >>= 3;
        b.0[3] >>= 3;
        c.0[3] >>= 3;

        let mut abc = a;
        abc.add_nocarry(&b);
        abc.add_nocarry(&c);

        let mut acb = a;
        acb.add_nocarry(&c);
        acb.add_nocarry(&b);

        let mut bac = b;
        bac.add_nocarry(&a);
        bac.add_nocarry(&c);

        let mut bca = b;
        bca.add_nocarry(&c);
        bca.add_nocarry(&a);

        let mut cab = c;
        cab.add_nocarry(&a);
        cab.add_nocarry(&b);

        let mut cba = c;
        cba.add_nocarry(&b);
        cba.add_nocarry(&a);

        assert_eq!(abc, acb);
        assert_eq!(abc, bac);
        assert_eq!(abc, bca);
        assert_eq!(abc, cab);
        assert_eq!(abc, cba);
    }

    // Adding 1 to (2^256 - 1) should produce a carry
    let mut x = FrRepr([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff]);
    assert!(x.add_nocarry(&FrRepr::from(1)));

    // Adding 1 to r should not produce a carry
    let mut x = FrRepr([0xffffffff00000001, 0x53bda402fffe5bfe, 0x3339d80809a1d805, 0x73eda753299d7d48]);
    assert!(!x.add_nocarry(&FrRepr::from(1)));
}

#[bench]
fn bench_fr_repr_add_nocarry(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<(FrRepr, FrRepr)> = (0..SAMPLES).map(|_| {
        let mut tmp1 = FrRepr::rand(&mut rng);
        let mut tmp2 = FrRepr::rand(&mut rng);
        // Shave a few bits off to avoid overflow.
        for _ in 0..3 {
            tmp1.div2();
            tmp2.div2();
        }
        (tmp1, tmp2)
    }).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count].0;
        tmp.add_nocarry(&v[count].1);
        count = (count + 1) % SAMPLES;
        tmp
    });
}

#[bench]
fn bench_fr_repr_sub_noborrow(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<(FrRepr, FrRepr)> = (0..SAMPLES).map(|_| {
        let tmp1 = FrRepr::rand(&mut rng);
        let mut tmp2 = tmp1;
        // Ensure tmp2 is smaller than tmp1.
        for _ in 0..10 {
            tmp2.div2();
        }
        (tmp1, tmp2)
    }).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count].0;
        tmp.sub_noborrow(&v[count].1);
        count = (count + 1) % SAMPLES;
        tmp
    });
}

#[bench]
fn bench_fr_repr_num_bits(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<FrRepr> = (0..SAMPLES).map(|_| FrRepr::rand(&mut rng)).collect();

    let mut count = 0;
    b.iter(|| {
        let tmp = v[count].num_bits();
        count = (count + 1) % SAMPLES;
        tmp
    });
}

#[bench]
fn bench_fr_repr_mul2(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<FrRepr> = (0..SAMPLES).map(|_| FrRepr::rand(&mut rng)).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count];
        tmp.mul2();
        count = (count + 1) % SAMPLES;
        tmp
    });
}

#[bench]
fn bench_fr_repr_div2(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<FrRepr> = (0..SAMPLES).map(|_| FrRepr::rand(&mut rng)).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count];
        tmp.div2();
        count = (count + 1) % SAMPLES;
        tmp
    });
}

#[test]
fn test_fr_is_valid() {
    let mut a = Fr(MODULUS);
    assert!(!a.is_valid());
    a.0.sub_noborrow(&FrRepr::from(1));
    assert!(a.is_valid());
    assert!(Fr(FrRepr::from(0)).is_valid());
    assert!(Fr(FrRepr([0xffffffff00000000, 0x53bda402fffe5bfe, 0x3339d80809a1d805, 0x73eda753299d7d48])).is_valid());
    assert!(!Fr(FrRepr([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff])).is_valid());

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        let a = Fr::rand(&mut rng);
        assert!(a.is_valid());
    }
}

#[test]
fn test_fr_add_assign() {
    {
        // Random number
        let mut tmp = Fr(FrRepr([0x437ce7616d580765, 0xd42d1ccb29d1235b, 0xed8f753821bd1423, 0x4eede1c9c89528ca]));
        assert!(tmp.is_valid());
        // Test that adding zero has no effect.
        tmp.add_assign(&Fr(FrRepr::from(0)));
        assert_eq!(tmp, Fr(FrRepr([0x437ce7616d580765, 0xd42d1ccb29d1235b, 0xed8f753821bd1423, 0x4eede1c9c89528ca])));
        // Add one and test for the result.
        tmp.add_assign(&Fr(FrRepr::from(1)));
        assert_eq!(tmp, Fr(FrRepr([0x437ce7616d580766, 0xd42d1ccb29d1235b, 0xed8f753821bd1423, 0x4eede1c9c89528ca])));
        // Add another random number that exercises the reduction.
        tmp.add_assign(&Fr(FrRepr([0x946f435944f7dc79, 0xb55e7ee6533a9b9b, 0x1e43b84c2f6194ca, 0x58717ab525463496])));
        assert_eq!(tmp, Fr(FrRepr([0xd7ec2abbb24fe3de, 0x35cdf7ae7d0d62f7, 0xd899557c477cd0e9, 0x3371b52bc43de018])));
        // Add one to (r - 1) and test for the result.
        tmp = Fr(FrRepr([0xffffffff00000000, 0x53bda402fffe5bfe, 0x3339d80809a1d805, 0x73eda753299d7d48]));
        tmp.add_assign(&Fr(FrRepr::from(1)));
        assert!(tmp.0.is_zero());
        // Add a random number to another one such that the result is r - 1
        tmp = Fr(FrRepr([0xade5adacdccb6190, 0xaa21ee0f27db3ccd, 0x2550f4704ae39086, 0x591d1902e7c5ba27]));
        tmp.add_assign(&Fr(FrRepr([0x521a525223349e70, 0xa99bb5f3d8231f31, 0xde8e397bebe477e, 0x1ad08e5041d7c321])));
        assert_eq!(tmp, Fr(FrRepr([0xffffffff00000000, 0x53bda402fffe5bfe, 0x3339d80809a1d805, 0x73eda753299d7d48])));
        // Add one to the result and test for it.
        tmp.add_assign(&Fr(FrRepr::from(1)));
        assert!(tmp.0.is_zero());
    }

    // Test associativity

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        // Generate a, b, c and ensure (a + b) + c == a + (b + c).
        let a = Fr::rand(&mut rng);
        let b = Fr::rand(&mut rng);
        let c = Fr::rand(&mut rng);

        let mut tmp1 = a;
        tmp1.add_assign(&b);
        tmp1.add_assign(&c);

        let mut tmp2 = b;
        tmp2.add_assign(&c);
        tmp2.add_assign(&a);

        assert!(tmp1.is_valid());
        assert!(tmp2.is_valid());
        assert_eq!(tmp1, tmp2);
    }
}

#[test]
fn test_fr_sub_assign() {
    {
        // Test arbitrary subtraction that tests reduction.
        let mut tmp = Fr(FrRepr([0x6a68c64b6f735a2b, 0xd5f4d143fe0a1972, 0x37c17f3829267c62, 0xa2f37391f30915c]));
        tmp.sub_assign(&Fr(FrRepr([0xade5adacdccb6190, 0xaa21ee0f27db3ccd, 0x2550f4704ae39086, 0x591d1902e7c5ba27])));
        assert_eq!(tmp, Fr(FrRepr([0xbc83189d92a7f89c, 0x7f908737d62d38a3, 0x45aa62cfe7e4c3e1, 0x24ffc5896108547d])));
        
        // Test the opposite subtraction which doesn't test reduction.
        tmp = Fr(FrRepr([0xade5adacdccb6190, 0xaa21ee0f27db3ccd, 0x2550f4704ae39086, 0x591d1902e7c5ba27]));
        tmp.sub_assign(&Fr(FrRepr([0x6a68c64b6f735a2b, 0xd5f4d143fe0a1972, 0x37c17f3829267c62, 0xa2f37391f30915c])));
        assert_eq!(tmp, Fr(FrRepr([0x437ce7616d580765, 0xd42d1ccb29d1235b, 0xed8f753821bd1423, 0x4eede1c9c89528ca])));
        
        // Test for sensible results with zero
        tmp = Fr(FrRepr::from(0));
        tmp.sub_assign(&Fr(FrRepr::from(0)));
        assert!(tmp.is_zero());

        tmp = Fr(FrRepr([0x437ce7616d580765, 0xd42d1ccb29d1235b, 0xed8f753821bd1423, 0x4eede1c9c89528ca]));
        tmp.sub_assign(&Fr(FrRepr::from(0)));
        assert_eq!(tmp, Fr(FrRepr([0x437ce7616d580765, 0xd42d1ccb29d1235b, 0xed8f753821bd1423, 0x4eede1c9c89528ca])));
    }

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        // Ensure that (a - b) + (b - a) = 0.
        let a = Fr::rand(&mut rng);
        let b = Fr::rand(&mut rng);

        let mut tmp1 = a;
        tmp1.sub_assign(&b);

        let mut tmp2 = b;
        tmp2.sub_assign(&a);

        tmp1.add_assign(&tmp2);
        assert!(tmp1.is_zero());
    }
}

#[bench]
fn bench_fr_add_assign(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<(Fr, Fr)> = (0..SAMPLES).map(|_| (Fr::rand(&mut rng), Fr::rand(&mut rng))).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count].0;
        tmp.add_assign(&v[count].1);
        count = (count + 1) % SAMPLES;
        tmp
    });
}

#[bench]
fn bench_fr_sub_assign(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<(Fr, Fr)> = (0..SAMPLES).map(|_| (Fr::rand(&mut rng), Fr::rand(&mut rng))).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count].0;
        tmp.sub_assign(&v[count].1);
        count = (count + 1) % SAMPLES;
        tmp
    });
}

#[test]
fn test_fr_mul_assign() {
    let mut tmp = Fr(FrRepr([0x6b7e9b8faeefc81a, 0xe30a8463f348ba42, 0xeff3cb67a8279c9c, 0x3d303651bd7c774d]));
    tmp.mul_assign(&Fr(FrRepr([0x13ae28e3bc35ebeb, 0xa10f4488075cae2c, 0x8160e95a853c3b5d, 0x5ae3f03b561a841d])));
    assert!(tmp == Fr(FrRepr([0x23717213ce710f71, 0xdbee1fe53a16e1af, 0xf565d3e1c2a48000, 0x4426507ee75df9d7])));

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000000 {
        // Ensure that (a * b) * c = a * (b * c)
        let a = Fr::rand(&mut rng);
        let b = Fr::rand(&mut rng);
        let c = Fr::rand(&mut rng);

        let mut tmp1 = a;
        tmp1.mul_assign(&b);
        tmp1.mul_assign(&c);

        let mut tmp2 = b;
        tmp2.mul_assign(&c);
        tmp2.mul_assign(&a);

        assert_eq!(tmp1, tmp2);
    }

    for _ in 0..1000000 {
        // Ensure that r * (a + b + c) = r*a + r*b + r*c

        let r = Fr::rand(&mut rng);
        let mut a = Fr::rand(&mut rng);
        let mut b = Fr::rand(&mut rng);
        let mut c = Fr::rand(&mut rng);

        let mut tmp1 = a;
        tmp1.add_assign(&b);
        tmp1.add_assign(&c);
        tmp1.mul_assign(&r);

        a.mul_assign(&r);
        b.mul_assign(&r);
        c.mul_assign(&r);

        a.add_assign(&b);
        a.add_assign(&c);

        assert_eq!(tmp1, a);
    }
}

#[bench]
fn bench_fr_mul_assign(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<(Fr, Fr)> = (0..SAMPLES).map(|_| (Fr::rand(&mut rng), Fr::rand(&mut rng))).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count].0;
        tmp.mul_assign(&v[count].1);
        count = (count + 1) % SAMPLES;
        tmp
    });
}

#[test]
fn test_fr_squaring() {
    let mut a = Fr(FrRepr([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0x73eda753299d7d47]));
    assert!(a.is_valid());
    a.square();
    assert_eq!(a, Fr::from_repr(FrRepr([0xc0d698e7bde077b8, 0xb79a310579e76ec2, 0xac1da8d0a9af4e5f, 0x13f629c49bf23e97])).unwrap());

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000000 {
        // Ensure that (a * a) = a^2
        let a = Fr::rand(&mut rng);

        let mut tmp = a;
        tmp.square();

        let mut tmp2 = a;
        tmp2.mul_assign(&a);

        assert_eq!(tmp, tmp2);
    }
}

#[bench]
fn bench_fr_square(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<Fr> = (0..SAMPLES).map(|_| Fr::rand(&mut rng)).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count];
        tmp.square();
        count = (count + 1) % SAMPLES;
        tmp
    });
}


#[test]
fn test_fr_inverse() {
    assert!(Fr::zero().inverse().is_none());

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let one = Fr::one();

    for _ in 0..1000 {
        // Ensure that a * a^-1 = 1
        let mut a = Fr::rand(&mut rng);
        let ainv = a.inverse().unwrap();
        a.mul_assign(&ainv);
        assert_eq!(a, one);
    }
}

#[bench]
fn bench_fr_inverse(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<Fr> = (0..SAMPLES).map(|_| Fr::rand(&mut rng)).collect();

    let mut count = 0;
    b.iter(|| {
        count = (count + 1) % SAMPLES;
        v[count].inverse()
    });
}

#[test]
fn test_fr_double() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        // Ensure doubling a is equivalent to adding a to itself.
        let mut a = Fr::rand(&mut rng);
        let mut b = a;
        b.add_assign(&a);
        a.double();
        assert_eq!(a, b);
    }
}

#[test]
fn test_fr_negate() {
    {
        let mut a = Fr::zero();
        a.negate();

        assert!(a.is_zero());
    }

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        // Ensure (a - (-a)) = 0.
        let mut a = Fr::rand(&mut rng);
        let mut b = a;
        b.negate();
        a.add_assign(&b);

        assert!(a.is_zero());
    }
}

#[bench]
fn bench_fr_negate(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<Fr> = (0..SAMPLES).map(|_| Fr::rand(&mut rng)).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count];
        tmp.negate();
        count = (count + 1) % SAMPLES;
        tmp
    });
}


#[test]
fn test_fr_pow() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for i in 0..1000 {
        // Exponentiate by various small numbers and ensure it consists with repeated
        // multiplication.
        let a = Fr::rand(&mut rng);
        let target = a.pow(&[i]);
        let mut c = Fr::one();
        for _ in 0..i {
            c.mul_assign(&a);
        }
        assert_eq!(c, target);
    }

    for _ in 0..1000 {
        // Exponentiating by the modulus should have no effect in a prime field.
        let a = Fr::rand(&mut rng);

        assert_eq!(a, a.pow(Fr::char()));
    }
}

#[test]
fn test_fr_sqrt() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    assert_eq!(Fr::zero().sqrt().unwrap(), Fr::zero());

    for _ in 0..1000 {
        // Ensure sqrt(a^2) = a or -a
        let a = Fr::rand(&mut rng);
        let mut nega = a;
        nega.negate();
        let mut b = a;
        b.square();

        let b = b.sqrt().unwrap();

        assert!(a == b || nega == b);
    }

    for _ in 0..1000 {
        // Ensure sqrt(a)^2 = a for random a
        let a = Fr::rand(&mut rng);

        if let Some(mut tmp) = a.sqrt() {
            tmp.square();

            assert_eq!(a, tmp);
        }
    }
}

#[bench]
fn bench_fr_sqrt(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<Fr> = (0..SAMPLES).map(|_| {
        let mut tmp = Fr::rand(&mut rng);
        tmp.square();
        tmp
    }).collect();

    let mut count = 0;
    b.iter(|| {
        count = (count + 1) % SAMPLES;
        v[count].sqrt()
    });
}

#[test]
fn test_fr_from_into_repr() {
    // r + 1 should not be in the field
    assert!(Fr::from_repr(FrRepr([0xffffffff00000002, 0x53bda402fffe5bfe, 0x3339d80809a1d805, 0x73eda753299d7d48])).is_err());

    // r should not be in the field
    assert!(Fr::from_repr(Fr::char()).is_err());

    // Multiply some arbitrary representations to see if the result is as expected.
    let a = FrRepr([0x25ebe3a3ad3c0c6a, 0x6990e39d092e817c, 0x941f900d42f5658e, 0x44f8a103b38a71e0]);
    let mut a_fr = Fr::from_repr(a).unwrap();
    let b = FrRepr([0x264e9454885e2475, 0x46f7746bb0308370, 0x4683ef5347411f9, 0x58838d7f208d4492]);
    let b_fr = Fr::from_repr(b).unwrap();
    let c = FrRepr([0x48a09ab93cfc740d, 0x3a6600fbfc7a671, 0x838567017501d767, 0x7161d6da77745512]);
    a_fr.mul_assign(&b_fr);
    assert_eq!(a_fr.into_repr(), c);

    // Zero should be in the field.
    assert!(Fr::from_repr(FrRepr::from(0)).unwrap().is_zero());

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        // Try to turn Fr elements into representations and back again, and compare.
        let a = Fr::rand(&mut rng);
        let a_repr = a.into_repr();
        let b_repr = FrRepr::from(a);
        assert_eq!(a_repr, b_repr);
        let a_again = Fr::from_repr(a_repr).unwrap();

        assert_eq!(a, a_again);
    }
}

#[bench]
fn bench_fr_into_repr(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<Fr> = (0..SAMPLES).map(|_| {
        Fr::rand(&mut rng)
    }).collect();

    let mut count = 0;
    b.iter(|| {
        count = (count + 1) % SAMPLES;
        v[count].into_repr()
    });
}

#[bench]
fn bench_fr_from_repr(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<FrRepr> = (0..SAMPLES).map(|_| {
        Fr::rand(&mut rng).into_repr()
    }).collect();

    let mut count = 0;
    b.iter(|| {
        count = (count + 1) % SAMPLES;
        Fr::from_repr(v[count])
    });
}

#[test]
fn test_fr_repr_display() {
    assert_eq!(
        format!("{}", FrRepr([0x2829c242fa826143, 0x1f32cf4dd4330917, 0x932e4e479d168cd9, 0x513c77587f563f64])),
        "0x513c77587f563f64932e4e479d168cd91f32cf4dd43309172829c242fa826143".to_string()
    );
    assert_eq!(
        format!("{}", FrRepr([0x25ebe3a3ad3c0c6a, 0x6990e39d092e817c, 0x941f900d42f5658e, 0x44f8a103b38a71e0])),
        "0x44f8a103b38a71e0941f900d42f5658e6990e39d092e817c25ebe3a3ad3c0c6a".to_string()
    );
    assert_eq!(
        format!("{}", FrRepr([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff])),
        "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_string()
    );
    assert_eq!(
        format!("{}", FrRepr([0, 0, 0, 0])),
        "0x0000000000000000000000000000000000000000000000000000000000000000".to_string()
    );
}

#[test]
fn test_fr_display() {
    assert_eq!(
        format!("{}", Fr::from_repr(FrRepr([0xc3cae746a3b5ecc7, 0x185ec8eb3f5b5aee, 0x684499ffe4b9dd99, 0x7c9bba7afb68faa])).unwrap()),
        "Fr(0x07c9bba7afb68faa684499ffe4b9dd99185ec8eb3f5b5aeec3cae746a3b5ecc7)".to_string()
    );
    assert_eq!(
        format!("{}", Fr::from_repr(FrRepr([0x44c71298ff198106, 0xb0ad10817df79b6a, 0xd034a80a2b74132b, 0x41cf9a1336f50719])).unwrap()),
        "Fr(0x41cf9a1336f50719d034a80a2b74132bb0ad10817df79b6a44c71298ff198106)".to_string()
    );
}

#[test]
fn test_fr_num_bits() {
    assert_eq!(Fr::num_bits(), 255);
    assert_eq!(Fr::capacity(), 254);
}

#[test]
fn test_fr_root_of_unity() {
    assert_eq!(Fr::s(), 32);
    assert_eq!(Fr::multiplicative_generator(), Fr::from_repr(FrRepr::from(7)).unwrap());
    assert_eq!(
        Fr::multiplicative_generator().pow([0xfffe5bfeffffffff, 0x9a1d80553bda402, 0x299d7d483339d808, 0x73eda753]),
        Fr::root_of_unity()
    );
    assert_eq!(
        Fr::root_of_unity().pow([1 << Fr::s()]),
        Fr::one()
    );
    assert!(Fr::multiplicative_generator().sqrt().is_none());
}

#[test]
fn fr_field_tests() {
    ::tests::field::random_field_tests::<Fr>();
    ::tests::field::random_sqrt_tests::<Fr>();
    ::tests::field::random_frobenius_tests::<Fr, _>(Fr::char(), 13);
    ::tests::field::from_str_tests::<Fr>();
}

#[test]
fn fr_repr_tests() {
    ::tests::repr::random_repr_tests::<FrRepr>();
}
