use bellman::groth16::*;
use pairing::*;
use pairing::bls12_381::{Fr, FrRepr, Bls12};
use bellman::*;
use rand::thread_rng;

use jubjub::*;

use base::*;

use std::fs::File;
use std::path::Path;

struct C2Ccircuit<'a>{
    generators: &'a[(Vec<Fr>, Vec<Fr>)],
    j:& 'a JubJub,
    //r_cm1
    rcm1:Vec<Assignment<bool>>,
    //va1
    va1:Assignment<Fr>,
    //addr_sk1
    addr_sk1:Vec<Assignment<bool>>,
    //coin path 1
    path1:Vec<Vec<Assignment<bool>>>,
    //path location 1
    loc1:Vec<Assignment<bool>>,
    //r_cm2
    rcm2:Vec<Assignment<bool>>,
    //va2
    va2:Assignment<Fr>,
    //addr_sk2
    addr_sk2:Vec<Assignment<bool>>,
    //coin path 2
    path2:Vec<Vec<Assignment<bool>>>,
    //path location 2
    loc2:Vec<Assignment<bool>>,
    //r_cm
    rcm:Vec<Assignment<bool>>,
    //addr
    addr:Vec<Assignment<bool>>,
    //result
    res: &'a mut Vec<FrRepr>
}

impl<'a> C2Ccircuit<'a>{
    fn blank(
        generators: &'a[(Vec<Fr>,Vec<Fr>)],
        j:&'a JubJub,
        res: &'a mut Vec<FrRepr>
    ) -> C2Ccircuit<'a>
    {
        C2Ccircuit{
            generators,
            j,
            rcm1: (0..RCMBIT).map(|_| Assignment::unknown()).collect(),
            va1: Assignment::unknown(),
            addr_sk1: (0..ADSK).map(|_| Assignment::unknown()).collect(),
            path1: (0..TREEDEPTH).map(|_| (0..PHBIT).map(|_| Assignment::unknown()).collect()).collect(),
            loc1: (0..TREEDEPTH).map(|_| Assignment::unknown()).collect(),
            rcm2: (0..RCMBIT).map(|_| Assignment::unknown()).collect(),
            va2: Assignment::unknown(),
            addr_sk2: (0..ADSK).map(|_| Assignment::unknown()).collect(),
            path2: (0..TREEDEPTH).map(|_| (0..PHBIT).map(|_| Assignment::unknown()).collect()).collect(),
            loc2: (0..TREEDEPTH).map(|_| Assignment::unknown()).collect(),
            rcm: (0..RCMBIT).map(|_| Assignment::unknown()).collect(),
            addr: (0..ADSK).map(|_| Assignment::unknown()).collect(),
            res
        }
    }

    fn new(
        generators: &'a[(Vec<Fr>,Vec<Fr>)],
        j:&'a JubJub,
        rcm1:Vec<bool>,
        va1:Fr,
        addr_sk1:Vec<bool>,
        path1:Vec<[u64;4]>,
        loc1:Vec<bool>,
        rcm2:Vec<bool>,
        va2:Fr,
        addr_sk2:Vec<bool>,
        path2:Vec<[u64;4]>,
        loc2:Vec<bool>,
        rcm:Vec<bool>,
        addr:Vec<bool>,
        res: &'a mut Vec<FrRepr>
    )->C2Ccircuit<'a>{
        assert_eq!(rcm1.len(), RCMBIT);
        assert_eq!(addr_sk1.len(), ADSK);
        assert_eq!(path1.len(),TREEDEPTH);
        assert_eq!(loc1.len(),TREEDEPTH);
        assert_eq!(rcm2.len(), RCMBIT);
        assert_eq!(addr_sk2.len(), ADSK);
        assert_eq!(path2.len(),TREEDEPTH);
        assert_eq!(loc2.len(),TREEDEPTH);
        assert_eq!(rcm.len(), RCMBIT);
        assert_eq!(addr.len(), PHBIT);
        assert_eq!(res.len(), 0);
        let path1:Vec<Vec<bool>> = path1.into_iter().map(|u644|{
            let mut v = Vec::with_capacity(256);
            for u in u644.into_iter(){
                let mut u = *u;
                v.push((u&1)==1);
                for _ in 0..63{
                    u>>=1;
                    v.push((u&1)==1);
                }
            }
            v
        }).collect();
        let path2:Vec<Vec<bool>> = path2.into_iter().map(|u644|{
            let mut v = Vec::with_capacity(256);
            for u in u644.into_iter(){
                let mut u = *u;
                v.push((u&1)==1);
                for _ in 0..63{
                    u>>=1;
                    v.push((u&1)==1);
                }
            }
            v
        }).collect();
        C2Ccircuit{
            generators,
            j,
            rcm1:rcm1.iter().map(|&b|Assignment::known(b)).collect(),
            va1:Assignment::known(va1),
            addr_sk1:addr_sk1.iter().map(|&b|Assignment::known(b)).collect(),
            path1:path1.iter().map(|ref ph| ph.iter().map(|&b| Assignment::known(b)).collect()).collect(),
            loc1:loc1.iter().map(|&b|Assignment::known(b)).collect(),
            rcm2:rcm2.iter().map(|&b|Assignment::known(b)).collect(),
            va2:Assignment::known(va2),
            addr_sk2:addr_sk2.iter().map(|&b|Assignment::known(b)).collect(),
            path2:path2.iter().map(|ref ph| ph.iter().map(|&b| Assignment::known(b)).collect()).collect(),
            loc2:loc2.iter().map(|&b|Assignment::known(b)).collect(),
            rcm:rcm.iter().map(|&b|Assignment::known(b)).collect(),
            addr:addr.iter().map(|&b|Assignment::known(b)).collect(),
            res
        }
    }
}

