extern crate rand;
extern crate zktx;

use rand::{Rng, thread_rng};
use zktx::base::*;

fn test_b2c() {
    println!("test_b2c");
    use zktx::b2c::*;

    ensure_b2c_param().unwrap();

    use std::time::{Duration, Instant};
    let mut total = Duration::new(0, 0);

    const SAMPLES: u32 = 10;

    println!("Creating {} proofs and averaging the time spent creating them.", SAMPLES);

    for _ in 0..SAMPLES {
        let now = Instant::now();
        let rng = &mut thread_rng();
        let rcm = (0..RCMBIT).map(|_| rng.gen()).collect::<Vec<bool>>();
        let addr = (0..PHBIT).map(|_| rng.gen()).collect::<Vec<bool>>();
        let ba = "1000";
        let va = "10";
        let (proof, bn, coin) = b2c_info(rcm, ba, va, addr).unwrap();
//        println!("H_B   = {:?}", bn);
//        println!("coin  = {:?}", coin);
//        println!("proof  = {:?}", proof);
        total += now.elapsed();

        let res = b2c_verify(ba, va, coin, proof).unwrap();
        assert!(res);
    }
    println!("average proving time: {:?}", total / SAMPLES);
}

fn test_c2b(){
    println!("test_c2b");
    use zktx::c2b::*;

    ensure_c2b_param().unwrap();

    use std::time::{Duration,Instant};
    let mut total = Duration::new(0, 0);

    const SAMPLES:u32 = 10;

    println!("Creating {} proofs and averaging the time spent creating them.", SAMPLES);

    for _ in 0..SAMPLES{
        let now = Instant::now();
        let rng = &mut thread_rng();
        let rcm = (0..RCMBIT).map(|_| rng.gen()).collect::<Vec<bool>>();
        let addr_sk = (0..ADSK).map(|_| rng.gen()).collect::<Vec<bool>>();
        let ba = "1000";
        let va = "10";
        let path = (0..TREEDEPTH).map(|_| {
            let mut v:[u64;4] = [0;4];
            for i in 0..4{
                v[i] = rng.gen();
            }
            v
        }).collect();
        let locs = (0..TREEDEPTH).map(|_| rng.gen()).collect::<Vec<bool>>();
        let (proof,bn,nullifier,root) = c2b_info(rcm,ba,va,addr_sk,path,locs).unwrap();
//        println!("H_B   = {:?}",bn);
//        println!("nullifier  = {:?}",nullifier);
//        println!("root = {:?}",root);
//        println!("proof  = {:?}", proof);
        total += now.elapsed();

        let res = c2b_verify(ba,va,nullifier,root,proof).unwrap();
        assert!(res);
    }
    println!("average proving time: {:?}", total / SAMPLES);
}

fn test_c2p(){
    println!("test_c2p");
    use zktx::c2p::*;
    use zktx::{pedersen_hash,pedersen_hash_root};

    ensure_c2p_param().unwrap();

    use std::time::{Duration,Instant};
    let mut total = Duration::new(0, 0);

    const SAMPLES:u32 = 10;

    println!("Creating {} proofs and averaging the time spent creating them.", SAMPLES);

    for _ in 0..SAMPLES{
        let now = Instant::now();
        //倒序：359=101100111 -> [1,1,1,0,0,1,1,0,1]
        let rng = &mut thread_rng();
        let rh = (0..RHBIT).map(|_| rng.gen()).collect::<Vec<bool>>();
        let rhn = (0..RHBIT).map(|_| rng.gen()).collect::<Vec<bool>>();
        let rcm = (0..RCMBIT).map(|_| rng.gen()).collect::<Vec<bool>>();
        let addr_sk = (0..ADSK).map(|_| rng.gen()).collect::<Vec<bool>>();
        let ba = "1000";
        let va = "10";
        let path:Vec<[u64;4]> = (0..TREEDEPTH).map(|_| {
            let mut v:[u64;4] = [0;4];
            for i in 0..4{
                v[i] = rng.gen();
            }
            v
        }).collect();
        let locs:Vec<bool> = (0..TREEDEPTH).map(|_| rng.gen()).collect::<Vec<bool>>();
        let coin = pedersen_hash({
            let addr = pedersen_hash({
                let mut v = addr_sk.clone();
                for _ in 0..256{
                    v.push(true);
                }
                v
            }.as_slice());
            let mut v = Vec::with_capacity(256);
            for num in addr.into_iter(){
                let mut num = *num;
                for _ in 0..64{
                    v.push(num&1==1);
                    num>>=1;
                }
            }
            let addr = v;
            let mut node = rcm.clone();
            let mut va = [false;128];
            va[1]=true;
            va[3]=true;//10
            for b in va.iter(){
                node.push(*b);
            }
            for b in addr.iter(){
                node.push(*b);
            }
            node
        }.as_slice());
        let path2 = path.clone();
        let loc2 = locs.clone();
        let (proof,hb,nullifier,hbn,root) = c2p_info(rh,rhn,rcm,ba,va,addr_sk,path,locs).unwrap();
//        println!("H_B   = {:?}",hb);
//        println!("nullifier  = {:?}",nullifier);
//        println!("H_B-n = {:?}",hbn);
//        println!("root = {:?}",root);
//        println!("proof  = {:?}", proof);
        total += now.elapsed();

        let root = {
            let mut root = coin;
            for i in 0..TREEDEPTH{
                if loc2[i]{
                    root = pedersen_hash_root(path2[i],root);
                }else{
                    root = pedersen_hash_root(root,path2[i]);
                }
            }
            root
        };

        let res = c2p_verify(hb,nullifier,hbn,root,proof).unwrap();
        assert!(res);
    }
    println!("average proving time: {:?}", total / SAMPLES);
}

