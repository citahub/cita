use super::*;
use ::*;

fn test_vectors<G: CurveProjective, E: EncodedPoint<Affine=G::Affine>>(expected: &[u8])
{
    let mut e = G::zero();

    let mut v = vec![];
    {
        let mut expected = expected;
        for _ in 0..1000 {
            let e_affine = e.into_affine();
            let encoded = E::from_affine(e_affine);
            v.extend_from_slice(encoded.as_ref());

            let mut decoded = E::empty();
            decoded.as_mut().copy_from_slice(&expected[0..E::size()]);
            expected = &expected[E::size()..];
            let decoded = decoded.into_affine().unwrap();
            assert_eq!(e_affine, decoded);

            e.add_assign(&G::one());
        }
    }

    assert_eq!(&v[..], expected);
}

#[test]
fn test_g1_uncompressed_valid_vectors() {
    test_vectors::<G1, G1Uncompressed>(include_bytes!("g1_uncompressed_valid_test_vectors.dat"));
}

#[test]
fn test_g1_compressed_valid_vectors() {
    test_vectors::<G1, G1Compressed>(include_bytes!("g1_compressed_valid_test_vectors.dat"));
}

#[test]
fn test_g2_uncompressed_valid_vectors() {
    test_vectors::<G2, G2Uncompressed>(include_bytes!("g2_uncompressed_valid_test_vectors.dat"));
}

#[test]
fn test_g2_compressed_valid_vectors() {
    test_vectors::<G2, G2Compressed>(include_bytes!("g2_compressed_valid_test_vectors.dat"));
}

#[test]
fn test_g1_uncompressed_invalid_vectors() {
    {
        let z = G1Affine::zero().into_uncompressed();

        {
            let mut z = z;
            z.as_mut()[0] |= 0b1000_0000;
            if let Err(GroupDecodingError::UnexpectedCompressionMode) = z.into_affine() {
                // :)
            } else {
                panic!("should have rejected the point because we expected an uncompressed point");
            }
        }

        {
            let mut z = z;
            z.as_mut()[0] |= 0b0010_0000;
            if let Err(GroupDecodingError::UnexpectedInformation) = z.into_affine() {
                // :)
            } else {
                panic!("should have rejected the point because the parity bit should not be set if the point is at infinity");
            }
        }

        for i in 0..G1Uncompressed::size() {
            let mut z = z;
            z.as_mut()[i] |= 0b0000_0001;
            if let Err(GroupDecodingError::UnexpectedInformation) = z.into_affine() {
                // :)
            } else {
                panic!("should have rejected the point because the coordinates should be zeroes at the point at infinity");
            }
        }
    }

    let o = G1Affine::one().into_uncompressed();

    {
        let mut o = o;
        o.as_mut()[0] |= 0b1000_0000;
        if let Err(GroupDecodingError::UnexpectedCompressionMode) = o.into_affine() {
            // :)
        } else {
            panic!("should have rejected the point because we expected an uncompressed point");
        }
    }

    let m = Fq::char();

    {
        let mut o = o;
        m.write_be(&mut o.as_mut()[0..]).unwrap();

        if let Err(GroupDecodingError::CoordinateDecodingError(coordinate, _)) = o.into_affine() {
            assert_eq!(coordinate, "x coordinate");
        } else {
            panic!("should have rejected the point")
        }
    }

    {
        let mut o = o;
        m.write_be(&mut o.as_mut()[48..]).unwrap();

        if let Err(GroupDecodingError::CoordinateDecodingError(coordinate, _)) = o.into_affine() {
            assert_eq!(coordinate, "y coordinate");
        } else {
            panic!("should have rejected the point")
        }
    }

    {
        let m = Fq::zero().into_repr();

        let mut o = o;
        m.write_be(&mut o.as_mut()[0..]).unwrap();

        if let Err(GroupDecodingError::NotOnCurve) = o.into_affine() {
            // :)
        } else {
            panic!("should have rejected the point because it isn't on the curve")
        }
    }

    {
        let mut o = o;
        let mut x = Fq::one();

        loop {
            let mut x3b = x;
            x3b.square();
            x3b.mul_assign(&x);
            x3b.add_assign(&Fq::from_repr(FqRepr::from(4)).unwrap()); // TODO: perhaps expose coeff_b through API?

            if let Some(y) = x3b.sqrt() {
                // We know this is on the curve, but it's likely not going to be in the correct subgroup.
                x.into_repr().write_be(&mut o.as_mut()[0..]).unwrap();
                y.into_repr().write_be(&mut o.as_mut()[48..]).unwrap();

                if let Err(GroupDecodingError::NotInSubgroup) = o.into_affine() {
                    break
                } else {
                    panic!("should have rejected the point because it isn't in the correct subgroup")
                }
            } else {
                x.add_assign(&Fq::one());
            }
        }
    }
}

