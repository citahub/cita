use super::{CurveProjective, PrimeFieldRepr};

/// Replaces the contents of `table` with a w-NAF window table for the given window size.
///
/// This function will panic if provided a window size below two, or above 22.
pub fn wnaf_table<G: CurveProjective>(table: &mut Vec<G>, mut base: G, window: usize)
{
    assert!(window < 23);
    assert!(window > 1);

    table.truncate(0);
    table.reserve(1 << (window-1));

    let mut dbl = base;
    dbl.double();

    for _ in 0..(1 << (window-1)) {
        table.push(base);
        base.add_assign(&dbl);
    }
}

/// Replaces the contents of `wnaf` with the w-NAF representation of a scalar.
///
/// This function will panic if provided a window size below two, or above 22.
pub fn wnaf_form<S: PrimeFieldRepr>(wnaf: &mut Vec<i64>, mut c: S, window: usize)
{
    assert!(window < 23);
    assert!(window > 1);

    wnaf.truncate(0);

    while !c.is_zero() {
        let mut u;
        if c.is_odd() {
            u = (c.as_ref()[0] % (1 << (window+1))) as i64;

            if u > (1 << window) {
                u -= 1 << (window+1);
            }

            if u > 0 {
                c.sub_noborrow(&S::from(u as u64));
            } else {
                c.add_nocarry(&S::from((-u) as u64));
            }
        } else {
            u = 0;
        }

        wnaf.push(u);

        c.div2();
    }
}

/// Performs w-NAF exponentiation with the provided window table and w-NAF form scalar.
///
/// This function must be provided a `table` and `wnaf` that were constructed with
/// the same window size; otherwise, it may panic or produce invalid results.
pub fn wnaf_exp<G: CurveProjective>(table: &[G], wnaf: &[i64]) -> G
{
    let mut result = G::zero();

    let mut found_one = false;

    for n in wnaf.iter().rev() {
        if found_one {
            result.double();
        }

        if *n != 0 {
            found_one = true;

            if *n > 0 {
                result.add_assign(&table[(n/2) as usize]);
            } else {
                result.sub_assign(&table[((-n)/2) as usize]);
            }
        }
    }

    result
}
