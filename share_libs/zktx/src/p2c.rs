use bellman::groth16::*;
use pairing::*;
use pairing::bls12_381::{Fr, FrRepr, Bls12};
use bellman::*;
use rand::thread_rng;

use jubjub::*;

use base::*;

use std::fs::File;
use std::path::Path;

struct P2Ccircuit<'a>{
    generators: &'a[(Vec<Fr>, Vec<Fr>)],
    j:& 'a JubJub,

    //r_h
    rh:Vec<Assignment<bool>>,
    //r_h-next
    rhn:Vec<Assignment<bool>>,
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

impl<'a> P2Ccircuit<'a>{
    fn blank(
        generators: &'a[(Vec<Fr>,Vec<Fr>)],
        j:&'a JubJub,
        res: &'a mut Vec<FrRepr>
    ) -> P2Ccircuit<'a>{
        P2Ccircuit{
            generators,
            j,
            rh: (0..RHBIT).map(|_| Assignment::unknown()).collect(),
            rhn: (0..RHBIT).map(|_| Assignment::unknown()).collect(),
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
        rh:Vec<bool>,
        rhn:Vec<bool>,
        rcm:Vec<bool>,
        ba:Fr,
        va:Fr,
        addr:Vec<bool>,
        res: &'a mut Vec<FrRepr>
    ) -> P2Ccircuit<'a>
    {
        assert_eq!(rh.len(), RHBIT);
        assert_eq!(rhn.len(), RHBIT);
        assert_eq!(rcm.len(), RCMBIT);
        assert_eq!(addr.len(), PHBIT);
        assert_eq!(res.len(), 0);
        P2Ccircuit{
            generators,
            j,
            rh:rh.iter().map(|&b|Assignment::known(b)).collect(),
            rhn:rhn.iter().map(|&b|Assignment::known(b)).collect(),
            rcm:rcm.iter().map(|&b|Assignment::known(b)).collect(),
            ba:Assignment::known(ba),
            va:Assignment::known(va),
            addr:addr.iter().map(|&b|Assignment::known(b)).collect(),
            res
        }
    }
}

struct P2CcircuitInput
{
    //H_B
    hb:Num<Bls12>,
    //H_B-next
    hbn:Num<Bls12>,
    //coin
    coin:Num<Bls12>
}

impl<'a> Input<Bls12> for P2CcircuitInput{
    fn synthesize<CS:PublicConstraintSystem<Bls12>>(self,cs:&mut CS)->Result<(),Error>{
        let hb_input = cs.alloc_input(||{
            Ok(*self.hb.getvalue().get()?)
        })?;
        let coin_input = cs.alloc_input(||{
            Ok(*self.coin.getvalue().get()?)
        })?;
        let hbn_input = cs.alloc_input(||{
            Ok(*self.hbn.getvalue().get()?)
        })?;

        cs.enforce(
            LinearCombination::zero() + self.hb.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + hb_input
        );
        cs.enforce(
            LinearCombination::zero() + self.coin.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + coin_input
        );
        cs.enforce(
            LinearCombination::zero() + self.hbn.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + hbn_input
        );

        Ok(())
    }
}

impl<'a> Circuit<Bls12> for P2Ccircuit<'a>{
    type InputMap = P2CcircuitInput;

