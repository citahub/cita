use bellman::groth16::*;
use pairing::*;
use pairing::bls12_381::{Fr, FrRepr, Bls12};
use bellman::*;
use rand::thread_rng;

use jubjub::*;

use base::*;

use std::fs::File;
use std::path::Path;

struct B2Ccircuit<'a>{
    generators:&'a [(Vec<Fr>,Vec<Fr>)],
    j:&'a JubJub,

    //r_cm
    rcm:Vec<Assignment<bool>>,
    //Balance
    ba:Assignment<Fr>,
    //value
    va:Assignment<Fr>,
    //addr
    addr:Vec<Assignment<bool>>,
    //result
    res: &'a mut Vec<FrRepr>
}

impl<'a> B2Ccircuit<'a>{
    fn blank(
        generators:&'a [(Vec<Fr>,Vec<Fr>)],
        j:&'a JubJub,
        res:&'a mut Vec<FrRepr>
    )->B2Ccircuit<'a>{
        B2Ccircuit{
            generators,
            j,
            rcm: (0..RCMBIT).map(|_| Assignment::unknown()).collect(),
            ba:Assignment::unknown(),
            va:Assignment::unknown(),
            addr: (0..PHBIT).map(|_| Assignment::unknown()).collect(),
            res
        }
    }

    fn new(
        generators: &'a[(Vec<Fr>,Vec<Fr>)],
        j:&'a JubJub,
        rcm:Vec<bool>,
        ba:Fr,
        va:Fr,
        addr:Vec<bool>,
        res: &'a mut Vec<FrRepr>
    )->B2Ccircuit<'a>{
        assert_eq!(rcm.len(), RCMBIT);
        assert_eq!(addr.len(), PHBIT);
        assert_eq!(res.len(), 0);
        B2Ccircuit{
            generators,
            j,
            rcm:rcm.iter().map(|&b|Assignment::known(b)).collect(),
            ba:Assignment::known(ba),
            va:Assignment::known(va),
            addr:addr.iter().map(|&b|Assignment::known(b)).collect(),
            res
        }
    }
}

struct B2CcircuitInput{
    //Balance
    ba:Num<Bls12>,
    //value
    va:Num<Bls12>,
    //coin
    coin:Num<Bls12>
}

impl<'a> Input<Bls12> for B2CcircuitInput{
    fn synthesize<CS:PublicConstraintSystem<Bls12>>(self,cs:&mut CS)->Result<(),Error>{
        let ba_input = cs.alloc_input(||{
            Ok(*self.ba.getvalue().get()?)
        })?;
        let coin_input = cs.alloc_input(||{
            Ok(*self.coin.getvalue().get()?)
        })?;
        let va_input = cs.alloc_input(||{
            Ok(*self.va.getvalue().get()?)
        })?;

        cs.enforce(
            LinearCombination::zero() + self.ba.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + ba_input
        );
        cs.enforce(
            LinearCombination::zero() + self.coin.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + coin_input
        );
        cs.enforce(
            LinearCombination::zero() + self.va.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + va_input
        );

        Ok(())
    }
}

impl<'a> Circuit<Bls12> for B2Ccircuit<'a>{
    type InputMap = B2CcircuitInput;

    fn synthesize<CS:ConstraintSystem<Bls12>>(self,cs:&mut CS)->Result<Self::InputMap,Error>{
        let mut rcm = Vec::with_capacity(RCMBIT);
        for b in self.rcm.iter() {
            rcm.push(Bit::alloc(cs, *b)?);
        }
        let mut addr = Vec::with_capacity(PHBIT);
        for b in self.addr.iter() {
            addr.push(Bit::alloc(cs, *b)?);
        }

        let ba = Num::new(cs,self.ba)?;
        let va = Num::new(cs,self.va)?;
        let bn = ba.sub(cs,&va)?;
        let bit_ba = ba.unpack_sized(cs, VBIT)?;
        let bit_va = va.unpack_sized(cs, VBIT)?;
        let bit_bn = bn.unpack_sized(cs, VBIT)?;
        assert_eq!(bit_ba.len(), VBIT);
        assert_eq!(bit_va.len(), VBIT);
        assert_eq!(bit_bn.len(), VBIT);

        assert_nonless_than(&bit_ba,&bit_va,cs)?;

        if let Ok(x) = bn.getvalue().get(){
            self.res.push(x.into_repr());
        }

        //coin = PH(addr|value|rcm)
        let vin = {
            for b in bit_va.iter(){
                rcm.push(*b);
            }
            for b in addr.iter(){
                rcm.push(*b);
            }
            rcm
        };
        assert_eq!(vin.len(), PHIN);
        let coin = pedersen_hash(cs, &vin, self.generators, self.j)?;
        if let Ok(x) = coin.getvalue().get(){
            self.res.push(x.into_repr());
        }

        Ok(B2CcircuitInput{
            ba,
            va,
            coin
        })
    }
}

pub fn b2c_info(rcm:Vec<bool>,ba:&str,va:&str,addr:Vec<bool>)->Result<(
    (([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
    [u64;4],[u64;4]),Error>{
    let rng = &mut thread_rng();
    let j = JubJub::new();
    let mut res: Vec<FrRepr> = vec![];
    let proof = create_random_proof::<Bls12, _, _, _>(B2Ccircuit::new(
        &ph_generator(),
        &j,
        rcm,
        Fr::from_str(ba).unwrap(),
        Fr::from_str(va).unwrap(),
        addr,
        &mut res
    ), b2c_param()?, rng)?.serial();
    let bn = res[0].serial();
    let coin = res[1].serial();
    Ok((proof,bn,coin))
}

pub fn b2c_verify(ba:&str, va:&str, coin:[u64;4],
                  proof:(([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
                  )->Result<bool,Error>{
    verify_proof(&b2c_vk()?, &Proof::from_serial(proof), |cs| {
        let ba = Fr::from_str(ba).unwrap();
        let coin = Fr::from_repr(FrRepr::from_serial(coin)).unwrap();
        let va = Fr::from_str(va).unwrap();
        let ba_var = cs.alloc({||Ok(ba)})?;
        let coin_var = cs.alloc({||Ok(coin)})?;
        let va_var = cs.alloc({||Ok(va)})?;
        Ok(B2CcircuitInput{
            ba:Num::create(Assignment::known(ba),ba_var),
            coin:Num::create(Assignment::known(coin),coin_var),
            va:Num::create(Assignment::known(va),va_var)
        })
    })
}

pub fn ensure_b2c_param() ->Result<(),Error>{
    if !Path::new(B2CPARAMPATH).exists(){
        println!("Creating the parameters");
        let rng = &mut thread_rng();
        let params = generate_random_parameters::<Bls12, _, _>(B2Ccircuit::blank(
            &ph_generator(),
            &JubJub::new(),
            &mut vec![]
        ), rng)?;
        params.write(&mut File::create(B2CPARAMPATH).unwrap()).unwrap();
        println!("Just wrote the parameters to disk!");
    }
    Ok(())
}

fn b2c_param()->Result<ProverStream,Error>{
    ensure_b2c_param()?;
    let params = ProverStream::new(B2CPARAMPATH).unwrap();
    Ok(params)
}

fn b2c_vk()->Result<(PreparedVerifyingKey<Bls12>),Error>{
    ensure_b2c_param()?;
    let mut params = ProverStream::new(B2CPARAMPATH)?;
    let vk2 = params.get_vk(4)?;
    let vk = prepare_verifying_key(&vk2);
    Ok(vk)
}