struct C2CcircuitInput{
    //nullifier1
    nullifier1:Num<Bls12>,
    //nullifier2
    nullifier2:Num<Bls12>,
    //root1
    root1:Num<Bls12>,
    //root2
    root2:Num<Bls12>,
    //coin
    coin:Num<Bls12>
}

impl<'a> Input<Bls12> for C2CcircuitInput{
    fn synthesize<CS:PublicConstraintSystem<Bls12>>(self,cs:&mut CS)->Result<(),Error>{
        let nullifier1_input = cs.alloc_input(||{
            Ok(*self.nullifier1.getvalue().get()?)
        })?;
        let nullifier2_input = cs.alloc_input(||{
            Ok(*self.nullifier2.getvalue().get()?)
        })?;
        let root1_input = cs.alloc_input(||{
            Ok(*self.root1.getvalue().get()?)
        })?;
        let root2_input = cs.alloc_input(||{
            Ok(*self.root2.getvalue().get()?)
        })?;
        let coin_input = cs.alloc_input(||{
            Ok(*self.coin.getvalue().get()?)
        })?;

        cs.enforce(
            LinearCombination::zero() + self.nullifier1.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + nullifier1_input
        );
        cs.enforce(
            LinearCombination::zero() + self.nullifier2.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + nullifier2_input
        );
        cs.enforce(
            LinearCombination::zero() + self.root1.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + root1_input
        );
        cs.enforce(
            LinearCombination::zero() + self.root2.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + root2_input
        );
        cs.enforce(
            LinearCombination::zero() + self.coin.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + coin_input
        );

        Ok(())
    }
}

impl<'a> Circuit<Bls12> for C2Ccircuit<'a>{
    type InputMap = C2CcircuitInput;

