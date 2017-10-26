
extern crate pairing;
extern crate bellman;
extern crate rand;
extern crate jubjub;

pub use pairing::bls12_381::FrRepr;

pub mod base;

pub mod b2c;

pub mod c2b;

pub mod c2c;

pub mod c2p;

pub mod p2c;

pub fn pedersen_hash(bits:&[bool])->[u64;4]{
    assert_eq!(bits.len(),base::PHIN);
    jubjub::pedersen_hash_real(bits,&base::ph_generator()).unwrap().serial()
}

pub fn pedersen_hash_root(c0:[u64;4],c1:[u64;4])->[u64;4]{
    let mut v = Vec::with_capacity(512);
    for num in c0.into_iter(){
        let mut num = *num;
        for _ in 0..64{
            v.push(num&1==1);
            num>>=1;
        }
    }
    for num in c1.into_iter(){
        let mut num = *num;
        for _ in 0..64{
            v.push(num&1==1);
            num>>=1;
        }
    }
    jubjub::pedersen_hash_real(v.as_slice(),&base::ph_generator()).unwrap().serial()
}