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
    rcm:Assignment<Fr>,
    //value
    va:Assignment<Fr>,
    //addr
    addr:(Assignment<Fr>,Assignment<Fr>),
    //random number,
    random:Assignment<Fr>,
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
            rcm:Assignment::unknown(),
            va:Assignment::unknown(),
            addr:(Assignment::unknown(),Assignment::unknown()),
            random:Assignment::unknown(),
            res
        }
    }

    fn new(
        generators: &'a[(Vec<Fr>,Vec<Fr>)],
        j:&'a JubJub,
        rcm:Fr,
        va:Fr,
        addr:(Fr,Fr),
        random:Fr,
        res: &'a mut Vec<FrRepr>
    )->B2Ccircuit<'a>{
        assert_eq!(res.len(), 0);
        B2Ccircuit{
            generators,
            j,
            rcm:Assignment::known(rcm),
            va:Assignment::known(va),
            addr:(Assignment::known(addr.0),Assignment::known(addr.1)),
            random:Assignment::known(random),
            res
        }
    }
}

struct B2CcircuitInput{
    //value
    va:Num<Bls12>,
    //coin
    coin:Num<Bls12>,
    //rP
    rp:(Num<Bls12>, Num<Bls12>),
    //enc
    enc:Num<Bls12>
}

impl<'a> Input<Bls12> for B2CcircuitInput{
    fn synthesize<CS:PublicConstraintSystem<Bls12>>(self,cs:&mut CS)->Result<(),Error>{
        let coin_input = cs.alloc_input(||{
            Ok(*self.coin.getvalue().get()?)
        })?;
        let va_input = cs.alloc_input(||{
            Ok(*self.va.getvalue().get()?)
        })?;
        let rpx_input = cs.alloc_input(||{
            Ok(*self.rp.0.getvalue().get()?)
        })?;
        let rpy_input = cs.alloc_input(||{
            Ok(*self.rp.1.getvalue().get()?)
        })?;
        let enc_input = cs.alloc_input(||{
            Ok(*self.enc.getvalue().get()?)
        })?;

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
        cs.enforce(
            LinearCombination::zero() + self.rp.0.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + rpx_input
        );
        cs.enforce(
            LinearCombination::zero() + self.rp.1.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + rpy_input
        );
        cs.enforce(
            LinearCombination::zero() + self.enc.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + enc_input
        );

        Ok(())
    }
}

impl<'a> Circuit<Bls12> for B2Ccircuit<'a>{
    type InputMap = B2CcircuitInput;

    fn synthesize<CS:ConstraintSystem<Bls12>>(self,cs:&mut CS)->Result<Self::InputMap,Error>{
        let rcm_num = Num::new(cs,self.rcm)?;
        let mut rcm = rcm_num.unpack_sized(cs,RCMBIT)?;
        let random_num = Num::new(cs,self.random)?;
        let random = random_num.unpack_sized(cs,256)?;
        let addr_x_num = Num::new(cs,self.addr.0)?;
        let addr_x_bit = addr_x_num.unpack_sized(cs, PHOUT)?;
        let addr_y_num = Num::new(cs,self.addr.1)?;

        let va = Num::new(cs,self.va)?;
        let bit_va = va.unpack_sized(cs, VBIT)?;
        assert_eq!(bit_va.len(), VBIT);

        //prepare table
        let p1 = Point::enc_point_table(256, 1, cs)?;

        //coin = PH(addr|value|rcm)
        let vin = {
            for b in bit_va.iter(){
                rcm.push(*b);
            }
            for b in addr_x_bit.iter(){
                rcm.push(*b);
            }
            rcm
        };
        assert_eq!(vin.len(), PHIN);
        let coin = pedersen_hash(cs, &vin, self.generators, self.j)?;
        if let Ok(x) = coin.getvalue().get(){
            self.res.push(x.into_repr());
        }

        //Enc
        let message = {
            let b128 = Num::new(cs,Assignment::known(Fr::from_repr(FrRepr::from_serial([0,0,1,0])).unwrap()))?;
            va.mul(cs,&b128)?.add(cs,&rcm_num)
        }?;
        let qtable = Point::point_mul_table((&addr_x_num, &addr_y_num), 256, cs)?;
        let rp = Point::multiply(&p1,&random,cs)?;
        let rq = Point::multiply(&qtable,&random,cs)?;
        if let (Ok(x),Ok(y)) = (rp.0.getvalue().get(),rp.1.getvalue().get()){
            self.res.push(x.into_repr());
            self.res.push(y.into_repr());
        }
        let key = rq.0;
        let enc = key.add(cs,&message)?;
        if let Ok(x) = enc.getvalue().get(){
            self.res.push(x.into_repr());
        }

        Ok(B2CcircuitInput{
            va,
            coin,
            rp,
            enc
        })
    }
}

pub fn b2c_info(rcm:[u64;2],va:[u64;2],addr:([u64;4],[u64;4]),enc_random:[u64;4])->Result<(
    (([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
    [u64;4],([u64;4],[u64;4]),[u64;4]),Error>{
    let rng = &mut thread_rng();
    let j = JubJub::new();
    let mut res: Vec<FrRepr> = vec![];
    let proof = create_random_proof::<Bls12, _, _, _>(B2Ccircuit::new(
        &ph_generator(),
        &j,
        Fr::from_repr(FrRepr([rcm[0],rcm[1],0,0])).unwrap(),
        Fr::from_repr(FrRepr([va[0],va[1],0,0])).unwrap(),
        (Fr::from_repr(FrRepr(addr.0)).unwrap(),Fr::from_repr(FrRepr(addr.1)).unwrap()),
        Fr::from_serial(enc_random),
        &mut res
    ), b2c_param()?, rng)?.serial();
    let coin = res[0].serial();
    let rp = (res[1].serial(),res[2].serial());
    let enc = res[3].serial();
    Ok((proof,coin,rp,enc))
}

pub fn b2c_verify(va:[u64;2], coin:[u64;4],rp:([u64;4],[u64;4]),enc:[u64;4],
                  proof:(([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
                  )->Result<bool,Error>{
    verify_proof(&b2c_vk()?, &Proof::from_serial(proof), |cs| {
        let coin = Fr::from_repr(FrRepr::from_serial(coin)).unwrap();
        let va = Fr::from_repr(FrRepr([va[0],va[1],0,0])).unwrap();
        let enc = Fr::from_repr(FrRepr::from_serial(enc)).unwrap();
        let rpx = Fr::from_repr(FrRepr::from_serial(rp.0)).unwrap();
        let rpy = Fr::from_repr(FrRepr::from_serial(rp.1)).unwrap();
        Ok(B2CcircuitInput{
            coin:Num::new(cs,Assignment::known(coin))?,
            va:Num::new(cs,Assignment::known(va))?,
            rp:(Num::new(cs, Assignment::known(rpx))?, Num::new(cs, Assignment::known(rpy))?),
            enc:Num::new(cs,Assignment::known(enc))?
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
    let vk2 = params.get_vk(6)?;
    let vk = prepare_verifying_key(&vk2);
    Ok(vk)
}