#[test]
fn test_g2_uncompressed_invalid_vectors() {
    {
        let z = G2Affine::zero().into_uncompressed();

        {
            let mut z = z;
            z.as_mut()[0] |= 0b1000_0000;
            if let Err(GroupDecodingError::UnexpectedCompressionMode) = z.into_affine() {
                // :)
            } else {
                panic!("should have rejected the point because we expected an uncompressed point");
            }
        }

        {
            let mut z = z;
            z.as_mut()[0] |= 0b0010_0000;
            if let Err(GroupDecodingError::UnexpectedInformation) = z.into_affine() {
                // :)
            } else {
                panic!("should have rejected the point because the parity bit should not be set if the point is at infinity");
            }
        }

        for i in 0..G2Uncompressed::size() {
            let mut z = z;
            z.as_mut()[i] |= 0b0000_0001;
            if let Err(GroupDecodingError::UnexpectedInformation) = z.into_affine() {
                // :)
            } else {
                panic!("should have rejected the point because the coordinates should be zeroes at the point at infinity");
            }
        }
    }

    let o = G2Affine::one().into_uncompressed();

    {
        let mut o = o;
        o.as_mut()[0] |= 0b1000_0000;
        if let Err(GroupDecodingError::UnexpectedCompressionMode) = o.into_affine() {
            // :)
        } else {
            panic!("should have rejected the point because we expected an uncompressed point");
        }
    }

    let m = Fq::char();

    {
        let mut o = o;
        m.write_be(&mut o.as_mut()[0..]).unwrap();

        if let Err(GroupDecodingError::CoordinateDecodingError(coordinate, _)) = o.into_affine() {
            assert_eq!(coordinate, "x coordinate (c1)");
        } else {
            panic!("should have rejected the point")
        }
    }

    {
        let mut o = o;
        m.write_be(&mut o.as_mut()[48..]).unwrap();

        if let Err(GroupDecodingError::CoordinateDecodingError(coordinate, _)) = o.into_affine() {
            assert_eq!(coordinate, "x coordinate (c0)");
        } else {
            panic!("should have rejected the point")
        }
    }

    {
        let mut o = o;
        m.write_be(&mut o.as_mut()[96..]).unwrap();

        if let Err(GroupDecodingError::CoordinateDecodingError(coordinate, _)) = o.into_affine() {
            assert_eq!(coordinate, "y coordinate (c1)");
        } else {
            panic!("should have rejected the point")
        }
    }

    {
        let mut o = o;
        m.write_be(&mut o.as_mut()[144..]).unwrap();

        if let Err(GroupDecodingError::CoordinateDecodingError(coordinate, _)) = o.into_affine() {
            assert_eq!(coordinate, "y coordinate (c0)");
        } else {
            panic!("should have rejected the point")
        }
    }

    {
        let m = Fq::zero().into_repr();

        let mut o = o;
        m.write_be(&mut o.as_mut()[0..]).unwrap();
        m.write_be(&mut o.as_mut()[48..]).unwrap();

        if let Err(GroupDecodingError::NotOnCurve) = o.into_affine() {
            // :)
        } else {
            panic!("should have rejected the point because it isn't on the curve")
        }
    }

    {
        let mut o = o;
        let mut x = Fq2::one();

        loop {
            let mut x3b = x;
            x3b.square();
            x3b.mul_assign(&x);
            x3b.add_assign(&Fq2 {
                c0: Fq::from_repr(FqRepr::from(4)).unwrap(),
                c1: Fq::from_repr(FqRepr::from(4)).unwrap()
            }); // TODO: perhaps expose coeff_b through API?

            if let Some(y) = x3b.sqrt() {
                // We know this is on the curve, but it's likely not going to be in the correct subgroup.
                x.c1.into_repr().write_be(&mut o.as_mut()[0..]).unwrap();
                x.c0.into_repr().write_be(&mut o.as_mut()[48..]).unwrap();
                y.c1.into_repr().write_be(&mut o.as_mut()[96..]).unwrap();
                y.c0.into_repr().write_be(&mut o.as_mut()[144..]).unwrap();

                if let Err(GroupDecodingError::NotInSubgroup) = o.into_affine() {
                    break
                } else {
                    panic!("should have rejected the point because it isn't in the correct subgroup")
                }
            } else {
                x.add_assign(&Fq2::one());
            }
        }
    }
}

