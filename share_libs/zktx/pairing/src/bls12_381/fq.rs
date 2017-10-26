use ::{Field, PrimeField, SqrtField, PrimeFieldRepr, PrimeFieldDecodingError};
use std::cmp::Ordering;
use super::fq2::Fq2;

// q = 4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787
const MODULUS: FqRepr = FqRepr([0xb9feffffffffaaab, 0x1eabfffeb153ffff, 0x6730d2a0f6b0f624, 0x64774b84f38512bf, 0x4b1ba7b6434bacd7, 0x1a0111ea397fe69a]);

// The number of bits needed to represent the modulus.
const MODULUS_BITS: u32 = 381;

// The number of bits that must be shaved from the beginning of
// the representation when randomly sampling.
const REPR_SHAVE_BITS: u32 = 3;

// R = 2**384 % q
const R: FqRepr = FqRepr([0x760900000002fffd, 0xebf4000bc40c0002, 0x5f48985753c758ba, 0x77ce585370525745, 0x5c071a97a256ec6d, 0x15f65ec3fa80e493]);

// R2 = R^2 % q
const R2: FqRepr = FqRepr([0xf4df1f341c341746, 0xa76e6a609d104f1, 0x8de5476c4c95b6d5, 0x67eb88a9939d83c0, 0x9a793e85b519952d, 0x11988fe592cae3aa]);

// INV = -(q^{-1} mod q) mod q
const INV: u64 = 0x89f3fffcfffcfffd;

// GENERATOR = 2 (multiplicative generator of q-1 order, that is also quadratic nonresidue)
const GENERATOR: FqRepr = FqRepr([0x321300000006554f, 0xb93c0018d6c40005, 0x57605e0db0ddbb51, 0x8b256521ed1f9bcb, 0x6cf28d7901622c03, 0x11ebab9dbb81e28c]);

// 2^s * t = MODULUS - 1 with t odd
const S: u32 = 1;

// 2^s root of unity computed by GENERATOR^t
const ROOT_OF_UNITY: FqRepr = FqRepr([0x43f5fffffffcaaae, 0x32b7fff2ed47fffd, 0x7e83a49a2e99d69, 0xeca8f3318332bb7a, 0xef148d1ea0f4c069, 0x40ab3263eff0206]);

// B coefficient of BLS12-381 curve, 4.
pub const B_COEFF: Fq = Fq(FqRepr([0xaa270000000cfff3, 0x53cc0032fc34000a, 0x478fe97a6b0a807f, 0xb1d37ebee6ba24d7, 0x8ec9733bbf78ab2f, 0x9d645513d83de7e]));

// The generators of G1/G2 are computed by finding the lexicographically smallest valid x coordinate,
// and its lexicographically smallest y coordinate and multiplying it by the cofactor such that the
// result is nonzero.

// Generator of G1
// x = 3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507
// y = 1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569
pub const G1_GENERATOR_X: Fq = Fq(FqRepr([0x5cb38790fd530c16, 0x7817fc679976fff5, 0x154f95c7143ba1c1, 0xf0ae6acdf3d0e747, 0xedce6ecc21dbf440, 0x120177419e0bfb75]));
pub const G1_GENERATOR_Y: Fq = Fq(FqRepr([0xbaac93d50ce72271, 0x8c22631a7918fd8e, 0xdd595f13570725ce, 0x51ac582950405194, 0xe1c8c3fad0059c0, 0xbbc3efc5008a26a]));

// Generator of G2
// x = 3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758*u + 352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160
// y = 927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582*u + 1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905
pub const G2_GENERATOR_X_C0: Fq = Fq(FqRepr([0xf5f28fa202940a10, 0xb3f5fb2687b4961a, 0xa1a893b53e2ae580, 0x9894999d1a3caee9, 0x6f67b7631863366b, 0x58191924350bcd7]));
pub const G2_GENERATOR_X_C1: Fq = Fq(FqRepr([0xa5a9c0759e23f606, 0xaaa0c59dbccd60c3, 0x3bb17e18e2867806, 0x1b1ab6cc8541b367, 0xc2b6ed0ef2158547, 0x11922a097360edf3]));
pub const G2_GENERATOR_Y_C0: Fq = Fq(FqRepr([0x4c730af860494c4a, 0x597cfa1f5e369c5a, 0xe7e6856caa0a635a, 0xbbefb5e96e0d495f, 0x7d3a975f0ef25a2, 0x83fd8e7e80dae5]));
pub const G2_GENERATOR_Y_C1: Fq = Fq(FqRepr([0xadc0fc92df64b05d, 0x18aa270a2b1461dc, 0x86adac6a3be4eba0, 0x79495c4ec93da33a, 0xe7175850a43ccaed, 0xb2bc2a163de1bf2]));

// Coefficients for the Frobenius automorphism.
pub const FROBENIUS_COEFF_FQ2_C1: [Fq; 2] = [
    // Fq(-1)**(((q^0) - 1) / 2)
    Fq(FqRepr([0x760900000002fffd, 0xebf4000bc40c0002, 0x5f48985753c758ba, 0x77ce585370525745, 0x5c071a97a256ec6d, 0x15f65ec3fa80e493])),
    // Fq(-1)**(((q^1) - 1) / 2)
    Fq(FqRepr([0x43f5fffffffcaaae, 0x32b7fff2ed47fffd, 0x7e83a49a2e99d69, 0xeca8f3318332bb7a, 0xef148d1ea0f4c069, 0x40ab3263eff0206]))
];

pub const FROBENIUS_COEFF_FQ6_C1: [Fq2; 6] = [
    // Fq2(u + 1)**(((q^0) - 1) / 3)
    Fq2 {
        c0: Fq(FqRepr([0x760900000002fffd, 0xebf4000bc40c0002, 0x5f48985753c758ba, 0x77ce585370525745, 0x5c071a97a256ec6d, 0x15f65ec3fa80e493])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]))
    },
    // Fq2(u + 1)**(((q^1) - 1) / 3)
    Fq2 {
        c0: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
        c1: Fq(FqRepr([0xcd03c9e48671f071, 0x5dab22461fcda5d2, 0x587042afd3851b95, 0x8eb60ebe01bacb9e, 0x3f97d6e83d050d2, 0x18f0206554638741]))
    },
    // Fq2(u + 1)**(((q^2) - 1) / 3)
    Fq2 {
        c0: Fq(FqRepr([0x30f1361b798a64e8, 0xf3b8ddab7ece5a2a, 0x16a8ca3ac61577f7, 0xc26a2ff874fd029b, 0x3636b76660701c6e, 0x51ba4ab241b6160])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]))
    },
    // Fq2(u + 1)**(((q^3) - 1) / 3)
    Fq2 {
        c0: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
        c1: Fq(FqRepr([0x760900000002fffd, 0xebf4000bc40c0002, 0x5f48985753c758ba, 0x77ce585370525745, 0x5c071a97a256ec6d, 0x15f65ec3fa80e493]))
    },
    // Fq2(u + 1)**(((q^4) - 1) / 3)
    Fq2 {
        c0: Fq(FqRepr([0xcd03c9e48671f071, 0x5dab22461fcda5d2, 0x587042afd3851b95, 0x8eb60ebe01bacb9e, 0x3f97d6e83d050d2, 0x18f0206554638741])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]))
    },
    // Fq2(u + 1)**(((q^5) - 1) / 3)
    Fq2 {
        c0: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0])),
        c1: Fq(FqRepr([0x30f1361b798a64e8, 0xf3b8ddab7ece5a2a, 0x16a8ca3ac61577f7, 0xc26a2ff874fd029b, 0x3636b76660701c6e, 0x51ba4ab241b6160]))
    }
];

pub const FROBENIUS_COEFF_FQ6_C2: [Fq2; 6] = [
    // Fq2(u + 1)**(((2q^0) - 2) / 3)
    Fq2 {
        c0: Fq(FqRepr([0x760900000002fffd, 0xebf4000bc40c0002, 0x5f48985753c758ba, 0x77ce585370525745, 0x5c071a97a256ec6d, 0x15f65ec3fa80e493])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]))
    },
    // Fq2(u + 1)**(((2q^1) - 2) / 3)
    Fq2 {
        c0: Fq(FqRepr([0x890dc9e4867545c3, 0x2af322533285a5d5, 0x50880866309b7e2c, 0xa20d1b8c7e881024, 0x14e4f04fe2db9068, 0x14e56d3f1564853a])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]))
    },
    // Fq2(u + 1)**(((2q^2) - 2) / 3)
    Fq2 {
        c0: Fq(FqRepr([0xcd03c9e48671f071, 0x5dab22461fcda5d2, 0x587042afd3851b95, 0x8eb60ebe01bacb9e, 0x3f97d6e83d050d2, 0x18f0206554638741])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]))
    },
    // Fq2(u + 1)**(((2q^3) - 2) / 3)
    Fq2 {
        c0: Fq(FqRepr([0x43f5fffffffcaaae, 0x32b7fff2ed47fffd, 0x7e83a49a2e99d69, 0xeca8f3318332bb7a, 0xef148d1ea0f4c069, 0x40ab3263eff0206])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]))
    },
    // Fq2(u + 1)**(((2q^4) - 2) / 3)
    Fq2 {
        c0: Fq(FqRepr([0x30f1361b798a64e8, 0xf3b8ddab7ece5a2a, 0x16a8ca3ac61577f7, 0xc26a2ff874fd029b, 0x3636b76660701c6e, 0x51ba4ab241b6160])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]))
    },
    // Fq2(u + 1)**(((2q^5) - 2) / 3)
    Fq2 {
        c0: Fq(FqRepr([0xecfb361b798dba3a, 0xc100ddb891865a2c, 0xec08ff1232bda8e, 0xd5c13cc6f1ca4721, 0x47222a47bf7b5c04, 0x110f184e51c5f59])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]))
    }
];

