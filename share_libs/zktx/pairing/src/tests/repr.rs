use rand::{SeedableRng, XorShiftRng};
use ::{PrimeFieldRepr};

pub fn random_repr_tests<R: PrimeFieldRepr>() {
    random_encoding_tests::<R>();
    random_muln_tests::<R>();
    random_divn_tests::<R>();
}

fn random_encoding_tests<R: PrimeFieldRepr>() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        let r = R::rand(&mut rng);
        let mut rdecoded = R::default();

        let mut v: Vec<u8> = vec![];
        r.write_be(&mut v).unwrap();
        rdecoded.read_be(&v[0..]).unwrap();

        assert_eq!(r, rdecoded);
    }
}

fn random_muln_tests<R: PrimeFieldRepr>() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..100 {
        let r = R::rand(&mut rng);

        for shift in 0..(r.num_bits()+1) {
            let mut r1 = r;
            let mut r2 = r;

            for _ in 0..shift {
                r1.mul2();
            }

            r2.muln(shift);

            assert_eq!(r1, r2);
        }
    }
}

fn random_divn_tests<R: PrimeFieldRepr>() {
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..100 {
        let r = R::rand(&mut rng);

        for shift in 0..(r.num_bits()+1) {
            let mut r1 = r;
            let mut r2 = r;

            for _ in 0..shift {
                r1.div2();
            }

            r2.divn(shift);

            assert_eq!(r1, r2);
        }
    }
}