#[test]
fn test_g1_compressed_invalid_vectors() {
    {
        let z = G1Affine::zero().into_compressed();

        {
            let mut z = z;
            z.as_mut()[0] &= 0b0111_1111;
            if let Err(GroupDecodingError::UnexpectedCompressionMode) = z.into_affine() {
                // :)
            } else {
                panic!("should have rejected the point because we expected a compressed point");
            }
        }

        {
            let mut z = z;
            z.as_mut()[0] |= 0b0010_0000;
            if let Err(GroupDecodingError::UnexpectedInformation) = z.into_affine() {
                // :)
            } else {
                panic!("should have rejected the point because the parity bit should not be set if the point is at infinity");
            }
        }

        for i in 0..G1Compressed::size() {
            let mut z = z;
            z.as_mut()[i] |= 0b0000_0001;
            if let Err(GroupDecodingError::UnexpectedInformation) = z.into_affine() {
                // :)
            } else {
                panic!("should have rejected the point because the coordinates should be zeroes at the point at infinity");
            }
        }
    }

    let o = G1Affine::one().into_compressed();

    {
        let mut o = o;
        o.as_mut()[0] &= 0b0111_1111;
        if let Err(GroupDecodingError::UnexpectedCompressionMode) = o.into_affine() {
            // :)
        } else {
            panic!("should have rejected the point because we expected a compressed point");
        }
    }

    let m = Fq::char();

    {
        let mut o = o;
        m.write_be(&mut o.as_mut()[0..]).unwrap();
        o.as_mut()[0] |= 0b1000_0000;

        if let Err(GroupDecodingError::CoordinateDecodingError(coordinate, _)) = o.into_affine() {
            assert_eq!(coordinate, "x coordinate");
        } else {
            panic!("should have rejected the point")
        }
    }

    {
        let mut o = o;
        let mut x = Fq::one();

        loop {
            let mut x3b = x;
            x3b.square();
            x3b.mul_assign(&x);
            x3b.add_assign(&Fq::from_repr(FqRepr::from(4)).unwrap()); // TODO: perhaps expose coeff_b through API?

            if let Some(_) = x3b.sqrt() {
                x.add_assign(&Fq::one());
            } else {
                x.into_repr().write_be(&mut o.as_mut()[0..]).unwrap();
                o.as_mut()[0] |= 0b1000_0000;

                if let Err(GroupDecodingError::NotOnCurve) = o.into_affine() {
                    break
                } else {
                    panic!("should have rejected the point because it isn't on the curve")
                }
            }
        }
    }

    {
        let mut o = o;
        let mut x = Fq::one();

        loop {
            let mut x3b = x;
            x3b.square();
            x3b.mul_assign(&x);
            x3b.add_assign(&Fq::from_repr(FqRepr::from(4)).unwrap()); // TODO: perhaps expose coeff_b through API?

            if let Some(_) = x3b.sqrt() {
                // We know this is on the curve, but it's likely not going to be in the correct subgroup.
                x.into_repr().write_be(&mut o.as_mut()[0..]).unwrap();
                o.as_mut()[0] |= 0b1000_0000;

                if let Err(GroupDecodingError::NotInSubgroup) = o.into_affine() {
                    break
                } else {
                    panic!("should have rejected the point because it isn't in the correct subgroup")
                }
            } else {
                x.add_assign(&Fq::one());
            }
        }
    }
}