// non_residue^((modulus^i-1)/6) for i=0,...,11
pub const FROBENIUS_COEFF_FQ12_C1: [Fq2; 12] = [
    // Fq2(u + 1)**(((q^0) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([0x760900000002fffd, 0xebf4000bc40c0002, 0x5f48985753c758ba, 0x77ce585370525745, 0x5c071a97a256ec6d, 0x15f65ec3fa80e493])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]))
    },
    // Fq2(u + 1)**(((q^1) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([0x7089552b319d465, 0xc6695f92b50a8313, 0x97e83cccd117228f, 0xa35baecab2dc29ee, 0x1ce393ea5daace4d, 0x8f2220fb0fb66eb])),
        c1: Fq(FqRepr([0xb2f66aad4ce5d646, 0x5842a06bfc497cec, 0xcf4895d42599d394, 0xc11b9cba40a8e8d0, 0x2e3813cbe5a0de89, 0x110eefda88847faf]))
    },
    // Fq2(u + 1)**(((q^2) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([0xecfb361b798dba3a, 0xc100ddb891865a2c, 0xec08ff1232bda8e, 0xd5c13cc6f1ca4721, 0x47222a47bf7b5c04, 0x110f184e51c5f59])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]))
    },
    // Fq2(u + 1)**(((q^3) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([0x3e2f585da55c9ad1, 0x4294213d86c18183, 0x382844c88b623732, 0x92ad2afd19103e18, 0x1d794e4fac7cf0b9, 0xbd592fc7d825ec8])),
        c1: Fq(FqRepr([0x7bcfa7a25aa30fda, 0xdc17dec12a927e7c, 0x2f088dd86b4ebef1, 0xd1ca2087da74d4a7, 0x2da2596696cebc1d, 0xe2b7eedbbfd87d2]))
    },
    // Fq2(u + 1)**(((q^4) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([0x30f1361b798a64e8, 0xf3b8ddab7ece5a2a, 0x16a8ca3ac61577f7, 0xc26a2ff874fd029b, 0x3636b76660701c6e, 0x51ba4ab241b6160])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]))
    },
    // Fq2(u + 1)**(((q^5) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([0x3726c30af242c66c, 0x7c2ac1aad1b6fe70, 0xa04007fbba4b14a2, 0xef517c3266341429, 0x95ba654ed2226b, 0x2e370eccc86f7dd])),
        c1: Fq(FqRepr([0x82d83cf50dbce43f, 0xa2813e53df9d018f, 0xc6f0caa53c65e181, 0x7525cf528d50fe95, 0x4a85ed50f4798a6b, 0x171da0fd6cf8eebd]))
    },
    // Fq2(u + 1)**(((q^6) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([0x43f5fffffffcaaae, 0x32b7fff2ed47fffd, 0x7e83a49a2e99d69, 0xeca8f3318332bb7a, 0xef148d1ea0f4c069, 0x40ab3263eff0206])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]))
    },
    // Fq2(u + 1)**(((q^7) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([0xb2f66aad4ce5d646, 0x5842a06bfc497cec, 0xcf4895d42599d394, 0xc11b9cba40a8e8d0, 0x2e3813cbe5a0de89, 0x110eefda88847faf])),
        c1: Fq(FqRepr([0x7089552b319d465, 0xc6695f92b50a8313, 0x97e83cccd117228f, 0xa35baecab2dc29ee, 0x1ce393ea5daace4d, 0x8f2220fb0fb66eb]))
    },
    // Fq2(u + 1)**(((q^8) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([0xcd03c9e48671f071, 0x5dab22461fcda5d2, 0x587042afd3851b95, 0x8eb60ebe01bacb9e, 0x3f97d6e83d050d2, 0x18f0206554638741])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]))
    },
    // Fq2(u + 1)**(((q^9) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([0x7bcfa7a25aa30fda, 0xdc17dec12a927e7c, 0x2f088dd86b4ebef1, 0xd1ca2087da74d4a7, 0x2da2596696cebc1d, 0xe2b7eedbbfd87d2])),
        c1: Fq(FqRepr([0x3e2f585da55c9ad1, 0x4294213d86c18183, 0x382844c88b623732, 0x92ad2afd19103e18, 0x1d794e4fac7cf0b9, 0xbd592fc7d825ec8]))
    },
    // Fq2(u + 1)**(((q^10) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([0x890dc9e4867545c3, 0x2af322533285a5d5, 0x50880866309b7e2c, 0xa20d1b8c7e881024, 0x14e4f04fe2db9068, 0x14e56d3f1564853a])),
        c1: Fq(FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x0]))
    },
    // Fq2(u + 1)**(((q^11) - 1) / 6)
    Fq2 {
        c0: Fq(FqRepr([0x82d83cf50dbce43f, 0xa2813e53df9d018f, 0xc6f0caa53c65e181, 0x7525cf528d50fe95, 0x4a85ed50f4798a6b, 0x171da0fd6cf8eebd])),
        c1: Fq(FqRepr([0x3726c30af242c66c, 0x7c2ac1aad1b6fe70, 0xa04007fbba4b14a2, 0xef517c3266341429, 0x95ba654ed2226b, 0x2e370eccc86f7dd]))
    }
];

// -((2**384) mod q) mod q
pub const NEGATIVE_ONE: Fq = Fq(FqRepr([0x43f5fffffffcaaae, 0x32b7fff2ed47fffd, 0x7e83a49a2e99d69, 0xeca8f3318332bb7a, 0xef148d1ea0f4c069, 0x40ab3263eff0206]));

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct FqRepr(pub [u64; 6]);

impl FqRepr{
    pub fn serial(&self)->[u64;6]{
        self.0
    }
    pub fn from_serial(serial:[u64;6])->FqRepr{
        FqRepr(serial)
    }
}

impl ::rand::Rand for FqRepr {
    #[inline(always)]
    fn rand<R: ::rand::Rng>(rng: &mut R) -> Self {
        FqRepr(rng.gen())
    }
}

impl ::std::fmt::Display for FqRepr
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        try!(write!(f, "0x"));
        for i in self.0.iter().rev() {
            try!(write!(f, "{:016x}", *i));
        }

        Ok(())
    }
}

impl AsRef<[u64]> for FqRepr {
    #[inline(always)]
    fn as_ref(&self) -> &[u64] {
        &self.0
    }
}

impl AsMut<[u64]> for FqRepr {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [u64] {
        &mut self.0
    }
}

impl From<u64> for FqRepr {
    #[inline(always)]
    fn from(val: u64) -> FqRepr {
        let mut repr = Self::default();
        repr.0[0] = val;
        repr
    }
}

