extern crate rand;
extern crate zktx;

use rand::{Rng, thread_rng};
use zktx::base::*;

fn test_b2c(samples:u32) {
    println!("test_b2c");
    use zktx::b2c::*;

    ensure_b2c_param().unwrap();

    use std::time::{Duration, Instant};
    let mut total = Duration::new(0, 0);

    println!("Creating {} proofs and averaging the time spent creating them.", samples);

    for _ in 0..samples {
        let now = Instant::now();
        let rng = &mut thread_rng();
        let rcm = [rng.gen(),rng.gen()];
        let addr = ([rng.gen(),rng.gen(),rng.gen(),0],[rng.gen(),rng.gen(),rng.gen(),0]);
        let random:[u64;4] = [rng.gen(),rng.gen(),rng.gen(),rng.gen()];
        let va:[u64;2] = [10,0];
        let (proof, coin,rp,enc) = b2c_info(rcm, va, addr,random).unwrap();
//        println!("H_B   = {:?}", bn);
//        println!("coin  = {:?}", coin);
//        println!("proof  = {:?}", proof);
        total += now.elapsed();

        let res = b2c_verify(va, coin, rp,enc,proof).unwrap();
        assert!(res);
    }
    println!("average proving time: {:?}", total / samples);
}

fn test_c2b(samples:u32){
    println!("test_c2b");
    use zktx::c2b::*;

    ensure_c2b_param().unwrap();

    use std::time::{Duration,Instant};
    let mut total = Duration::new(0, 0);

    println!("Creating {} proofs and averaging the time spent creating them.", samples);

    for _ in 0..samples{
        let now = Instant::now();
        let rng = &mut thread_rng();
        let rcm :[u64;2]= [rng.gen(),rng.gen()];
        let addr_sk = (0..ADSK).map(|_| rng.gen()).collect::<Vec<bool>>();
        let ba :[u64;2]= [1000,0];
        let va :[u64;2]= [10,0];
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
    println!("average proving time: {:?}", total / samples);
}

fn test_c2p(samples:u32){
    println!("test_c2p");
    use zktx::c2p::*;
    use zktx::{pedersen_hash,pedersen_hash_root};

    ensure_c2p_param().unwrap();

    use std::time::{Duration,Instant};
    let mut total = Duration::new(0, 0);

    println!("Creating {} proofs and averaging the time spent creating them.", samples);

    for _ in 0..samples{
        let now = Instant::now();
        //倒序：359=101100111 -> [1,1,1,0,0,1,1,0,1]
        let rng = &mut thread_rng();
        let rcm :[u64;2]= [rng.gen(),rng.gen()];
        let addr_sk = (0..ADSK).map(|_| rng.gen()).collect::<Vec<bool>>();
        let va :[u64;2]= [10,0];
        let path:Vec<[u64;4]> = (0..TREEDEPTH).map(|_| {
            let mut v:[u64;4] = [0;4];
            for i in 0..4{
                v[i] = rng.gen();
            }
            v
        }).collect();
        let locs:Vec<bool> = (0..TREEDEPTH).map(|_| rng.gen()).collect::<Vec<bool>>();
        let coin = pedersen_hash({
            let addr = address(&addr_sk).0;
            let mut v = Vec::with_capacity(256);
            for num in addr.into_iter(){
                let mut num = *num;
                for _ in 0..64{
                    v.push(num&1==1);
                    num>>=1;
                }
            }
            let addr = v;
            let mut node = Vec::with_capacity(256);
            for num in rcm.into_iter(){
                let mut num = *num;
                for _ in 0..64{
                    node.push(num&1==1);
                    num>>=1;
                }
            }
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
        let (proof,nullifier,root,delt_ba) = c2p_info(rcm,va,addr_sk,path,locs).unwrap();
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

        let res = c2p_verify(nullifier,root,delt_ba,proof).unwrap();
        assert!(res);
    }
    println!("average proving time: {:?}", total / samples);
}

fn test_p2c(samples:u32){
    println!("test_p2c");
    use zktx::p2c::*;

    ensure_p2c_param().unwrap();

    use std::time::{Duration,Instant};
    let mut total = Duration::new(0, 0);

    println!("Creating {} proofs and averaging the time spent creating them.", samples);

    for _ in 0..samples{
        let now = Instant::now();
        //倒序：359=101100111 -> [1,1,1,0,0,1,1,0,1]
        let rng = &mut thread_rng();
        let rh:[u64;4] = [rng.gen(),rng.gen(),rng.gen(),0];
        let rcm :[u64;2]= [rng.gen(),rng.gen()];
        let addr = ([rng.gen(),rng.gen(),rng.gen(),0],[rng.gen(),rng.gen(),rng.gen(),0]);
        let random:[u64;4] = [rng.gen(),rng.gen(),rng.gen(),rng.gen()];
        let ba :[u64;2]= [1000,0];
        let va :[u64;2]= [10,0];
        let (proof,hb,coin,delt_ba,rp,enc) = p2c_info(rh,rcm,ba,va,addr,random).unwrap();
//        println!("H_B   = {:?}",hb);
//        println!("coin  = {:?}",coin);
//        println!("H_B-n = {:?}",hbn);
//        println!("proof  = {:?}", proof);
        total += now.elapsed();

        let res = p2c_verify(hb,coin,delt_ba,rp,enc,proof).unwrap();
        assert!(res);
    }
    println!("average proving time: {:?}", total / samples);
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
    test_b2c(10);
    test_p2c(10);
    test_c2b(5);
    test_c2p(5);
}