fn test_p2c(){
    println!("test_p2c");
    use zktx::p2c::*;

    ensure_p2c_param().unwrap();

    use std::time::{Duration,Instant};
    let mut total = Duration::new(0, 0);

    const SAMPLES:u32 = 10;

    println!("Creating {} proofs and averaging the time spent creating them.", SAMPLES);

    for _ in 0..SAMPLES{
        let now = Instant::now();
        //倒序：359=101100111 -> [1,1,1,0,0,1,1,0,1]
        let rng = &mut thread_rng();
        let rh = (0..RHBIT).map(|_| rng.gen()).collect::<Vec<bool>>();
        let rhn = (0..RHBIT).map(|_| rng.gen()).collect::<Vec<bool>>();
        let rcm = (0..RCMBIT).map(|_| rng.gen()).collect::<Vec<bool>>();
        let addr = (0..PHBIT).map(|_| rng.gen()).collect::<Vec<bool>>();
        let ba = "1000";
        let va = "10";
        let (proof,hb,coin,hbn) = p2c_info(rh,rhn,rcm,ba,va,addr).unwrap();
//        println!("H_B   = {:?}",hb);
//        println!("coin  = {:?}",coin);
//        println!("H_B-n = {:?}",hbn);
//        println!("proof  = {:?}", proof);
        total += now.elapsed();

        let res = p2c_verify(hb,coin,hbn,proof).unwrap();
        assert!(res);
    }
    println!("average proving time: {:?}", total / SAMPLES);
}

fn test_c2c(){
    println!("test_c2c");
    use zktx::c2c::*;

    ensure_c2c_param().unwrap();

    use std::time::{Duration,Instant};
    let mut total = Duration::new(0, 0);

    const SAMPLES:u32 = 10;

    println!("Creating {} proofs and averaging the time spent creating them.", SAMPLES);

    for _ in 0..SAMPLES{
        let now = Instant::now();
        let rng = &mut thread_rng();
        let rcm1 = (0..RCMBIT).map(|_| rng.gen()).collect::<Vec<bool>>();
        let addr_sk1 = (0..ADSK).map(|_| rng.gen()).collect::<Vec<bool>>();
        let va1 = "10";
        let path1 = (0..TREEDEPTH).map(|_| {
            let mut v:[u64;4] = [0;4];
            for i in 0..4{
                v[i] = rng.gen();
            }
            v
        }).collect();
        let locs1 = (0..TREEDEPTH).map(|_| rng.gen()).collect::<Vec<bool>>();
        let rcm2 = (0..RCMBIT).map(|_| rng.gen()).collect::<Vec<bool>>();
        let addr_sk2 = (0..ADSK).map(|_| rng.gen()).collect::<Vec<bool>>();
        let va2 = "10";
        let path2 = (0..TREEDEPTH).map(|_| {
            let mut v:[u64;4] = [0;4];
            for i in 0..4{
                v[i] = rng.gen();
            }
            v
        }).collect();
        let locs2 = (0..TREEDEPTH).map(|_| rng.gen()).collect::<Vec<bool>>();
        let rcm = (0..RCMBIT).map(|_| rng.gen()).collect::<Vec<bool>>();
        let addr = (0..PHBIT).map(|_| rng.gen()).collect::<Vec<bool>>();
        let (proof,coin,nullifier1,nullifier2,root1,root2) = c2c_info(rcm1,va1,addr_sk1,path1,locs1,rcm2,va2,addr_sk2,path2,locs2,rcm,addr).unwrap();
//        println!("nullifier1  = {:?}",nullifier1);
//        println!("root1 = {:?}",root1);
//        println!("nullifier2  = {:?}",nullifier2);
//        println!("root2 = {:?}",root2);
//        println!("proof  = {:?}", proof);
        total += now.elapsed();

        let res = c2c_verify(nullifier1,root1,nullifier2,root2,coin,proof).unwrap();
        assert!(res);
    }
    println!("average proving time: {:?}", total / SAMPLES);
}

fn test_pedersen(){
    use zktx::pedersen_hash;
    let rng = &mut thread_rng();
    let bits = (0..PHIN).map(|_|rng.gen()).collect::<Vec<bool>>();
    let res = pedersen_hash(&bits);
    println!("PH = {:?}",res);
}

fn main(){
    test_pedersen();
    test_c2p();
    test_c2b();
    test_p2c();
    test_b2c();
    test_c2c();
}