impl Ord for FqRepr {
    #[inline(always)]
    fn cmp(&self, other: &FqRepr) -> Ordering {
        for (a, b) in self.0.iter().rev().zip(other.0.iter().rev()) {
            if a < b {
                return Ordering::Less
            } else if a > b {
                return Ordering::Greater
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for FqRepr {
    #[inline(always)]
    fn partial_cmp(&self, other: &FqRepr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PrimeFieldRepr for FqRepr {
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
        if n >= 64 * 6 {
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
        if n >= 64 * 6 {
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
        let mut ret = (6 as u32) * 64;
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
    fn add_nocarry(&mut self, other: &FqRepr) -> bool {
        let mut carry = 0;

        for (a, b) in self.0.iter_mut().zip(other.0.iter()) {
            *a = ::adc(*a, *b, &mut carry);
        }

        carry != 0
    }

    #[inline(always)]
    fn sub_noborrow(&mut self, other: &FqRepr) -> bool {
        let mut borrow = 0;

        for (a, b) in self.0.iter_mut().zip(other.0.iter()) {
            *a = ::sbb(*a, *b, &mut borrow);
        }

        borrow != 0
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Fq(FqRepr);

impl Fq{
    pub fn serial(&self)->[u64;6]{
        self.0.serial()
    }
    pub fn from_serial(serial:[u64;6])->Fq{
        Fq(FqRepr::from_serial(serial))
    }
}

/// `Fq` elements are ordered lexicographically.
impl Ord for Fq {
    #[inline(always)]
    fn cmp(&self, other: &Fq) -> Ordering {
        self.into_repr().cmp(&other.into_repr())
    }
}

impl PartialOrd for Fq {
    #[inline(always)]
    fn partial_cmp(&self, other: &Fq) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ::std::fmt::Display for Fq
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "Fq({})", self.into_repr())
    }
}

impl ::rand::Rand for Fq {
    fn rand<R: ::rand::Rng>(rng: &mut R) -> Self {
        loop {
            let mut tmp = Fq(FqRepr::rand(rng));
            tmp.0.divn(REPR_SHAVE_BITS);
            if tmp.is_valid() {
                return tmp
            }
        }
    }
}

impl From<Fq> for FqRepr {
    fn from(e: Fq) -> FqRepr {
        e.into_repr()
    }
}

impl PrimeField for Fq {
    type Repr = FqRepr;

    fn from_repr(r: FqRepr) -> Result<Fq, PrimeFieldDecodingError> {
        let mut r = Fq(r);
        if r.is_valid() {
            r.mul_assign(&Fq(R2));

            Ok(r)
        } else {
            Err(PrimeFieldDecodingError::NotInField(format!("{}", r.0)))
        }
    }

    fn into_repr(&self) -> FqRepr {
        let mut r = *self;
        r.mont_reduce((self.0).0[0], (self.0).0[1],
                      (self.0).0[2], (self.0).0[3],
                      (self.0).0[4], (self.0).0[5],
                      0, 0, 0, 0, 0, 0);
        r.0
    }

    fn char() -> FqRepr {
        MODULUS
    }

    fn num_bits() -> u32 {
        MODULUS_BITS
    }

    fn capacity() -> u32 {
        Self::num_bits() - 1
    }

    fn multiplicative_generator() -> Self {
        Fq(GENERATOR)
    }

    fn s() -> u32 {
        S
    }

    fn root_of_unity() -> Self {
        Fq(ROOT_OF_UNITY)
    }
}

impl Field for Fq {
    #[inline]
    fn zero() -> Self {
        Fq(FqRepr::from(0))
    }

    #[inline]
    fn one() -> Self {
        Fq(R)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    #[inline]
    fn add_assign(&mut self, other: &Fq) {
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
    fn sub_assign(&mut self, other: &Fq) {
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

            let one = FqRepr::from(1);

            let mut u = self.0;
            let mut v = MODULUS;
            let mut b = Fq(R2); // Avoids unnecessary reduction step.
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
    fn mul_assign(&mut self, other: &Fq)
    {
        let mut carry = 0;
        let r0 = ::mac_with_carry(0, (self.0).0[0], (other.0).0[0], &mut carry);
        let r1 = ::mac_with_carry(0, (self.0).0[0], (other.0).0[1], &mut carry);
        let r2 = ::mac_with_carry(0, (self.0).0[0], (other.0).0[2], &mut carry);
        let r3 = ::mac_with_carry(0, (self.0).0[0], (other.0).0[3], &mut carry);
        let r4 = ::mac_with_carry(0, (self.0).0[0], (other.0).0[4], &mut carry);
        let r5 = ::mac_with_carry(0, (self.0).0[0], (other.0).0[5], &mut carry);
        let r6 = carry;
        let mut carry = 0;
        let r1 = ::mac_with_carry(r1, (self.0).0[1], (other.0).0[0], &mut carry);
        let r2 = ::mac_with_carry(r2, (self.0).0[1], (other.0).0[1], &mut carry);
        let r3 = ::mac_with_carry(r3, (self.0).0[1], (other.0).0[2], &mut carry);
        let r4 = ::mac_with_carry(r4, (self.0).0[1], (other.0).0[3], &mut carry);
        let r5 = ::mac_with_carry(r5, (self.0).0[1], (other.0).0[4], &mut carry);
        let r6 = ::mac_with_carry(r6, (self.0).0[1], (other.0).0[5], &mut carry);
        let r7 = carry;
        let mut carry = 0;
        let r2 = ::mac_with_carry(r2, (self.0).0[2], (other.0).0[0], &mut carry);
        let r3 = ::mac_with_carry(r3, (self.0).0[2], (other.0).0[1], &mut carry);
        let r4 = ::mac_with_carry(r4, (self.0).0[2], (other.0).0[2], &mut carry);
        let r5 = ::mac_with_carry(r5, (self.0).0[2], (other.0).0[3], &mut carry);
        let r6 = ::mac_with_carry(r6, (self.0).0[2], (other.0).0[4], &mut carry);
        let r7 = ::mac_with_carry(r7, (self.0).0[2], (other.0).0[5], &mut carry);
        let r8 = carry;
        let mut carry = 0;
        let r3 = ::mac_with_carry(r3, (self.0).0[3], (other.0).0[0], &mut carry);
        let r4 = ::mac_with_carry(r4, (self.0).0[3], (other.0).0[1], &mut carry);
        let r5 = ::mac_with_carry(r5, (self.0).0[3], (other.0).0[2], &mut carry);
        let r6 = ::mac_with_carry(r6, (self.0).0[3], (other.0).0[3], &mut carry);
        let r7 = ::mac_with_carry(r7, (self.0).0[3], (other.0).0[4], &mut carry);
        let r8 = ::mac_with_carry(r8, (self.0).0[3], (other.0).0[5], &mut carry);
        let r9 = carry;
        let mut carry = 0;
        let r4 = ::mac_with_carry(r4, (self.0).0[4], (other.0).0[0], &mut carry);
        let r5 = ::mac_with_carry(r5, (self.0).0[4], (other.0).0[1], &mut carry);
        let r6 = ::mac_with_carry(r6, (self.0).0[4], (other.0).0[2], &mut carry);
        let r7 = ::mac_with_carry(r7, (self.0).0[4], (other.0).0[3], &mut carry);
        let r8 = ::mac_with_carry(r8, (self.0).0[4], (other.0).0[4], &mut carry);
        let r9 = ::mac_with_carry(r9, (self.0).0[4], (other.0).0[5], &mut carry);
        let r10 = carry;
        let mut carry = 0;
        let r5 = ::mac_with_carry(r5, (self.0).0[5], (other.0).0[0], &mut carry);
        let r6 = ::mac_with_carry(r6, (self.0).0[5], (other.0).0[1], &mut carry);
        let r7 = ::mac_with_carry(r7, (self.0).0[5], (other.0).0[2], &mut carry);
        let r8 = ::mac_with_carry(r8, (self.0).0[5], (other.0).0[3], &mut carry);
        let r9 = ::mac_with_carry(r9, (self.0).0[5], (other.0).0[4], &mut carry);
        let r10 = ::mac_with_carry(r10, (self.0).0[5], (other.0).0[5], &mut carry);
        let r11 = carry;
        self.mont_reduce(r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11);
    }

    #[inline]
    fn square(&mut self)
    {
        let mut carry = 0;
        let r1 = ::mac_with_carry(0, (self.0).0[0], (self.0).0[1], &mut carry);
        let r2 = ::mac_with_carry(0, (self.0).0[0], (self.0).0[2], &mut carry);
        let r3 = ::mac_with_carry(0, (self.0).0[0], (self.0).0[3], &mut carry);
        let r4 = ::mac_with_carry(0, (self.0).0[0], (self.0).0[4], &mut carry);
        let r5 = ::mac_with_carry(0, (self.0).0[0], (self.0).0[5], &mut carry);
        let r6 = carry;
        let mut carry = 0;
        let r3 = ::mac_with_carry(r3, (self.0).0[1], (self.0).0[2], &mut carry);
        let r4 = ::mac_with_carry(r4, (self.0).0[1], (self.0).0[3], &mut carry);
        let r5 = ::mac_with_carry(r5, (self.0).0[1], (self.0).0[4], &mut carry);
        let r6 = ::mac_with_carry(r6, (self.0).0[1], (self.0).0[5], &mut carry);
        let r7 = carry;
        let mut carry = 0;
        let r5 = ::mac_with_carry(r5, (self.0).0[2], (self.0).0[3], &mut carry);
        let r6 = ::mac_with_carry(r6, (self.0).0[2], (self.0).0[4], &mut carry);
        let r7 = ::mac_with_carry(r7, (self.0).0[2], (self.0).0[5], &mut carry);
        let r8 = carry;
        let mut carry = 0;
        let r7 = ::mac_with_carry(r7, (self.0).0[3], (self.0).0[4], &mut carry);
        let r8 = ::mac_with_carry(r8, (self.0).0[3], (self.0).0[5], &mut carry);
        let r9 = carry;
        let mut carry = 0;
        let r9 = ::mac_with_carry(r9, (self.0).0[4], (self.0).0[5], &mut carry);
        let r10 = carry;

        let r11 = r10 >> 63;
        let r10 = (r10 << 1) | (r9 >> 63);
        let r9 = (r9 << 1) | (r8 >> 63);
        let r8 = (r8 << 1) | (r7 >> 63);
        let r7 = (r7 << 1) | (r6 >> 63);
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
        let r8 = ::mac_with_carry(r8, (self.0).0[4], (self.0).0[4], &mut carry);
        let r9 = ::adc(r9, 0, &mut carry);
        let r10 = ::mac_with_carry(r10, (self.0).0[5], (self.0).0[5], &mut carry);
        let r11 = ::adc(r11, 0, &mut carry);
        self.mont_reduce(r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11);
    }
}

impl Fq {
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
        mut r7: u64,
        mut r8: u64,
        mut r9: u64,
        mut r10: u64,
        mut r11: u64
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
        r4 = ::mac_with_carry(r4, k, MODULUS.0[4], &mut carry);
        r5 = ::mac_with_carry(r5, k, MODULUS.0[5], &mut carry);
        r6 = ::adc(r6, 0, &mut carry);
        let carry2 = carry;
        let k = r1.wrapping_mul(INV);
        let mut carry = 0;
        ::mac_with_carry(r1, k, MODULUS.0[0], &mut carry);
        r2 = ::mac_with_carry(r2, k, MODULUS.0[1], &mut carry);
        r3 = ::mac_with_carry(r3, k, MODULUS.0[2], &mut carry);
        r4 = ::mac_with_carry(r4, k, MODULUS.0[3], &mut carry);
        r5 = ::mac_with_carry(r5, k, MODULUS.0[4], &mut carry);
        r6 = ::mac_with_carry(r6, k, MODULUS.0[5], &mut carry);
        r7 = ::adc(r7, carry2, &mut carry);
        let carry2 = carry;
        let k = r2.wrapping_mul(INV);
        let mut carry = 0;
        ::mac_with_carry(r2, k, MODULUS.0[0], &mut carry);
        r3 = ::mac_with_carry(r3, k, MODULUS.0[1], &mut carry);
        r4 = ::mac_with_carry(r4, k, MODULUS.0[2], &mut carry);
        r5 = ::mac_with_carry(r5, k, MODULUS.0[3], &mut carry);
        r6 = ::mac_with_carry(r6, k, MODULUS.0[4], &mut carry);
        r7 = ::mac_with_carry(r7, k, MODULUS.0[5], &mut carry);
        r8 = ::adc(r8, carry2, &mut carry);
        let carry2 = carry;
        let k = r3.wrapping_mul(INV);
        let mut carry = 0;
        ::mac_with_carry(r3, k, MODULUS.0[0], &mut carry);
        r4 = ::mac_with_carry(r4, k, MODULUS.0[1], &mut carry);
        r5 = ::mac_with_carry(r5, k, MODULUS.0[2], &mut carry);
        r6 = ::mac_with_carry(r6, k, MODULUS.0[3], &mut carry);
        r7 = ::mac_with_carry(r7, k, MODULUS.0[4], &mut carry);
        r8 = ::mac_with_carry(r8, k, MODULUS.0[5], &mut carry);
        r9 = ::adc(r9, carry2, &mut carry);
        let carry2 = carry;
        let k = r4.wrapping_mul(INV);
        let mut carry = 0;
        ::mac_with_carry(r4, k, MODULUS.0[0], &mut carry);
        r5 = ::mac_with_carry(r5, k, MODULUS.0[1], &mut carry);
        r6 = ::mac_with_carry(r6, k, MODULUS.0[2], &mut carry);
        r7 = ::mac_with_carry(r7, k, MODULUS.0[3], &mut carry);
        r8 = ::mac_with_carry(r8, k, MODULUS.0[4], &mut carry);
        r9 = ::mac_with_carry(r9, k, MODULUS.0[5], &mut carry);
        r10 = ::adc(r10, carry2, &mut carry);
        let carry2 = carry;
        let k = r5.wrapping_mul(INV);
        let mut carry = 0;
        ::mac_with_carry(r5, k, MODULUS.0[0], &mut carry);
        r6 = ::mac_with_carry(r6, k, MODULUS.0[1], &mut carry);
        r7 = ::mac_with_carry(r7, k, MODULUS.0[2], &mut carry);
        r8 = ::mac_with_carry(r8, k, MODULUS.0[3], &mut carry);
        r9 = ::mac_with_carry(r9, k, MODULUS.0[4], &mut carry);
        r10 = ::mac_with_carry(r10, k, MODULUS.0[5], &mut carry);
        r11 = ::adc(r11, carry2, &mut carry);
        (self.0).0[0] = r6;
        (self.0).0[1] = r7;
        (self.0).0[2] = r8;
        (self.0).0[3] = r9;
        (self.0).0[4] = r10;
        (self.0).0[5] = r11;
        self.reduce();
    }
}

impl SqrtField for Fq {
    fn sqrt(&self) -> Option<Self> {
        // Shank's algorithm for q mod 4 = 3
        // https://eprint.iacr.org/2012/685.pdf (page 9, algorithm 2)

        // a1 = self^((q - 3) // 2)
        let mut a1 = self.pow([0xee7fbfffffffeaaa, 0x7aaffffac54ffff, 0xd9cc34a83dac3d89, 0xd91dd2e13ce144af, 0x92c6e9ed90d2eb35, 0x680447a8e5ff9a6]);
        let mut a0 = a1;
        a0.square();
        a0.mul_assign(self);

        if a0 == NEGATIVE_ONE
        {
            None
        }
        else
        {
            a1.mul_assign(self);
            Some(a1)
        }
    }
}

#[test]
fn test_b_coeff() {
    assert_eq!(Fq::from_repr(FqRepr::from(4)).unwrap(), B_COEFF);
}

#[test]
fn test_frob_coeffs() {
    let mut nqr = Fq::one();
    nqr.negate();

    assert_eq!(FROBENIUS_COEFF_FQ2_C1[0], Fq::one());
    assert_eq!(FROBENIUS_COEFF_FQ2_C1[1], nqr.pow([0xdcff7fffffffd555, 0xf55ffff58a9ffff, 0xb39869507b587b12, 0xb23ba5c279c2895f, 0x258dd3db21a5d66b, 0xd0088f51cbff34d]));

    let nqr = Fq2 {
        c0: Fq::one(),
        c1: Fq::one()
    };

    assert_eq!(FROBENIUS_COEFF_FQ6_C1[0], Fq2::one());
    assert_eq!(FROBENIUS_COEFF_FQ6_C1[1], nqr.pow([0x9354ffffffffe38e, 0xa395554e5c6aaaa, 0xcd104635a790520c, 0xcc27c3d6fbd7063f, 0x190937e76bc3e447, 0x8ab05f8bdd54cde]));
    assert_eq!(FROBENIUS_COEFF_FQ6_C1[2], nqr.pow([0xb78e0000097b2f68, 0xd44f23b47cbd64e3, 0x5cb9668120b069a9, 0xccea85f9bf7b3d16, 0xdba2c8d7adb356d, 0x9cd75ded75d7429, 0xfc65c31103284fab, 0xc58cb9a9b249ee24, 0xccf734c3118a2e9a, 0xa0f4304c5a256ce6, 0xc3f0d2f8e0ba61f8, 0xe167e192ebca97]));
    assert_eq!(FROBENIUS_COEFF_FQ6_C1[3], nqr.pow([0xdbc6fcd6f35b9e06, 0x997dead10becd6aa, 0x9dbbd24c17206460, 0x72b97acc6057c45e, 0xf8e9a230bf0c628e, 0x647ccb1885c63a7, 0xce80264fc55bf6ee, 0x94d8d716c3939fc4, 0xad78f0eb77ee6ee1, 0xd6fe49bfe57dc5f9, 0x2656d6c15c63647, 0xdf6282f111fa903, 0x1bdba63e0632b4bb, 0x6883597bcaa505eb, 0xa56d4ec90c34a982, 0x7e4c42823bbe90b2, 0xf64728aa6dcb0f20, 0x16e57e16ef152f]));
    assert_eq!(FROBENIUS_COEFF_FQ6_C1[4], nqr.pow([0x4649add3c71c6d90, 0x43caa6528972a865, 0xcda8445bbaaa0fbb, 0xc93dea665662aa66, 0x2863bc891834481d, 0x51a0c3f5d4ccbed8, 0x9210e660f90ccae9, 0xe2bd6836c546d65e, 0xf223abbaa7cf778b, 0xd4f10b222cf11680, 0xd540f5eff4a1962e, 0xa123a1f140b56526, 0x31ace500636a59f6, 0x3a82bc8c8dfa57a9, 0x648c511e217fc1f8, 0x36c17ffd53a4558f, 0x881bef5fd684eefd, 0x5d648dbdc5dbb522, 0x8fd07bf06e5e59b8, 0x8ddec8a9acaa4b51, 0x4cc1f8688e2def26, 0xa74e63cb492c03de, 0x57c968173d1349bb, 0x253674e02a866]));
    assert_eq!(FROBENIUS_COEFF_FQ6_C1[5], nqr.pow([0xf896f792732eb2be, 0x49c86a6d1dc593a1, 0xe5b31e94581f91c3, 0xe3da5cc0a6b20d7f, 0x822caef950e0bfed, 0x317ed950b9ee67cd, 0xffd664016ee3f6cd, 0x77d991c88810b122, 0x62e72e635e698264, 0x905e1a1a2d22814a, 0xf5b7ab3a3f33d981, 0x175871b0bc0e25dd, 0x1e2e9a63df5c3772, 0xe888b1f7445b149d, 0x9551c19e5e7e2c24, 0xecf21939a3d2d6be, 0xd830dbfdab72dbd4, 0x7b34af8d622d40c0, 0x3df6d20a45671242, 0xaf86bee30e21d98, 0x41064c1534e5df5d, 0xf5f6cabd3164c609, 0xa5d14bdf2b7ee65, 0xa718c069defc9138, 0xdb1447e770e3110e, 0xc1b164a9e90af491, 0x7180441f9d251602, 0x1fd3a5e6a9a893e, 0x1e17b779d54d5db, 0x3c7afafe3174]));

    assert_eq!(FROBENIUS_COEFF_FQ6_C2[0], Fq2::one());
    assert_eq!(FROBENIUS_COEFF_FQ6_C2[1], nqr.pow([0x26a9ffffffffc71c, 0x1472aaa9cb8d5555, 0x9a208c6b4f20a418, 0x984f87adf7ae0c7f, 0x32126fced787c88f, 0x11560bf17baa99bc]));
    assert_eq!(FROBENIUS_COEFF_FQ6_C2[2], nqr.pow([0x6f1c000012f65ed0, 0xa89e4768f97ac9c7, 0xb972cd024160d353, 0x99d50bf37ef67a2c, 0x1b74591af5b66adb, 0x139aebbdaebae852, 0xf8cb862206509f56, 0x8b1973536493dc49, 0x99ee698623145d35, 0x41e86098b44ad9cd, 0x87e1a5f1c174c3f1, 0x1c2cfc325d7952f]));
    assert_eq!(FROBENIUS_COEFF_FQ6_C2[3], nqr.pow([0xb78df9ade6b73c0c, 0x32fbd5a217d9ad55, 0x3b77a4982e40c8c1, 0xe572f598c0af88bd, 0xf1d344617e18c51c, 0xc8f996310b8c74f, 0x9d004c9f8ab7eddc, 0x29b1ae2d87273f89, 0x5af1e1d6efdcddc3, 0xadfc937fcafb8bf3, 0x4cadad82b8c6c8f, 0x1bec505e223f5206, 0x37b74c7c0c656976, 0xd106b2f7954a0bd6, 0x4ada9d9218695304, 0xfc988504777d2165, 0xec8e5154db961e40, 0x2dcafc2dde2a5f]));
    assert_eq!(FROBENIUS_COEFF_FQ6_C2[4], nqr.pow([0x8c935ba78e38db20, 0x87954ca512e550ca, 0x9b5088b775541f76, 0x927bd4ccacc554cd, 0x50c779123068903b, 0xa34187eba9997db0, 0x2421ccc1f21995d2, 0xc57ad06d8a8dacbd, 0xe44757754f9eef17, 0xa9e2164459e22d01, 0xaa81ebdfe9432c5d, 0x424743e2816aca4d, 0x6359ca00c6d4b3ed, 0x750579191bf4af52, 0xc918a23c42ff83f0, 0x6d82fffaa748ab1e, 0x1037debfad09ddfa, 0xbac91b7b8bb76a45, 0x1fa0f7e0dcbcb370, 0x1bbd9153595496a3, 0x9983f0d11c5bde4d, 0x4e9cc796925807bc, 0xaf92d02e7a269377, 0x4a6ce9c0550cc]));
    assert_eq!(FROBENIUS_COEFF_FQ6_C2[5], nqr.pow([0xf12def24e65d657c, 0x9390d4da3b8b2743, 0xcb663d28b03f2386, 0xc7b4b9814d641aff, 0x4595df2a1c17fdb, 0x62fdb2a173dccf9b, 0xffacc802ddc7ed9a, 0xefb3239110216245, 0xc5ce5cc6bcd304c8, 0x20bc34345a450294, 0xeb6f56747e67b303, 0x2eb0e361781c4bbb, 0x3c5d34c7beb86ee4, 0xd11163ee88b6293a, 0x2aa3833cbcfc5849, 0xd9e4327347a5ad7d, 0xb061b7fb56e5b7a9, 0xf6695f1ac45a8181, 0x7beda4148ace2484, 0x15f0d7dc61c43b30, 0x820c982a69cbbeba, 0xebed957a62c98c12, 0x14ba297be56fdccb, 0x4e3180d3bdf92270, 0xb6288fcee1c6221d, 0x8362c953d215e923, 0xe300883f3a4a2c05, 0x3fa74bcd535127c, 0x3c2f6ef3aa9abb6, 0x78f5f5fc62e8]));

    assert_eq!(FROBENIUS_COEFF_FQ12_C1[0], Fq2::one());
    assert_eq!(FROBENIUS_COEFF_FQ12_C1[1], nqr.pow([0x49aa7ffffffff1c7, 0x51caaaa72e35555, 0xe688231ad3c82906, 0xe613e1eb7deb831f, 0xc849bf3b5e1f223, 0x45582fc5eeaa66f]));
    assert_eq!(FROBENIUS_COEFF_FQ12_C1[2], nqr.pow([0xdbc7000004bd97b4, 0xea2791da3e5eb271, 0x2e5cb340905834d4, 0xe67542fcdfbd9e8b, 0x86dd1646bd6d9ab6, 0x84e6baef6baeba14, 0x7e32e188819427d5, 0x62c65cd4d924f712, 0x667b9a6188c5174d, 0x507a18262d12b673, 0xe1f8697c705d30fc, 0x70b3f0c975e54b]));
    assert_eq!(FROBENIUS_COEFF_FQ12_C1[3], nqr.pow(vec![0x6de37e6b79adcf03, 0x4cbef56885f66b55, 0x4edde9260b903230, 0x395cbd66302be22f, 0xfc74d1185f863147, 0x323e658c42e31d3, 0x67401327e2adfb77, 0xca6c6b8b61c9cfe2, 0xd6bc7875bbf73770, 0xeb7f24dff2bee2fc, 0x8132b6b60ae31b23, 0x86fb1417888fd481, 0x8dedd31f03195a5d, 0x3441acbde55282f5, 0x52b6a764861a54c1, 0x3f2621411ddf4859, 0xfb23945536e58790, 0xb72bf0b778a97]));
    assert_eq!(FROBENIUS_COEFF_FQ12_C1[4], nqr.pow(vec![0xa324d6e9e38e36c8, 0xa1e5532944b95432, 0x66d4222ddd5507dd, 0xe49ef5332b315533, 0x1431de448c1a240e, 0xa8d061faea665f6c, 0x490873307c866574, 0xf15eb41b62a36b2f, 0x7911d5dd53e7bbc5, 0x6a78859116788b40, 0x6aa07af7fa50cb17, 0x5091d0f8a05ab293, 0x98d6728031b52cfb, 0x1d415e4646fd2bd4, 0xb246288f10bfe0fc, 0x9b60bffea9d22ac7, 0x440df7afeb42777e, 0x2eb246dee2edda91, 0xc7e83df8372f2cdc, 0x46ef6454d65525a8, 0x2660fc344716f793, 0xd3a731e5a49601ef, 0x2be4b40b9e89a4dd, 0x129b3a7015433]));
    assert_eq!(FROBENIUS_COEFF_FQ12_C1[5], nqr.pow(vec![0xfc4b7bc93997595f, 0xa4e435368ee2c9d0, 0xf2d98f4a2c0fc8e1, 0xf1ed2e60535906bf, 0xc116577ca8705ff6, 0x98bf6ca85cf733e6, 0x7feb3200b771fb66, 0x3becc8e444085891, 0x31739731af34c132, 0xc82f0d0d169140a5, 0xfadbd59d1f99ecc0, 0xbac38d85e0712ee, 0x8f174d31efae1bb9, 0x744458fba22d8a4e, 0x4aa8e0cf2f3f1612, 0x76790c9cd1e96b5f, 0x6c186dfed5b96dea, 0x3d9a57c6b116a060, 0x1efb690522b38921, 0x857c35f718710ecc, 0xa083260a9a72efae, 0xfafb655e98b26304, 0x52e8a5ef95bf732, 0x538c6034ef7e489c, 0xed8a23f3b8718887, 0x60d8b254f4857a48, 0x38c0220fce928b01, 0x80fe9d2f354d449f, 0xf0bdbbceaa6aed, 0x1e3d7d7f18ba]));
    assert_eq!(FROBENIUS_COEFF_FQ12_C1[6], nqr.pow(vec![0x21219610a012ba3c, 0xa5c19ad35375325, 0x4e9df1e497674396, 0xfb05b717c991c6ef, 0x4a1265bca93a32f2, 0xd875ff2a7bdc1f66, 0xc6d8754736c771b2, 0x2d80c759ba5a2ae7, 0x138a20df4b03cc1a, 0xc22d07fe68e93024, 0xd1dc474d3b433133, 0xc22aa5e75044e5c, 0xf657c6fbf9c17ebf, 0xc591a794a58660d, 0x2261850ee1453281, 0xd17d3bd3b7f5efb4, 0xf00cec8ec507d01, 0x2a6a775657a00ae6, 0x5f098a12ff470719, 0x409d194e7b5c5afa, 0x1d66478e982af5b, 0xda425a5b5e01ca3f, 0xf77e4f78747e903c, 0x177d49f73732c6fc, 0xa9618fecabe0e1f4, 0xba5337eac90bd080, 0x66fececdbc35d4e7, 0xa4cd583203d9206f, 0x98391632ceeca596, 0x4946b76e1236ad3f, 0xa0dec64e60e711a1, 0xfcb41ed3605013, 0x8ca8f9692ae1e3a9, 0xd3078bfc28cc1baf, 0xf0536f764e982f82, 0x3125f1a2656]));
    assert_eq!(FROBENIUS_COEFF_FQ12_C1[7], nqr.pow(vec![0x742754a1f22fdb, 0x2a1955c2dec3a702, 0x9747b28c796d134e, 0xc113a0411f59db79, 0x3bb0fa929853bfc1, 0x28c3c25f8f6fb487, 0xbc2b6c99d3045b34, 0x98fb67d6badde1fd, 0x48841d76a24d2073, 0xd49891145fe93ae6, 0xc772b9c8e74d4099, 0xccf4e7b9907755bb, 0x9cf47b25d42fd908, 0x5616a0c347fc445d, 0xff93b7a7ad1b8a6d, 0xac2099256b78a77a, 0x7804a95b02892e1c, 0x5cf59ca7bfd69776, 0xa7023502acd3c866, 0xc76f4982fcf8f37, 0x51862a5a57ac986e, 0x38b80ed72b1b1023, 0x4a291812066a61e1, 0xcd8a685eff45631, 0x3f40f708764e4fa5, 0x8aa0441891285092, 0x9eff60d71cdf0a9, 0x4fdd9d56517e2bfa, 0x1f3c80d74a28bc85, 0x24617417c064b648, 0x7ddda1e4385d5088, 0xf9e132b11dd32a16, 0xcc957cb8ef66ab99, 0xd4f206d37cb752c5, 0x40de343f28ad616b, 0x8d1f24379068f0e3, 0x6f31d7947ea21137, 0x27311f9c32184061, 0x9eea0664cc78ce5f, 0x7d4151f6fea9a0da, 0x454096fa75bd571a, 0x4fe0f20ecb]));
    assert_eq!(FROBENIUS_COEFF_FQ12_C1[8], nqr.pow(vec![0x802f5720d0b25710, 0x6714f0a258b85c7c, 0x31394c90afdf16e, 0xe9d2b0c64f957b19, 0xe67c0d9c5e7903ee, 0x3156fdc5443ea8ef, 0x7c4c50524d88c892, 0xc99dc8990c0ad244, 0xd37ababf3649a896, 0x76fe4b838ff7a20c, 0xcf69ee2cec728db3, 0xb83535548e5f41, 0x371147684ccb0c23, 0x194f6f4fa500db52, 0xc4571dc78a4c5374, 0xe4d46d479999ca97, 0x76b6785a615a151c, 0xcceb8bcea7eaf8c1, 0x80d87a6fbe5ae687, 0x6a97ddddb85ce85, 0xd783958f26034204, 0x7144506f2e2e8590, 0x948693d377aef166, 0x8364621ed6f96056, 0xf021777c4c09ee2d, 0xc6cf5e746ecd50b, 0xa2337b7aa22743df, 0xae753f8bbacab39c, 0xfc782a9e34d3c1cc, 0x21b827324fe494d9, 0x5692ce350ed03b38, 0xf323a2b3cd0481b0, 0xe859c97a4ccad2e3, 0x48434b70381e4503, 0x46042d62e4132ed8, 0x48c4d6f56122e2f2, 0xf87711ab9f5c1af7, 0xb14b7a054759b469, 0x8eb0a96993ffa9aa, 0x9b21fb6fc58b760c, 0xf3abdd115d2e7d25, 0xf7beac3d4d12409c, 0x40a5585cce69bf03, 0x697881e1ba22d5a8, 0x3d6c04e6ad373fd9, 0x849871bf627be886, 0x550f4b9b71b28ef9, 0x81d2e0d78]));
    assert_eq!(FROBENIUS_COEFF_FQ12_C1[9], nqr.pow(vec![0x4af4accf7de0b977, 0x742485e21805b4ee, 0xee388fbc4ac36dec, 0x1e199da57ad178a, 0xc27c12b292c6726a, 0x162e6ed84505b5e8, 0xe191683f336e09df, 0x17deb7e8d1e0fce6, 0xd944f19ad06f5836, 0x4c5f5e59f6276026, 0xf1ba9c7c148a38a8, 0xd205fe2dba72b326, 0x9a2cf2a4c289824e, 0x4f47ad512c39e24d, 0xc5894d984000ea09, 0x2974c03ff7cf01fa, 0xfcd243b48cb99a22, 0x2b5150c9313ac1e8, 0x9089f37c7fc80eda, 0x989540cc9a7aea56, 0x1ab1d4e337e63018, 0x42b546c30d357e43, 0x1c6abc04f76233d9, 0x78b3b8d88bf73e47, 0x151c4e4c45dc68e6, 0x519a79c4f54397ed, 0x93f5b51535a127c5, 0x5fc51b6f52fa153e, 0x2e0504f2d4a965c3, 0xc85bd3a3da52bffe, 0x98c60957a46a89ef, 0x48c03b5976b91cae, 0xc6598040a0a61438, 0xbf0b49dc255953af, 0xb78dff905b628ab4, 0x68140b797ba74ab8, 0x116cf037991d1143, 0x2f7fe82e58acb0b8, 0xc20bf7a8f7be5d45, 0x86c2905c338d5709, 0xff13a3ae6c8ace3d, 0xb6f95e2282d08337, 0xd49f7b313e9cbf29, 0xf794517193a1ce8c, 0x39641fecb596a874, 0x411c4c4edf462fb3, 0x3f8cd55c10cf25b4, 0x2bdd7ea165e860b6, 0xacd7d2cef4caa193, 0x6558a1d09a05f96, 0x1f52b5f5b546fc20, 0x4ee22a5a8c250c12, 0xd3a63a54a205b6b3, 0xd2ff5be8]));
    assert_eq!(FROBENIUS_COEFF_FQ12_C1[10], nqr.pow(vec![0xe5953a4f96cdda44, 0x336b2d734cbc32bb, 0x3f79bfe3cd7410e, 0x267ae19aaa0f0332, 0x85a9c4db78d5c749, 0x90996b046b5dc7d8, 0x8945eae9820afc6a, 0x2644ddea2b036bd, 0x39898e35ac2e3819, 0x2574eab095659ab9, 0x65953d51ac5ea798, 0xc6b8c7afe6752466, 0x40e9e993e9286544, 0x7e0ad34ad9700ea0, 0xac1015eba2c69222, 0x24f057a19239b5d8, 0x2043b48c8a3767eb, 0x1117c124a75d7ff4, 0x433cfd1a09fb3ce7, 0x25b087ce4bcf7fb, 0xbcee0dc53a3e5bdb, 0xbffda040cf028735, 0xf7cf103a25512acc, 0x31d4ecda673130b9, 0xea0906dab18461e6, 0x5a40585a5ac3050d, 0x803358fc14fd0eda, 0x3678ca654eada770, 0x7b91a1293a45e33e, 0xcd5e5b8ea8530e43, 0x21ae563ab34da266, 0xecb00dad60df8894, 0x77fe53e652facfef, 0x9b7d1ad0b00244ec, 0xe695df5ca73f801, 0x23cdb21feeab0149, 0x14de113e7ea810d9, 0x52600cd958dac7e7, 0xc83392c14667e488, 0x9f808444bc1717fc, 0x56facb4bcf7c788f, 0x8bcad53245fc3ca0, 0xdef661e83f27d81c, 0x37d4ebcac9ad87e5, 0x6fe8b24f5cdb9324, 0xee08a26c1197654c, 0xc98b22f65f237e9a, 0xf54873a908ed3401, 0x6e1cb951d41f3f3, 0x290b2250a54e8df6, 0x7f36d51eb1db669e, 0xb08c7ed81a6ee43e, 0x95e1c90fb092f680, 0x429e4afd0e8b820, 0x2c14a83ee87d715c, 0xf37267575cfc8af5, 0xb99e9afeda3c2c30, 0x8f0f69da75792d5a, 0x35074a85a533c73, 0x156ed119]));
    assert_eq!(FROBENIUS_COEFF_FQ12_C1[11], nqr.pow(vec![0x107db680942de533, 0x6262b24d2052393b, 0x6136df824159ebc, 0xedb052c9970c5deb, 0xca813aea916c3777, 0xf49dacb9d76c1788, 0x624941bd372933bb, 0xa5e60c2520638331, 0xb38b661683411074, 0x1d2c9af4c43d962b, 0x17d807a0f14aa830, 0x6e6581a51012c108, 0x668a537e5b35e6f5, 0x6c396cf3782dca5d, 0x33b679d1bff536ed, 0x736cce41805d90aa, 0x8a562f369eb680bf, 0x9f61aa208a11ded8, 0x43dd89dd94d20f35, 0xcf84c6610575c10a, 0x9f318d49cf2fe8e6, 0xbbc6e5f25a6e434e, 0x6528c433d11d987b, 0xffced71cc48c0e8a, 0x4cbb1474f4cb2a26, 0x66a035c0b28b7231, 0xa6f2875faa1a82ae, 0xdd1ea3deff818b02, 0xe0cfdf0dcdecf701, 0x9aefa231f2f6d23, 0xfb251297efa06746, 0x5a40d367df985538, 0x1ea31d69ab506fed, 0xc64ea8280e89a73f, 0x969acf9f2d4496f4, 0xe84c9181ee60c52c, 0xc60f27fc19fc6ca4, 0x760b33d850154048, 0x84f69080f66c8457, 0xc0192ba0fabf640e, 0xd2c338765c23a3a8, 0xa7838c20f02cec6c, 0xb7cf01d020572877, 0xd63ffaeba0be200a, 0xf7492baeb5f041ac, 0x8602c5212170d117, 0xad9b2e83a5a42068, 0x2461829b3ba1083e, 0x7c34650da5295273, 0xdc824ba800a8265a, 0xd18d9b47836af7b2, 0x3af78945c58cbf4d, 0x7ed9575b8596906c, 0x6d0c133895009a66, 0x53bc1247ea349fe1, 0x6b3063078d41aa7a, 0x6184acd8cd880b33, 0x76f4d15503fd1b96, 0x7a9afd61eef25746, 0xce974aadece60609, 0x88ca59546a8ceafd, 0x6d29391c41a0ac07, 0x443843a60e0f46a6, 0xa1590f62fd2602c7, 0x536d5b15b514373f, 0x22d582b]));
}

#[test]
fn test_neg_one() {
    let mut o = Fq::one();
    o.negate();

    assert_eq!(NEGATIVE_ONE, o);
}

#[cfg(test)]
use rand::{SeedableRng, XorShiftRng, Rand};

#[test]
fn test_fq_repr_ordering() {
    fn assert_equality(a: FqRepr, b: FqRepr) {
        assert_eq!(a, b);
        assert!(a.cmp(&b) == Ordering::Equal);
    }

    fn assert_lt(a: FqRepr, b: FqRepr) {
        assert!(a < b);
        assert!(b > a);
    }

    assert_equality(FqRepr([9999, 9999, 9999, 9999, 9999, 9999]), FqRepr([9999, 9999, 9999, 9999, 9999, 9999]));
    assert_equality(FqRepr([9999, 9998, 9999, 9999, 9999, 9999]), FqRepr([9999, 9998, 9999, 9999, 9999, 9999]));
    assert_equality(FqRepr([9999, 9999, 9999, 9997, 9999, 9999]), FqRepr([9999, 9999, 9999, 9997, 9999, 9999]));
    assert_lt(FqRepr([9999, 9999, 9999, 9997, 9999, 9998]), FqRepr([9999, 9999, 9999, 9997, 9999, 9999]));
    assert_lt(FqRepr([9999, 9999, 9999, 9997, 9998, 9999]), FqRepr([9999, 9999, 9999, 9997, 9999, 9999]));
    assert_lt(FqRepr([9, 9999, 9999, 9997, 9998, 9999]), FqRepr([9999, 9999, 9999, 9997, 9999, 9999]));
}

#[test]
fn test_fq_repr_from() {
    assert_eq!(FqRepr::from(100), FqRepr([100, 0, 0, 0, 0, 0]));
}

#[test]
fn test_fq_repr_is_odd() {
    assert!(!FqRepr::from(0).is_odd());
    assert!(FqRepr::from(0).is_even());
    assert!(FqRepr::from(1).is_odd());
    assert!(!FqRepr::from(1).is_even());
    assert!(!FqRepr::from(324834872).is_odd());
    assert!(FqRepr::from(324834872).is_even());
    assert!(FqRepr::from(324834873).is_odd());
    assert!(!FqRepr::from(324834873).is_even());
}

#[test]
fn test_fq_repr_is_zero() {
    assert!(FqRepr::from(0).is_zero());
    assert!(!FqRepr::from(1).is_zero());
    assert!(!FqRepr([0, 0, 0, 0, 1, 0]).is_zero());
}

#[test]
fn test_fq_repr_div2() {
    let mut a = FqRepr([0x8b0ad39f8dd7482a, 0x147221c9a7178b69, 0x54764cb08d8a6aa0, 0x8519d708e1d83041, 0x41f82777bd13fdb, 0xf43944578f9b771b]);
    a.div2();
    assert_eq!(a, FqRepr([0xc58569cfc6eba415, 0xa3910e4d38bc5b4, 0xaa3b265846c53550, 0xc28ceb8470ec1820, 0x820fc13bbde89fed, 0x7a1ca22bc7cdbb8d]));
    for _ in 0..10 {
        a.div2();
    }
    assert_eq!(a, FqRepr([0x6d31615a73f1bae9, 0x54028e443934e2f1, 0x82a8ec99611b14d, 0xfb70a33ae11c3b06, 0xe36083f04eef7a27, 0x1e87288af1f36e]));
    for _ in 0..300 {
        a.div2();
    }
    assert_eq!(a, FqRepr([0x7288af1f36ee3608, 0x1e8, 0x0, 0x0, 0x0, 0x0]));
    for _ in 0..50 {
        a.div2();
    }
    assert_eq!(a, FqRepr([0x7a1ca2, 0x0, 0x0, 0x0, 0x0, 0x0]));
    for _ in 0..22 {
        a.div2();
    }
    assert_eq!(a, FqRepr([0x1, 0x0, 0x0, 0x0, 0x0, 0x0]));
    a.div2();
    assert!(a.is_zero());
}

#[test]
fn test_fq_repr_divn() {
    let mut a = FqRepr([0xaa5cdd6172847ffd, 0x43242c06aed55287, 0x9ddd5b312f3dd104, 0xc5541fd48046b7e7, 0x16080cf4071e0b05, 0x1225f2901aea514e]);
    a.divn(0);
    assert_eq!(
        a,
        FqRepr([0xaa5cdd6172847ffd, 0x43242c06aed55287, 0x9ddd5b312f3dd104, 0xc5541fd48046b7e7, 0x16080cf4071e0b05, 0x1225f2901aea514e])
    );
    a.divn(1);
    assert_eq!(
        a,
        FqRepr([0xd52e6eb0b9423ffe, 0x21921603576aa943, 0xceeead98979ee882, 0xe2aa0fea40235bf3, 0xb04067a038f0582, 0x912f9480d7528a7])
    );
    a.divn(50);
    assert_eq!(
        a,
        FqRepr([0x8580d5daaa50f54b, 0xab6625e7ba208864, 0x83fa9008d6fcf3bb, 0x19e80e3c160b8aa, 0xbe52035d4a29c2c1, 0x244])
    );
    a.divn(130);
    assert_eq!(
        a,
        FqRepr([0xa0fea40235bf3cee, 0x4067a038f0582e2a, 0x2f9480d7528a70b0, 0x91, 0x0, 0x0])
    );
    a.divn(64);
    assert_eq!(
        a,
        FqRepr([0x4067a038f0582e2a, 0x2f9480d7528a70b0, 0x91, 0x0, 0x0, 0x0])
    );
}

#[test]
fn test_fq_repr_mul2() {
    let mut a = FqRepr::from(23712937547);
    a.mul2();
    assert_eq!(a, FqRepr([0xb0acd6c96, 0x0, 0x0, 0x0, 0x0, 0x0]));
    for _ in 0..60 {
        a.mul2();
    }
    assert_eq!(a, FqRepr([0x6000000000000000, 0xb0acd6c9, 0x0, 0x0, 0x0, 0x0]));
    for _ in 0..300 {
        a.mul2();
    }
    assert_eq!(a, FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0xcd6c960000000000]));
    for _ in 0..17 {
        a.mul2();
    }
    assert_eq!(a, FqRepr([0x0, 0x0, 0x0, 0x0, 0x0, 0x2c00000000000000]));
    for _ in 0..6 {
        a.mul2();
    }
    assert!(a.is_zero());
}

#[test]
fn test_fq_repr_num_bits() {
    let mut a = FqRepr::from(0);
    assert_eq!(0, a.num_bits());
    a = FqRepr::from(1);
    for i in 1..385 {
        assert_eq!(i, a.num_bits());
        a.mul2();
    }
    assert_eq!(0, a.num_bits());
}

#[test]
fn test_fq_repr_sub_noborrow() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let mut t = FqRepr([0x827a4a08041ebd9, 0x3c239f3dcc8f0d6b, 0x9ab46a912d555364, 0x196936b17b43910b, 0xad0eb3948a5c34fd, 0xd56f7b5ab8b5ce8]);
    t.sub_noborrow(&FqRepr([0xc7867917187ca02b, 0x5d75679d4911ffef, 0x8c5b3e48b1a71c15, 0x6a427ae846fd66aa, 0x7a37e7265ee1eaf9, 0x7c0577a26f59d5]));
    assert!(t == FqRepr([0x40a12b8967c54bae, 0xdeae37a0837d0d7b, 0xe592c487bae374e, 0xaf26bbc934462a61, 0x32d6cc6e2b7a4a03, 0xcdaf23e091c0313]));

    for _ in 0..1000 {
        let mut a = FqRepr::rand(&mut rng);
        a.0[5] >>= 30;
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

    // Subtracting q+1 from q should produce a borrow
    let mut qplusone = FqRepr([0xb9feffffffffaaab, 0x1eabfffeb153ffff, 0x6730d2a0f6b0f624, 0x64774b84f38512bf, 0x4b1ba7b6434bacd7, 0x1a0111ea397fe69a]);
    assert!(qplusone.sub_noborrow(&FqRepr([0xb9feffffffffaaac, 0x1eabfffeb153ffff, 0x6730d2a0f6b0f624, 0x64774b84f38512bf, 0x4b1ba7b6434bacd7, 0x1a0111ea397fe69a])));

    // Subtracting x from x should produce no borrow
    let mut x = FqRepr([0xb9feffffffffaaac, 0x1eabfffeb153ffff, 0x6730d2a0f6b0f624, 0x64774b84f38512bf, 0x4b1ba7b6434bacd7, 0x1a0111ea397fe69a]);
    assert!(!x.sub_noborrow(&FqRepr([0xb9feffffffffaaac, 0x1eabfffeb153ffff, 0x6730d2a0f6b0f624, 0x64774b84f38512bf, 0x4b1ba7b6434bacd7, 0x1a0111ea397fe69a])))
}

#[test]
fn test_fq_repr_add_nocarry() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let mut t = FqRepr([0x827a4a08041ebd9, 0x3c239f3dcc8f0d6b, 0x9ab46a912d555364, 0x196936b17b43910b, 0xad0eb3948a5c34fd, 0xd56f7b5ab8b5ce8]);
    t.add_nocarry(&FqRepr([0xc7867917187ca02b, 0x5d75679d4911ffef, 0x8c5b3e48b1a71c15, 0x6a427ae846fd66aa, 0x7a37e7265ee1eaf9, 0x7c0577a26f59d5]));
    assert!(t == FqRepr([0xcfae1db798be8c04, 0x999906db15a10d5a, 0x270fa8d9defc6f79, 0x83abb199c240f7b6, 0x27469abae93e1ff6, 0xdd2fd2d4dfab6be]));

    // Test for the associativity of addition.
    for _ in 0..1000 {
        let mut a = FqRepr::rand(&mut rng);
        let mut b = FqRepr::rand(&mut rng);
        let mut c = FqRepr::rand(&mut rng);

        // Unset the first few bits, so that overflow won't occur.
        a.0[5] >>= 3;
        b.0[5] >>= 3;
        c.0[5] >>= 3;

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

    // Adding 1 to (2^384 - 1) should produce a carry
    let mut x = FqRepr([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff]);
    assert!(x.add_nocarry(&FqRepr::from(1)));

    // Adding 1 to q should not produce a carry
    let mut x = FqRepr([0xb9feffffffffaaab, 0x1eabfffeb153ffff, 0x6730d2a0f6b0f624, 0x64774b84f38512bf, 0x4b1ba7b6434bacd7, 0x1a0111ea397fe69a]);
    assert!(!x.add_nocarry(&FqRepr::from(1)));
}

#[bench]
fn bench_fq_repr_add_nocarry(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<(FqRepr, FqRepr)> = (0..SAMPLES).map(|_| {
        let mut tmp1 = FqRepr::rand(&mut rng);
        let mut tmp2 = FqRepr::rand(&mut rng);
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
fn bench_fq_repr_sub_noborrow(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<(FqRepr, FqRepr)> = (0..SAMPLES).map(|_| {
        let tmp1 = FqRepr::rand(&mut rng);
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
fn bench_fq_repr_num_bits(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<FqRepr> = (0..SAMPLES).map(|_| FqRepr::rand(&mut rng)).collect();

    let mut count = 0;
    b.iter(|| {
        let tmp = v[count].num_bits();
        count = (count + 1) % SAMPLES;
        tmp
    });
}

#[bench]
fn bench_fq_repr_mul2(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<FqRepr> = (0..SAMPLES).map(|_| FqRepr::rand(&mut rng)).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count];
        tmp.mul2();
        count = (count + 1) % SAMPLES;
        tmp
    });
}

#[bench]
fn bench_fq_repr_div2(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<FqRepr> = (0..SAMPLES).map(|_| FqRepr::rand(&mut rng)).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count];
        tmp.div2();
        count = (count + 1) % SAMPLES;
        tmp
    });
}

#[test]
fn test_fq_is_valid() {
    let mut a = Fq(MODULUS);
    assert!(!a.is_valid());
    a.0.sub_noborrow(&FqRepr::from(1));
    assert!(a.is_valid());
    assert!(Fq(FqRepr::from(0)).is_valid());
    assert!(Fq(FqRepr([0xdf4671abd14dab3e, 0xe2dc0c9f534fbd33, 0x31ca6c880cc444a6, 0x257a67e70ef33359, 0xf9b29e493f899b36, 0x17c8be1800b9f059])).is_valid());
    assert!(!Fq(FqRepr([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff])).is_valid());

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        let a = Fq::rand(&mut rng);
        assert!(a.is_valid());
    }
}

#[test]
fn test_fq_add_assign() {
    {
        // Random number
        let mut tmp = Fq(FqRepr([0x624434821df92b69, 0x503260c04fd2e2ea, 0xd9df726e0d16e8ce, 0xfbcb39adfd5dfaeb, 0x86b8a22b0c88b112, 0x165a2ed809e4201b]));
        assert!(tmp.is_valid());
        // Test that adding zero has no effect.
        tmp.add_assign(&Fq(FqRepr::from(0)));
        assert_eq!(tmp, Fq(FqRepr([0x624434821df92b69, 0x503260c04fd2e2ea, 0xd9df726e0d16e8ce, 0xfbcb39adfd5dfaeb, 0x86b8a22b0c88b112, 0x165a2ed809e4201b])));
        // Add one and test for the result.
        tmp.add_assign(&Fq(FqRepr::from(1)));
        assert_eq!(tmp, Fq(FqRepr([0x624434821df92b6a, 0x503260c04fd2e2ea, 0xd9df726e0d16e8ce, 0xfbcb39adfd5dfaeb, 0x86b8a22b0c88b112, 0x165a2ed809e4201b])));
        // Add another random number that exercises the reduction.
        tmp.add_assign(&Fq(FqRepr([0x374d8f8ea7a648d8, 0xe318bb0ebb8bfa9b, 0x613d996f0a95b400, 0x9fac233cb7e4fef1, 0x67e47552d253c52, 0x5c31b227edf25da])));
        assert_eq!(tmp, Fq(FqRepr([0xdf92c410c59fc997, 0x149f1bd05a0add85, 0xd3ec393c20fba6ab, 0x37001165c1bde71d, 0x421b41c9f662408e, 0x21c38104f435f5b])));
        // Add one to (q - 1) and test for the result.
        tmp = Fq(FqRepr([0xb9feffffffffaaaa, 0x1eabfffeb153ffff, 0x6730d2a0f6b0f624, 0x64774b84f38512bf, 0x4b1ba7b6434bacd7, 0x1a0111ea397fe69a]));
        tmp.add_assign(&Fq(FqRepr::from(1)));
        assert!(tmp.0.is_zero());
        // Add a random number to another one such that the result is q - 1
        tmp = Fq(FqRepr([0x531221a410efc95b, 0x72819306027e9717, 0x5ecefb937068b746, 0x97de59cd6feaefd7, 0xdc35c51158644588, 0xb2d176c04f2100]));
        tmp.add_assign(&Fq(FqRepr([0x66ecde5bef0fe14f, 0xac2a6cf8aed568e8, 0x861d70d86483edd, 0xcc98f1b7839a22e8, 0x6ee5e2a4eae7674e, 0x194e40737930c599])));
        assert_eq!(tmp, Fq(FqRepr([0xb9feffffffffaaaa, 0x1eabfffeb153ffff, 0x6730d2a0f6b0f624, 0x64774b84f38512bf, 0x4b1ba7b6434bacd7, 0x1a0111ea397fe69a])));
        // Add one to the result and test for it.
        tmp.add_assign(&Fq(FqRepr::from(1)));
        assert!(tmp.0.is_zero());
    }

    // Test associativity

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        // Generate a, b, c and ensure (a + b) + c == a + (b + c).
        let a = Fq::rand(&mut rng);
        let b = Fq::rand(&mut rng);
        let c = Fq::rand(&mut rng);

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
fn test_fq_sub_assign() {
    {
        // Test arbitrary subtraction that tests reduction.
        let mut tmp = Fq(FqRepr([0x531221a410efc95b, 0x72819306027e9717, 0x5ecefb937068b746, 0x97de59cd6feaefd7, 0xdc35c51158644588, 0xb2d176c04f2100]));
        tmp.sub_assign(&Fq(FqRepr([0x98910d20877e4ada, 0x940c983013f4b8ba, 0xf677dc9b8345ba33, 0xbef2ce6b7f577eba, 0xe1ae288ac3222c44, 0x5968bb602790806])));
        assert_eq!(tmp, Fq(FqRepr([0x748014838971292c, 0xfd20fad49fddde5c, 0xcf87f198e3d3f336, 0x3d62d6e6e41883db, 0x45a3443cd88dc61b, 0x151d57aaf755ff94])));
        
        // Test the opposite subtraction which doesn't test reduction.
        tmp = Fq(FqRepr([0x98910d20877e4ada, 0x940c983013f4b8ba, 0xf677dc9b8345ba33, 0xbef2ce6b7f577eba, 0xe1ae288ac3222c44, 0x5968bb602790806]));
        tmp.sub_assign(&Fq(FqRepr([0x531221a410efc95b, 0x72819306027e9717, 0x5ecefb937068b746, 0x97de59cd6feaefd7, 0xdc35c51158644588, 0xb2d176c04f2100])));
        assert_eq!(tmp, Fq(FqRepr([0x457eeb7c768e817f, 0x218b052a117621a3, 0x97a8e10812dd02ed, 0x2714749e0f6c8ee3, 0x57863796abde6bc, 0x4e3ba3f4229e706])));
        
        // Test for sensible results with zero
        tmp = Fq(FqRepr::from(0));
        tmp.sub_assign(&Fq(FqRepr::from(0)));
        assert!(tmp.is_zero());

        tmp = Fq(FqRepr([0x98910d20877e4ada, 0x940c983013f4b8ba, 0xf677dc9b8345ba33, 0xbef2ce6b7f577eba, 0xe1ae288ac3222c44, 0x5968bb602790806]));
        tmp.sub_assign(&Fq(FqRepr::from(0)));
        assert_eq!(tmp, Fq(FqRepr([0x98910d20877e4ada, 0x940c983013f4b8ba, 0xf677dc9b8345ba33, 0xbef2ce6b7f577eba, 0xe1ae288ac3222c44, 0x5968bb602790806])));
    }

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        // Ensure that (a - b) + (b - a) = 0.
        let a = Fq::rand(&mut rng);
        let b = Fq::rand(&mut rng);

        let mut tmp1 = a;
        tmp1.sub_assign(&b);

        let mut tmp2 = b;
        tmp2.sub_assign(&a);

        tmp1.add_assign(&tmp2);
        assert!(tmp1.is_zero());
    }
}

#[bench]
fn bench_fq_add_assign(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<(Fq, Fq)> = (0..SAMPLES).map(|_| (Fq::rand(&mut rng), Fq::rand(&mut rng))).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count].0;
        tmp.add_assign(&v[count].1);
        count = (count + 1) % SAMPLES;
        tmp
    });
}

#[bench]
fn bench_fq_sub_assign(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<(Fq, Fq)> = (0..SAMPLES).map(|_| (Fq::rand(&mut rng), Fq::rand(&mut rng))).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count].0;
        tmp.sub_assign(&v[count].1);
        count = (count + 1) % SAMPLES;
        tmp
    });
}

#[test]
fn test_fq_mul_assign() {
    let mut tmp = Fq(FqRepr([0xcc6200000020aa8a, 0x422800801dd8001a, 0x7f4f5e619041c62c, 0x8a55171ac70ed2ba, 0x3f69cc3a3d07d58b, 0xb972455fd09b8ef]));
    tmp.mul_assign(&Fq(FqRepr([0x329300000030ffcf, 0x633c00c02cc40028, 0xbef70d925862a942, 0x4f7fa2a82a963c17, 0xdf1eb2575b8bc051, 0x1162b680fb8e9566])));
    assert!(tmp == Fq(FqRepr([0x9dc4000001ebfe14, 0x2850078997b00193, 0xa8197f1abb4d7bf, 0xc0309573f4bfe871, 0xf48d0923ffaf7620, 0x11d4b58c7a926e66])));

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000000 {
        // Ensure that (a * b) * c = a * (b * c)
        let a = Fq::rand(&mut rng);
        let b = Fq::rand(&mut rng);
        let c = Fq::rand(&mut rng);

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

        let r = Fq::rand(&mut rng);
        let mut a = Fq::rand(&mut rng);
        let mut b = Fq::rand(&mut rng);
        let mut c = Fq::rand(&mut rng);

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
fn bench_fq_mul_assign(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<(Fq, Fq)> = (0..SAMPLES).map(|_| (Fq::rand(&mut rng), Fq::rand(&mut rng))).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count].0;
        tmp.mul_assign(&v[count].1);
        count = (count + 1) % SAMPLES;
        tmp
    });
}