    fn synthesize<CS:ConstraintSystem<Bls12>>(self,cs:&mut CS)->Result<Self::InputMap,Error> {
        let mut rcm1 = Vec::with_capacity(RCMBIT);
        for b in self.rcm1.iter() {
            rcm1.push(Bit::alloc(cs, *b)?);
        }
        let mut addr_sk1 = Vec::with_capacity(ADSK);
        for b in self.addr_sk1.iter() {
            addr_sk1.push(Bit::alloc(cs, *b)?);
        }
        let va1 = Num::new(cs,self.va1)?;
        let bit_va1 = va1.unpack_sized(cs, VBIT)?;
        assert_eq!(bit_va1.len(), VBIT);

        let mut rcm2 = Vec::with_capacity(RCMBIT);
        for b in self.rcm2.iter() {
            rcm2.push(Bit::alloc(cs, *b)?);
        }
        let mut addr_sk2 = Vec::with_capacity(ADSK);
        for b in self.addr_sk2.iter() {
            addr_sk2.push(Bit::alloc(cs, *b)?);
        }
        let va2 = Num::new(cs,self.va2)?;
        let bit_va2 = va2.unpack_sized(cs, VBIT)?;
        assert_eq!(bit_va2.len(), VBIT);

        let mut rcm = Vec::with_capacity(RCMBIT);
        for b in self.rcm.iter() {
            rcm.push(Bit::alloc(cs, *b)?);
        }
        let mut addr = Vec::with_capacity(PHBIT);
        for b in self.addr.iter() {
            addr.push(Bit::alloc(cs, *b)?);
        }
        let va = va1.add(cs,&va2)?;
        let bit_va = va.unpack_sized(cs, VBIT)?;
        assert_eq!(bit_va.len(), VBIT);
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

        //nullifier1 = PH(addr_sk1|value1|rcm1)
        let mut rcm12 = rcm1.clone();
        let vin = {
            for b in bit_va1.iter(){
                rcm1.push(*b);
            }
            for b in addr_sk1.iter(){
                rcm1.push(*b);
            }
            rcm1
        };
        assert_eq!(vin.len(), PHIN);
        let nullifier1 = pedersen_hash(cs, &vin, self.generators, self.j)?;
        if let Ok(x) = nullifier1.getvalue().get(){
            self.res.push(x.into_repr());
        }

        //nullifier2 = PH(addr_sk2|value2|rcm2)
        let mut rcm22 = rcm2.clone();
        let vin = {
            for b in bit_va2.iter(){
                rcm2.push(*b);
            }
            for b in addr_sk2.iter(){
                rcm2.push(*b);
            }
            rcm2
        };
        assert_eq!(vin.len(), PHIN);
        let nullifier2 = pedersen_hash(cs, &vin, self.generators, self.j)?;
        if let Ok(x) = nullifier2.getvalue().get(){
            self.res.push(x.into_repr());
        }

        assert_eq!(addr_sk1.len(), ADSK);
        for _ in 0..(PHIN-ADSK){
            addr_sk1.push(Bit::one(cs));
        }
        let addr1 = pedersen_hash(cs, &addr_sk1, self.generators, self.j)?;
        let addr1 = addr1.unpack_sized(cs, PHBIT)?;

        //coin1 = PH(addr1|value1|rcm1)
        let vin = {
            for b in bit_va1.iter(){
                rcm12.push(*b);
            }
            for b in addr1.iter(){
                rcm12.push(*b);
            }
            rcm12
        };
        assert_eq!(vin.len(), PHIN);
        let mut phout = pedersen_hash(cs, &vin, self.generators, self.j)?;

        let mut locs1 = Vec::with_capacity(TREEDEPTH);
        for b in self.loc1.iter(){
            locs1.push(Bit::alloc(cs,*b)?);
        }

        for (loc,sib) in locs1.iter().zip(self.path1.iter()){
            let phbits = phout.unpack_sized(cs,PHBIT)?;

            let mut vin = vec![];
            for (a,b) in sib.iter().zip(phbits.iter()){
                let bit_ph = &b;
                let bit_sib = &Bit::alloc(cs,*a)?;
                let bit_out = loc.choose_bit(cs,bit_ph,bit_sib)?;
                vin.push(bit_out);
            }
            for (a,b) in sib.iter().zip(phbits.iter()){
                let bit_ph = &b;
                let bit_sib = &Bit::alloc(cs,*a)?;
                let bit_out = loc.choose_bit(cs,bit_sib,bit_ph)?;
                vin.push(bit_out);
            }
            assert_eq!(vin.len(),PHIN);

            phout = pedersen_hash(cs, &vin, self.generators, self.j)?;
        }
        if let Ok(x) = phout.getvalue().get(){
            self.res.push(x.into_repr());
        }
        let root1 = phout;

        assert_eq!(addr_sk2.len(), ADSK);
        for _ in 0..(PHIN-ADSK){
            addr_sk2.push(Bit::one(cs));
        }
        let addr2 = pedersen_hash(cs, &addr_sk2, self.generators, self.j)?;
        let addr2 = addr2.unpack_sized(cs, PHBIT)?;

        //coin2 = PH(addr2|value2|rcm2)
        let vin = {
            for b in bit_va2.iter(){
                rcm22.push(*b);
            }
            for b in addr2.iter(){
                rcm22.push(*b);
            }
            rcm22
        };
        assert_eq!(vin.len(), PHIN);
        let mut phout = pedersen_hash(cs, &vin, self.generators, self.j)?;

        let mut locs2 = Vec::with_capacity(TREEDEPTH);
        for b in self.loc2.iter(){
            locs2.push(Bit::alloc(cs,*b)?);
        }

        for (loc,sib) in locs2.iter().zip(self.path2.iter()){
            let phbits = phout.unpack_sized(cs,PHBIT)?;

            let mut vin = vec![];
            for (a,b) in sib.iter().zip(phbits.iter()){
                let bit_ph = &b;
                let bit_sib = &Bit::alloc(cs,*a)?;
                let bit_out = loc.choose_bit(cs,bit_ph,bit_sib)?;
                vin.push(bit_out);
            }
            for (a,b) in sib.iter().zip(phbits.iter()){
                let bit_ph = &b;
                let bit_sib = &Bit::alloc(cs,*a)?;
                let bit_out = loc.choose_bit(cs,bit_sib,bit_ph)?;
                vin.push(bit_out);
            }
            assert_eq!(vin.len(),PHIN);

            phout = pedersen_hash(cs, &vin, self.generators, self.j)?;
        }
        if let Ok(x) = phout.getvalue().get(){
            self.res.push(x.into_repr());
        }
        let root2 = phout;

        Ok(C2CcircuitInput{
            nullifier1,
            nullifier2,
            root1,
            root2,
            coin
        })
    }
}