#[test]
fn test_g2_compressed_invalid_vectors() {
    {
        let z = G2Affine::zero().into_compressed();

        {
            let mut z = z;
            z.as_mut()[0] &= 0b0111_1111;
            if let Err(GroupDecodingError::UnexpectedCompressionMode) = z.into_affine() {
                // :)
            } else {
                panic!("should have rejected the point because we expected a compressed point");
            }
        }

        {
            let mut z = z;
            z.as_mut()[0] |= 0b0010_0000;
            if let Err(GroupDecodingError::UnexpectedInformation) = z.into_affine() {
                // :)
            } else {
                panic!("should have rejected the point because the parity bit should not be set if the point is at infinity");
            }
        }

        for i in 0..G2Compressed::size() {
            let mut z = z;
            z.as_mut()[i] |= 0b0000_0001;
            if let Err(GroupDecodingError::UnexpectedInformation) = z.into_affine() {
                // :)
            } else {
                panic!("should have rejected the point because the coordinates should be zeroes at the point at infinity");
            }
        }
    }

    let o = G2Affine::one().into_compressed();

    {
        let mut o = o;
        o.as_mut()[0] &= 0b0111_1111;
        if let Err(GroupDecodingError::UnexpectedCompressionMode) = o.into_affine() {
            // :)
        } else {
            panic!("should have rejected the point because we expected a compressed point");
        }
    }

    let m = Fq::char();

    {
        let mut o = o;
        m.write_be(&mut o.as_mut()[0..]).unwrap();
        o.as_mut()[0] |= 0b1000_0000;

        if let Err(GroupDecodingError::CoordinateDecodingError(coordinate, _)) = o.into_affine() {
            assert_eq!(coordinate, "x coordinate (c1)");
        } else {
            panic!("should have rejected the point")
        }
    }

    {
        let mut o = o;
        m.write_be(&mut o.as_mut()[48..]).unwrap();
        o.as_mut()[0] |= 0b1000_0000;

        if let Err(GroupDecodingError::CoordinateDecodingError(coordinate, _)) = o.into_affine() {
            assert_eq!(coordinate, "x coordinate (c0)");
        } else {
            panic!("should have rejected the point")
        }
    }

    {
        let mut o = o;
        let mut x = Fq2 {
            c0: Fq::one(),
            c1: Fq::one()
        };

        loop {
            let mut x3b = x;
            x3b.square();
            x3b.mul_assign(&x);
            x3b.add_assign(&Fq2 {
                c0: Fq::from_repr(FqRepr::from(4)).unwrap(),
                c1: Fq::from_repr(FqRepr::from(4)).unwrap(),
            }); // TODO: perhaps expose coeff_b through API?

            if let Some(_) = x3b.sqrt() {
                x.add_assign(&Fq2::one());
            } else {
                x.c1.into_repr().write_be(&mut o.as_mut()[0..]).unwrap();
                x.c0.into_repr().write_be(&mut o.as_mut()[48..]).unwrap();
                o.as_mut()[0] |= 0b1000_0000;

                if let Err(GroupDecodingError::NotOnCurve) = o.into_affine() {
                    break
                } else {
                    panic!("should have rejected the point because it isn't on the curve")
                }
            }
        }
    }

    {
        let mut o = o;
        let mut x = Fq2 {
            c0: Fq::one(),
            c1: Fq::one()
        };

        loop {
            let mut x3b = x;
            x3b.square();
            x3b.mul_assign(&x);
            x3b.add_assign(&Fq2 {
                c0: Fq::from_repr(FqRepr::from(4)).unwrap(),
                c1: Fq::from_repr(FqRepr::from(4)).unwrap(),
            }); // TODO: perhaps expose coeff_b through API?

            if let Some(_) = x3b.sqrt() {
                // We know this is on the curve, but it's likely not going to be in the correct subgroup.
                x.c1.into_repr().write_be(&mut o.as_mut()[0..]).unwrap();
                x.c0.into_repr().write_be(&mut o.as_mut()[48..]).unwrap();
                o.as_mut()[0] |= 0b1000_0000;

                if let Err(GroupDecodingError::NotInSubgroup) = o.into_affine() {
                    break
                } else {
                    panic!("should have rejected the point because it isn't in the correct subgroup")
                }
            } else {
                x.add_assign(&Fq2::one());
            }
        }
    }
}