#[test]
fn test_fq_squaring() {
    let mut a = Fq(FqRepr([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0x19ffffffffffffff]));
    assert!(a.is_valid());
    a.square();
    assert_eq!(a, Fq::from_repr(FqRepr([0x1cfb28fe7dfbbb86, 0x24cbe1731577a59, 0xcce1d4edc120e66e, 0xdc05c659b4e15b27, 0x79361e5a802c6a23, 0x24bcbe5d51b9a6f])).unwrap());

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000000 {
        // Ensure that (a * a) = a^2
        let a = Fq::rand(&mut rng);

        let mut tmp = a;
        tmp.square();

        let mut tmp2 = a;
        tmp2.mul_assign(&a);

        assert_eq!(tmp, tmp2);
    }
}

#[bench]
fn bench_fq_square(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<Fq> = (0..SAMPLES).map(|_| Fq::rand(&mut rng)).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count];
        tmp.square();
        count = (count + 1) % SAMPLES;
        tmp
    });
}

#[test]
fn test_fq_inverse() {
    assert!(Fq::zero().inverse().is_none());

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let one = Fq::one();

    for _ in 0..1000 {
        // Ensure that a * a^-1 = 1
        let mut a = Fq::rand(&mut rng);
        let ainv = a.inverse().unwrap();
        a.mul_assign(&ainv);
        assert_eq!(a, one);
    }
}