pub fn c2c_info(
    rcm1:Vec<bool>,va1:&str,addr_sk1:Vec<bool>,path1:Vec<[u64;4]>,loc1:Vec<bool>,
    rcm2:Vec<bool>,va2:&str,addr_sk2:Vec<bool>,path2:Vec<[u64;4]>,loc2:Vec<bool>,
    rcm:Vec<bool>,addr:Vec<bool>
)->Result<(
    (([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
    [u64;4],[u64;4],[u64;4],[u64;4],[u64;4]),Error>{
    let rng = &mut thread_rng();
    let j = JubJub::new();
    let mut res: Vec<FrRepr> = vec![];
    let proof = create_random_proof::<Bls12, _, _, _>(C2Ccircuit::new(
        &ph_generator(),
        &j,
        rcm1,
        Fr::from_str(va1).unwrap(),
        addr_sk1,
        path1,
        loc1,
        rcm2,
        Fr::from_str(va2).unwrap(),
        addr_sk2,
        path2,
        loc2,
        rcm,
        addr,
        &mut res
    ), c2c_param()?, rng)?.serial();
    let coin = res[0].serial();
    let nullifier1 = res[1].serial();
    let nullifier2 = res[2].serial();
    let root1 = res[3].serial();
    let root2 = res[4].serial();
    Ok((proof,coin,nullifier1,nullifier2,root1,root2))
}

pub fn c2c_verify(nullifier1:[u64;4], root1:[u64;4],nullifier2:[u64;4], root2:[u64;4],coin:[u64;4],
                  proof:(([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
                  )->Result<bool,Error>{
    verify_proof(&c2c_vk()?, &Proof::from_serial(proof), |cs| {
        let coin = Fr::from_repr(FrRepr::from_serial(coin)).unwrap();
        let nullifier1 = Fr::from_repr(FrRepr::from_serial(nullifier1)).unwrap();
        let root1 = Fr::from_repr(FrRepr::from_serial(root1)).unwrap();
        let nullifier2 = Fr::from_repr(FrRepr::from_serial(nullifier2)).unwrap();
        let root2 = Fr::from_repr(FrRepr::from_serial(root2)).unwrap();
        let coin_var = cs.alloc(||{Ok(coin)})?;
        let nullifier_var1 = cs.alloc({||Ok(nullifier1)})?;
        let root1_var = cs.alloc({||Ok(root1)})?;
        let nullifier_var2 = cs.alloc({||Ok(nullifier2)})?;
        let root2_var = cs.alloc({||Ok(root2)})?;
        Ok(C2CcircuitInput{
            nullifier1:Num::create(Assignment::known(nullifier1),nullifier_var1),
            nullifier2:Num::create(Assignment::known(nullifier2),nullifier_var2),
            root1:Num::create(Assignment::known(root1),root1_var),
            root2:Num::create(Assignment::known(root2),root2_var),
            coin:Num::create(Assignment::known(coin),coin_var)
        })
    })
}

pub fn ensure_c2c_param() ->Result<(),Error>{
    if !Path::new(C2CPARAMPATH).exists(){
        println!("Creating the parameters");
        let rng = &mut thread_rng();
        let params = generate_random_parameters::<Bls12, _, _>(C2Ccircuit::blank(
            &ph_generator(),
            &JubJub::new(),
            &mut vec![]
        ), rng)?;
        params.write(&mut File::create(C2CPARAMPATH).unwrap()).unwrap();
        println!("Just wrote the parameters to disk!");
    }
    Ok(())
}

pub fn c2c_param()->Result<ProverStream,Error>{
    ensure_c2c_param()?;
    let params = ProverStream::new(C2CPARAMPATH).unwrap();
    Ok(params)
}

pub fn c2c_vk()->Result<(PreparedVerifyingKey<Bls12>),Error>{
    ensure_c2c_param()?;
    let mut params = ProverStream::new(C2CPARAMPATH)?;
    let vk2 = params.get_vk(6)?;
    let vk = prepare_verifying_key(&vk2);
    Ok(vk)
}