use pairing::bls12_381::{Fr,FrRepr};
use pairing::{Field,PrimeField};
use rand::{XorShiftRng, SeedableRng};

use jubjub::*;

pub const VBIT:usize = 128;
pub const RHBIT:usize = 256;
pub const RCMBIT:usize = 128;
pub const PHOUT:usize = 256;
pub const PHIN:usize = 512;
pub const ADSK:usize = 256;
pub const TREEDEPTH:usize = 60;
pub const C2BPARAMPATH:&str = "PARAMS/c2bparams";
pub const P2CPARAMPATH:&str = "PARAMS/p2cparams";
pub const B2CPARAMPATH:&str = "PARAMS/b2cparams";
pub const C2PPARAMPATH:&str = "PARAMS/c2pparams";
pub const GENERATORPATH:&str = "PARAMS/generators";

pub fn ph_generator()->Vec<(Vec<Fr>,Vec<Fr>)>{
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    if !Path::new(GENERATORPATH).exists()
    {
        println!("Creating the pedersen hash generators");
        const SEED:[u32;4] = [0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654];
        let mut generator_rng = XorShiftRng::from_seed(SEED);
        let generators = generate_constant_table(&mut generator_rng, &JubJub::new());
        drop(generator_rng);

        let mut writer = File::create(GENERATORPATH).unwrap();

        for tup in generators.iter(){
            match tup{
                &(ref frxs,ref frys)=>{
                    for x in frxs.iter(){
                        for unit in x.serial().iter(){
                            writer.write_all(&u64to8(*unit)).unwrap();
                        }
                    }
                    for y in frys.iter(){
                        for unit in y.serial().iter(){
                            writer.write_all(&u64to8(*unit)).unwrap();
                        }
                    }
                }
            }
        }
        println!("Just wrote the generators to disk!");
        return generators
    }

    let mut reader = File::open(GENERATORPATH).unwrap();

    let mut serial = vec![];
    for _ in 0..128{
        let mut xs = vec![];
        let mut ys = vec![];
        for _ in 0..16{
            let mut nums:[u64;4]=[0;4];
            for i in 0..4{
                let mut num:[u8;8]=[0;8];
                reader.read(&mut num).unwrap();
                nums[i] = u8to64(num);
            }
            xs.push(Fr::from_serial(nums));
        }
        for _ in 0..16{
            let mut nums:[u64;4]=[0;4];
            for i in 0..4{
                let mut num:[u8;8]=[0;8];
                reader.read(&mut num).unwrap();
                nums[i] = u8to64(num);
            }
            ys.push(Fr::from_serial(nums));
        }
        serial.push((xs,ys));
    }
    serial
}

#[inline(always)]
fn u64to8(mut num: u64)->[u8;8]{
    let mut out:[u8;8] = [0;8];
    for i in 0..8{
        out[i] = (num & 0b11111111) as u8;
        num >>= 8;
    }
    out
}

#[inline(always)]
fn u8to64(nums:[u8;8])->u64{
    let mut res:u64 = 0;
    for i in 0..8{
        res <<=8;
        res |= nums[7-i] as u64;
    }
    res
}

pub fn address(addr_sk:&Vec<bool>) ->([u64;4], [u64;4]){
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);//TODO:choose the seed
    let j = JubJub::new();
    let (mut xp,mut yp) = Point::rand(&mut rng, &j).coordinate();
    let mut x0 = Fr::zero();
    let mut y0 = Fr::one();

    for i in 0..addr_sk.len(){
        if addr_sk[i] {
            let res = point_add(&x0, &y0, &xp, &yp, &j);
            x0 = res.0;
            y0 = res.1;
        }
        if i!=addr_sk.len()-1 {
            let res = point_double(xp, yp, &j);
            xp = res.0;
            yp = res.1;
        }
    }

    (x0.into_repr().serial(),y0.into_repr().serial())
}

fn point_double(x:Fr, y:Fr, j: &JubJub) ->(Fr, Fr){
    point_add(&x, &y, &x, &y, j)
}