#[bench]
fn bench_fq_inverse(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<Fq> = (0..SAMPLES).map(|_| Fq::rand(&mut rng)).collect();

    let mut count = 0;
    b.iter(|| {
        count = (count + 1) % SAMPLES;
        v[count].inverse()
    });
}

#[test]
fn test_fq_double() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        // Ensure doubling a is equivalent to adding a to itself.
        let mut a = Fq::rand(&mut rng);
        let mut b = a;
        b.add_assign(&a);
        a.double();
        assert_eq!(a, b);
    }
}

#[test]
fn test_fq_negate() {
    {
        let mut a = Fq::zero();
        a.negate();

        assert!(a.is_zero());
    }

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        // Ensure (a - (-a)) = 0.
        let mut a = Fq::rand(&mut rng);
        let mut b = a;
        b.negate();
        a.add_assign(&b);

        assert!(a.is_zero());
    }
}

#[bench]
fn bench_fq_negate(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<Fq> = (0..SAMPLES).map(|_| Fq::rand(&mut rng)).collect();

    let mut count = 0;
    b.iter(|| {
        let mut tmp = v[count];
        tmp.negate();
        count = (count + 1) % SAMPLES;
        tmp
    });
}


#[test]
fn test_fq_pow() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for i in 0..1000 {
        // Exponentiate by various small numbers and ensure it consists with repeated
        // multiplication.
        let a = Fq::rand(&mut rng);
        let target = a.pow(&[i]);
        let mut c = Fq::one();
        for _ in 0..i {
            c.mul_assign(&a);
        }
        assert_eq!(c, target);
    }

    for _ in 0..1000 {
        // Exponentiating by the modulus should have no effect in a prime field.
        let a = Fq::rand(&mut rng);

        assert_eq!(a, a.pow(Fq::char()));
    }
}

