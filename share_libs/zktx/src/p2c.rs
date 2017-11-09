use bellman::groth16::*;
use pairing::*;
use pairing::bls12_381::{Fr, FrRepr, Bls12};
use bellman::*;
use rand::{XorShiftRng,SeedableRng,thread_rng};

use jubjub::*;

use base::*;

use std::fs::File;
use std::path::Path;

struct P2Ccircuit<'a>{
    generators: &'a[(Vec<Fr>, Vec<Fr>)],
    j:& 'a JubJub,

    //r_h
    rh:Vec<Assignment<bool>>,
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
        rcm:Vec<bool>,
        ba:Fr,
        va:Fr,
        addr:Vec<bool>,
        res: &'a mut Vec<FrRepr>
    ) -> P2Ccircuit<'a>
    {
        assert_eq!(rh.len(), RHBIT);
        assert_eq!(rcm.len(), RCMBIT);
        assert_eq!(addr.len(), PHBIT);
        assert_eq!(res.len(), 0);
        P2Ccircuit{
            generators,
            j,
            rh:rh.iter().map(|&b|Assignment::known(b)).collect(),
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
    //ba*P1+rh*P2
    hb:(Num<Bls12>,Num<Bls12>),
    //coin
    coin:Num<Bls12>,
    //delta_balance,
    delt_ba:(Num<Bls12>, Num<Bls12>)
}

impl<'a> Input<Bls12> for P2CcircuitInput{
    fn synthesize<CS:PublicConstraintSystem<Bls12>>(self,cs:&mut CS)->Result<(),Error>{
        let delt_x_input = cs.alloc_input(||{
            Ok(*self.delt_ba.0.getvalue().get()?)
        })?;
        let delt_y_input = cs.alloc_input(||{
            Ok(*self.delt_ba.1.getvalue().get()?)
        })?;
        let hb_x_input = cs.alloc_input(||{
            Ok(*self.hb.0.getvalue().get()?)
        })?;
        let hb_y_input = cs.alloc_input(||{
            Ok(*self.hb.1.getvalue().get()?)
        })?;
        let coin_input = cs.alloc_input(||{
            Ok(*self.coin.getvalue().get()?)
        })?;

        cs.enforce(
            LinearCombination::zero() + self.delt_ba.0.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + delt_x_input
        );
        cs.enforce(
            LinearCombination::zero() + self.delt_ba.1.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + delt_y_input
        );
        cs.enforce(
            LinearCombination::zero() + self.hb.0.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + hb_x_input
        );cs.enforce(
            LinearCombination::zero() + self.hb.1.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + hb_y_input
        );
        cs.enforce(
            LinearCombination::zero() + self.coin.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + coin_input
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
        let bit_ba = ba.unpack_sized(cs, VBIT)?;
        let bit_va = va.unpack_sized(cs, VBIT)?;
        assert_eq!(bit_ba.len(), VBIT);
        assert_eq!(bit_va.len(), VBIT);

        assert_nonless_than(&bit_ba,&bit_va,cs)?;

        //ba*P1+rh*P2
        let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);//TODO:choose the seed
        let j = JubJub::new();
        let mut p1 = Point::rand(&mut rng, &j).toNum(cs)?;
        let mut p2 = Point::rand(&mut rng, &j).toNum(cs)?;
        let mut p0 = Point::zero().toNum(cs)?;
        for i in 0..VBIT{
            p0 = Point::pointAdd(&p0,&Point::pointChoose(&p1,bit_ba[i],cs)?,cs)?;
            if i!=VBIT-1 {
                p1 = Point::pointDouble(&p1,cs)?;
            }
        }
        for i in 0..RHBIT{
            p0 = Point::pointAdd(&p0,&Point::pointChoose(&p2,rh[i],cs)?,cs)?;
            if i!=RCMBIT-1 {
                p2 = Point::pointDouble(&p2,cs)?;
            }
        }
        if let (Ok(x),Ok(y)) = (p0.0.getvalue().get(),p0.1.getvalue().get()){
            self.res.push(x.into_repr());
            self.res.push(y.into_repr());
        }
        let hb = (p0.0,p0.1);

        //coin = PH(addr|value|rcm)
        let rcm2 = rcm.clone();
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

        //delta_ba
        let rcm = rcm2;
        let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);//TODO:choose the seed
        let j = JubJub::new();
        let mut p1 = Point::rand(&mut rng, &j).toNum(cs)?;
        let mut p2 = Point::rand(&mut rng, &j).toNum(cs)?;
        let mut p0 = Point::zero().toNum(cs)?;
        for i in 0..VBIT{
            p0 = Point::pointAdd(&p0,&Point::pointChoose(&p1,bit_va[i],cs)?,cs)?;
            if i!=VBIT-1 {
                p1 = Point::pointDouble(&p1,cs)?;
            }
        }
        for i in 0..RCMBIT{
            p0 = Point::pointAdd(&p0,&Point::pointChoose(&p2,rcm[i],cs)?,cs)?;
            if i!=RCMBIT-1 {
                p2 = Point::pointDouble(&p2,cs)?;
            }
        }
        if let (Ok(x),Ok(y)) = (p0.0.getvalue().get(),p0.1.getvalue().get()){
            self.res.push(x.into_repr());
            self.res.push(y.into_repr());
        }
        let delt_ba = (p0.0,p0.1);

        Ok(P2CcircuitInput{
            hb,
            coin,
            delt_ba
        })
    }
}