    fn synthesize<CS:ConstraintSystem<Bls12>>(self,cs:&mut CS)->Result<Self::InputMap,Error>{
        let mut rh = Vec::with_capacity(RHBIT);
        for b in self.rh.iter() {
            rh.push(Bit::alloc(cs, *b)?);
        }
        let mut rhn = Vec::with_capacity(RHBIT);
        for b in self.rhn.iter() {
            rhn.push(Bit::alloc(cs, *b)?);
        }
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

        let zero = PHIN - VBIT - RHBIT;
        //H_B = PH(1*zeros|Balance|r_h)
        let vin = {
            for b in bit_ba.iter() {
                rh.push(*b);
            }
            for _ in 0..zero {
                rh.push(Bit::one(cs));
            }
            rh
        };
        assert_eq!(vin.len(), PHIN);
        let hb = pedersen_hash(cs, &vin, self.generators, self.j)?;

        if let Ok(x) = hb.getvalue().get(){
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
            //            rcm.push(Bit::one(cs));
            rcm
        };
        assert_eq!(vin.len(), PHIN);
        let coin = pedersen_hash(cs, &vin, self.generators, self.j)?;
        if let Ok(x) = coin.getvalue().get(){
            self.res.push(x.into_repr());
        }

        let zero = PHIN - VBIT - RHBIT;
        //H_b-next = PH(1*zeros|B-next|r_h-next)
        let vin = {
            for b in bit_bn.iter(){
                rhn.push(*b);
            }
            for _ in 0..zero{
                rhn.push(Bit::one(cs));
            }
            rhn
        };
        assert_eq!(vin.len(), PHIN);
        let hbn = pedersen_hash(cs, &vin, self.generators, self.j)?;
        if let Ok(x) = hbn.getvalue().get(){
            self.res.push(x.into_repr());
        }

        Ok(P2CcircuitInput{
            hb,
            coin,
            hbn
        })
    }
}

pub fn p2c_info(rh:Vec<bool>,rhn:Vec<bool>,rcm:Vec<bool>,ba:&str,va:&str,addr:Vec<bool>)->Result<(
    (([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
    [u64;4],[u64;4],[u64;4]),Error>{
    let rng = &mut thread_rng();
    let j = JubJub::new();
    //TODO:Balance&value<2^vbit
    let mut res: Vec<FrRepr> = vec![];
    let proof = create_random_proof::<Bls12, _, _, _>(P2Ccircuit::new(
        &ph_generator(),
        &j,
        rh,
        rhn,
        rcm,
        Fr::from_str(ba).unwrap(),
        Fr::from_str(va).unwrap(),
        addr,
        &mut res
    ), p2c_param()?, rng)?.serial();
    let hb = res[0].serial();
    let coin = res[1].serial();
    let hbn = res[2].serial();
    Ok((proof,hb,coin,hbn))
}

pub fn p2c_verify(hb:[u64;4], coin:[u64;4], hbn:[u64;4],
                  proof:(([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
                  )->Result<bool,Error>{
    verify_proof(&p2c_vk()?, &Proof::from_serial(proof), |cs| {
        let hb = Fr::from_repr(FrRepr::from_serial(hb)).unwrap();
        let coin = Fr::from_repr(FrRepr::from_serial(coin)).unwrap();
        let hbn = Fr::from_repr(FrRepr::from_serial(hbn)).unwrap();
        let hb_var = cs.alloc({||Ok(hb)})?;
        let coin_var = cs.alloc({||Ok(coin)})?;
        let hbn_var = cs.alloc({||Ok(hbn)})?;
        Ok(P2CcircuitInput{
            hb:Num::create(Assignment::known(hb),hb_var),
            coin:Num::create(Assignment::known(coin),coin_var),
            hbn:Num::create(Assignment::known(hbn),hbn_var)
        })
    })
}

pub fn ensure_p2c_param() ->Result<(),Error>{
    if !Path::new(P2CPARAMPATH).exists(){
        println!("Creating the parameters");
        let rng = &mut thread_rng();
        let params = generate_random_parameters::<Bls12, _, _>(P2Ccircuit::blank(
            &ph_generator(),
            &JubJub::new(),
            &mut vec![]
        ), rng)?;
        params.write(&mut File::create(P2CPARAMPATH).unwrap()).unwrap();
        println!("Just wrote the parameters to disk!");
    }
    Ok(())
}

pub fn p2c_param()->Result<ProverStream,Error>{
    ensure_p2c_param()?;
    let params = ProverStream::new(P2CPARAMPATH).unwrap();
    Ok(params)
}

pub fn p2c_vk()->Result<(PreparedVerifyingKey<Bls12>),Error>{
    ensure_p2c_param()?;
    let mut params = ProverStream::new(P2CPARAMPATH)?;
    let vk2 = params.get_vk(4)?;
    let vk = prepare_verifying_key(&vk2);
    Ok(vk)
}