#[test]
fn test_fq_sqrt() {
	let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    assert_eq!(Fq::zero().sqrt().unwrap(), Fq::zero());

    for _ in 0..1000 {
        // Ensure sqrt(a^2) = a or -a
        let a = Fq::rand(&mut rng);
        let mut nega = a;
        nega.negate();
        let mut b = a;
        b.square();

        let b = b.sqrt().unwrap();

        assert!(a == b || nega == b);
    }

    for _ in 0..1000 {
        // Ensure sqrt(a)^2 = a for random a
        let a = Fq::rand(&mut rng);

        if let Some(mut tmp) = a.sqrt() {
            tmp.square();

            assert_eq!(a, tmp);
        }
    }
}

#[bench]
fn bench_fq_sqrt(b: &mut ::test::Bencher) {
	const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<Fq> = (0..SAMPLES).map(|_| {
        let mut tmp = Fq::rand(&mut rng);
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
fn test_fq_from_into_repr() {
    // q + 1 should not be in the field
    assert!(Fq::from_repr(FqRepr([0xb9feffffffffaaac, 0x1eabfffeb153ffff, 0x6730d2a0f6b0f624, 0x64774b84f38512bf, 0x4b1ba7b6434bacd7, 0x1a0111ea397fe69a])).is_err());

    // q should not be in the field
    assert!(Fq::from_repr(Fq::char()).is_err());

    // Multiply some arbitrary representations to see if the result is as expected.
    let a = FqRepr([0x4a49dad4ff6cde2d, 0xac62a82a8f51cd50, 0x2b1f41ab9f36d640, 0x908a387f480735f1, 0xae30740c08a875d7, 0x6c80918a365ef78]);
    let mut a_fq = Fq::from_repr(a).unwrap();
    let b = FqRepr([0xbba57917c32f0cf0, 0xe7f878cf87f05e5d, 0x9498b4292fd27459, 0xd59fd94ee4572cfa, 0x1f607186d5bb0059, 0xb13955f5ac7f6a3]);
    let b_fq = Fq::from_repr(b).unwrap();
    let c = FqRepr([0xf5f70713b717914c, 0x355ea5ac64cbbab1, 0xce60dd43417ec960, 0xf16b9d77b0ad7d10, 0xa44c204c1de7cdb7, 0x1684487772bc9a5a]);
    a_fq.mul_assign(&b_fq);
    assert_eq!(a_fq.into_repr(), c);

    // Zero should be in the field.
    assert!(Fq::from_repr(FqRepr::from(0)).unwrap().is_zero());

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        // Try to turn Fq elements into representations and back again, and compare.
        let a = Fq::rand(&mut rng);
        let a_repr = a.into_repr();
        let b_repr = FqRepr::from(a);
        assert_eq!(a_repr, b_repr);
        let a_again = Fq::from_repr(a_repr).unwrap();

        assert_eq!(a, a_again);
    }
}

#[bench]
fn bench_fq_into_repr(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<Fq> = (0..SAMPLES).map(|_| {
        Fq::rand(&mut rng)
    }).collect();

    let mut count = 0;
    b.iter(|| {
        count = (count + 1) % SAMPLES;
        v[count].into_repr()
    });
}

#[bench]
fn bench_fq_from_repr(b: &mut ::test::Bencher) {
    const SAMPLES: usize = 1000;

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let v: Vec<FqRepr> = (0..SAMPLES).map(|_| {
        Fq::rand(&mut rng).into_repr()
    }).collect();

    let mut count = 0;
    b.iter(|| {
        count = (count + 1) % SAMPLES;
        Fq::from_repr(v[count])
    });
}

#[test]
fn test_fq_repr_display() {
    assert_eq!(
        format!("{}", FqRepr([0xa956babf9301ea24, 0x39a8f184f3535c7b, 0xb38d35b3f6779585, 0x676cc4eef4c46f2c, 0xb1d4aad87651e694, 0x1947f0d5f4fe325a])),
        "0x1947f0d5f4fe325ab1d4aad87651e694676cc4eef4c46f2cb38d35b3f677958539a8f184f3535c7ba956babf9301ea24".to_string()
    );
    assert_eq!(
        format!("{}", FqRepr([0xb4171485fd8622dd, 0x864229a6edec7ec5, 0xc57f7bdcf8dfb707, 0x6db7ff0ecea4584a, 0xf8d8578c4a57132d, 0x6eb66d42d9fcaaa])),
        "0x06eb66d42d9fcaaaf8d8578c4a57132d6db7ff0ecea4584ac57f7bdcf8dfb707864229a6edec7ec5b4171485fd8622dd".to_string()
    );
    assert_eq!(
        format!("{}", FqRepr([0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff])),
        "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_string()
    );
    assert_eq!(
        format!("{}", FqRepr([0, 0, 0, 0, 0, 0])),
        "0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string()
    );
}

#[test]
fn test_fq_display() {
    assert_eq!(
        format!("{}", Fq::from_repr(FqRepr([0xa956babf9301ea24, 0x39a8f184f3535c7b, 0xb38d35b3f6779585, 0x676cc4eef4c46f2c, 0xb1d4aad87651e694, 0x1947f0d5f4fe325a])).unwrap()),
        "Fq(0x1947f0d5f4fe325ab1d4aad87651e694676cc4eef4c46f2cb38d35b3f677958539a8f184f3535c7ba956babf9301ea24)".to_string()
    );
    assert_eq!(
        format!("{}", Fq::from_repr(FqRepr([0xe28e79396ac2bbf8, 0x413f6f7f06ea87eb, 0xa4b62af4a792a689, 0xb7f89f88f59c1dc5, 0x9a551859b1e43a9a, 0x6c9f5a1060de974])).unwrap()),
        "Fq(0x06c9f5a1060de9749a551859b1e43a9ab7f89f88f59c1dc5a4b62af4a792a689413f6f7f06ea87ebe28e79396ac2bbf8)".to_string()
    );
}

#[test]
fn test_fq_num_bits() {
	assert_eq!(Fq::num_bits(), 381);
	assert_eq!(Fq::capacity(), 380);
}

#[test]
fn test_fq_root_of_unity() {
    assert_eq!(Fq::s(), 1);
    assert_eq!(Fq::multiplicative_generator(), Fq::from_repr(FqRepr::from(2)).unwrap());
    assert_eq!(
        Fq::multiplicative_generator().pow([0xdcff7fffffffd555, 0xf55ffff58a9ffff, 0xb39869507b587b12, 0xb23ba5c279c2895f, 0x258dd3db21a5d66b, 0xd0088f51cbff34d]),
        Fq::root_of_unity()
    );
    assert_eq!(
        Fq::root_of_unity().pow([1 << Fq::s()]),
        Fq::one()
    );
    assert!(Fq::multiplicative_generator().sqrt().is_none());
}

#[test]
fn fq_field_tests() {
    ::tests::field::random_field_tests::<Fq>();
    ::tests::field::random_sqrt_tests::<Fq>();
    ::tests::field::random_frobenius_tests::<Fq, _>(Fq::char(), 13);
    ::tests::field::from_str_tests::<Fq>();
}

#[test]
fn test_fq_ordering() {
    // FqRepr's ordering is well-tested, but we still need to make sure the Fq
    // elements aren't being compared in Montgomery form.
    for i in 0..100 {
        assert!(Fq::from_repr(FqRepr::from(i+1)).unwrap() > Fq::from_repr(FqRepr::from(i)).unwrap());
    }
}

#[test]
fn fq_repr_tests() {
    ::tests::repr::random_repr_tests::<FqRepr>();
}
