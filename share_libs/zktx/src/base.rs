use pairing::bls12_381::Fr;
use rand::{XorShiftRng, SeedableRng};

use jubjub::*;

pub const VBIT:usize = 128;
pub const RHBIT:usize = 256;
pub const RCMBIT:usize = 128;
pub const PHBIT:usize = 256;
pub const PHIN:usize = 512;
pub const ADSK:usize = 256;
pub const TREEDEPTH:usize = 40;
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