pub fn p2c_info(rh:Vec<bool>,rcm:Vec<bool>,ba:&str,va:&str,addr:Vec<bool>)->Result<(
    (([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
    ([u64;4],[u64;4]),[u64;4],([u64;4],[u64;4])),Error>{
    let rng = &mut thread_rng();
    let j = JubJub::new();
    //TODO:Balance&value<2^vbit
    let mut res: Vec<FrRepr> = vec![];
    let proof = create_random_proof::<Bls12, _, _, _>(P2Ccircuit::new(
        &ph_generator(),
        &j,
        rh,
        rcm,
        Fr::from_str(ba).unwrap(),
        Fr::from_str(va).unwrap(),
        addr,
        &mut res
    ), p2c_param()?, rng)?.serial();
    let hb = (res[0].serial(),res[1].serial());
    let coin = res[2].serial();
    let delt_ba = (res[3].serial(),res[4].serial());
    Ok((proof,hb,coin,delt_ba))
}

pub fn p2c_verify(hb:([u64;4],[u64;4]), coin:[u64;4], delt_ba:([u64;4],[u64;4]),
                  proof:(([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
                  )->Result<bool,Error>{
    verify_proof(&p2c_vk()?, &Proof::from_serial(proof), |cs| {
        let delt_x = Fr::from_repr(FrRepr::from_serial(delt_ba.0)).unwrap();
        let delt_y = Fr::from_repr(FrRepr::from_serial(delt_ba.1)).unwrap();
        let hb_x = Fr::from_repr(FrRepr::from_serial(hb.0)).unwrap();
        let hb_y = Fr::from_repr(FrRepr::from_serial(hb.1)).unwrap();
        let coin = Fr::from_repr(FrRepr::from_serial(coin)).unwrap();
        let delt_x_var = cs.alloc({||Ok(delt_x)})?;
        let delt_y_var = cs.alloc({||Ok(delt_y)})?;
        let hb_x_var = cs.alloc({||Ok(hb_x)})?;
        let hb_y_var = cs.alloc({||Ok(hb_y)})?;
        let coin_var = cs.alloc({||Ok(coin)})?;
        Ok(P2CcircuitInput{
            hb:(Num::create(Assignment::known(hb_x),hb_x_var),Num::create(Assignment::known(hb_y),hb_y_var)),
            coin:Num::create(Assignment::known(coin),coin_var),
            delt_ba:(Num::create(Assignment::known(delt_x),delt_x_var),Num::create(Assignment::known(delt_y),delt_y_var)),
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
    let vk2 = params.get_vk(6)?;
    let vk = prepare_verifying_key(&vk2);
    Ok(vk)
}