fn point_add(x0:&Fr, y0:&Fr, xp:&Fr, yp:&Fr, j: &JubJub) ->(Fr, Fr){
    let mut y1y2 = y0.clone();
    y1y2.mul_assign(yp);
    let mut x1x2 = x0.clone();
    x1x2.mul_assign(xp);
    let mut dx1x2y1y2 = j.d;
    dx1x2y1y2.mul_assign(&y1y2);
    dx1x2y1y2.mul_assign(&x1x2);

    let mut d1 = dx1x2y1y2;
    d1.add_assign(&Fr::one());
    d1 = d1.inverse().unwrap();

    let mut d2 = dx1x2y1y2;
    d2.negate();
    d2.add_assign(&Fr::one());
    d2 = d2.inverse().unwrap();

    let mut x1y2 = x0.clone();
    x1y2.mul_assign(yp);

    let mut y1x2 = y0.clone();
    y1x2.mul_assign(xp);

    let mut x = x1y2;
    x.add_assign(&y1x2);
    x.mul_assign(&d1);

    let mut y = y1y2;
    y.add_assign(&x1x2);
    y.mul_assign(&d2);

    (x.clone(),y.clone())
}

pub fn ecc_add(point1:([u64;4], [u64;4]), point2:([u64;4], [u64;4])) ->([u64;4], [u64;4]){
    let (xfr,yfr) = point_add(&Fr::from_serial(point1.0), &Fr::from_serial(point1.1), &Fr::from_serial(point2.0), &Fr::from_serial(point2.1), &JubJub::new());
    let x = xfr.serial();
    let y = yfr.serial();
    (x,y)
}

pub fn v_p1_add_r_p2(v:[u64;2], r:[u64;2]) ->([u64;4], [u64;4]){
    let v = {
        let mut vec = Vec::with_capacity(128);
        let mut num = v[0];
        for _ in 0..64{
            vec.push(num&1==1);
            num>>=1;
        }
        let mut num = v[1];
        for _ in 0..64{
            vec.push(num&1==1);
            num>>=1;
        }
        vec
    };
    let r = {
        let mut vec = Vec::with_capacity(128);
        let mut num = r[0];
        for _ in 0..64{
            vec.push(num&1==1);
            num>>=1;
        }
        let mut num = r[1];
        for _ in 0..64{
            vec.push(num&1==1);
            num>>=1;
        }
        vec
    };

    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);//TODO:choose the seed
    let j = JubJub::new();

    let (mut xp,mut yp) = Point::rand(&mut rng, &j).coordinate();
    let mut x0 = Fr::zero();
    let mut y0 = Fr::one();

    for i in 0..v.len(){
        if v[i] {
            let res = point_add(&x0, &y0, &xp, &yp, &j);
            x0 = res.0;
            y0 = res.1;
        }
        if i!=v.len()-1 {
            let res = point_double(xp, yp, &j);
            xp = res.0;
            yp = res.1;
        }
    }

    let (mut xp,mut yp) = Point::rand(&mut rng, &j).coordinate();
    for i in 0..r.len(){
        if r[i] {
            let res = point_add(&x0, &y0, &xp, &yp, &j);
            x0 = res.0;
            y0 = res.1;
        }
        if i!=r.len()-1 {
            let res = point_double(xp, yp, &j);
            xp = res.0;
            yp = res.1;
        }
    }

    (x0.serial(),y0.serial())
}

fn point_mul(point:([u64;4], [u64;4]), num:Vec<bool>) ->(Fr, Fr){
    let (mut xp,mut yp) = (Fr::from_repr(FrRepr::from_serial(point.0)).unwrap(),Fr::from_repr(FrRepr::from_serial(point.1)).unwrap());
    let mut x0 = Fr::zero();
    let mut y0 = Fr::one();
    let j = JubJub::new();

    for i in 0..num.len(){
        if num[i] {
            let res = point_add(&x0, &y0, &xp, &yp, &j);
            x0 = res.0;
            y0 = res.1;
        }
        if i!=num.len()-1 {
            let res = point_double(xp, yp, &j);
            xp = res.0;
            yp = res.1;
        }
    }

    (x0,y0)
}

pub fn decrypt(secret:[u64;4], rp:([u64;4], [u64;4]), sk:Vec<bool>) ->([u64;2], [u64;2]){
    let rqx = point_mul(rp, sk).0;
    let mut message = Fr::from_repr(FrRepr::from_serial(secret)).unwrap();
    message.sub_assign(&rqx);
    let message = message.into_repr().serial();
    let va = [message[2],message[3]];
    let rcm = [message[0],message[1]];
    (va,rcm)
}