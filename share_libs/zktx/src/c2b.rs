use bellman::groth16::*;
use pairing::*;
use pairing::bls12_381::{Fr, FrRepr, Bls12};
use bellman::*;
use rand::thread_rng;

use jubjub::*;

use base::*;

use std::fs::File;
use std::path::Path;

struct C2Bcircuit<'a>{
    generators: &'a[(Vec<Fr>, Vec<Fr>)],
    j:& 'a JubJub,

    //r_cm
    rcm:Vec<Assignment<bool>>,
    //Balance
    ba:Assignment<Fr>,
    //value
    va:Assignment<Fr>,
    //addr_sk
    addr_sk:Vec<Assignment<bool>>,
    //coin path
    path:Vec<Vec<Assignment<bool>>>,
    //path location
    loc:Vec<Assignment<bool>>,
    //result
    res: &'a mut Vec<FrRepr>
}

impl<'a> C2Bcircuit<'a>{
    fn blank(
        generators: &'a[(Vec<Fr>,Vec<Fr>)],
        j:&'a JubJub,
        res: &'a mut Vec<FrRepr>
    ) -> C2Bcircuit<'a>
    {
        C2Bcircuit{
            generators,
            j,
            rcm: (0..RCMBIT).map(|_| Assignment::unknown()).collect(),
            ba:Assignment::unknown(),
            va:Assignment::unknown(),
            addr_sk: (0..ADSK).map(|_| Assignment::unknown()).collect(),
            path: (0..TREEDEPTH).map(|_| (0..PHBIT).map(|_| Assignment::unknown()).collect()).collect(),
            loc: (0..TREEDEPTH).map(|_| Assignment::unknown()).collect(),
            res
        }
    }

    fn new(
        generators: &'a[(Vec<Fr>,Vec<Fr>)],
        j:&'a JubJub,
        rcm:Vec<bool>,
        ba:Fr,
        va:Fr,
        addr_sk:Vec<bool>,
        path:Vec<[u64;4]>,
        loc:Vec<bool>,
        res: &'a mut Vec<FrRepr>
    )->C2Bcircuit<'a>
    {
        assert_eq!(rcm.len(), RCMBIT);
        assert_eq!(addr_sk.len(), ADSK);
        assert_eq!(res.len(), 0);
        assert_eq!(path.len(),TREEDEPTH);
        assert_eq!(loc.len(),TREEDEPTH);
        let path:Vec<Vec<bool>> = path.into_iter().map(|u644|{
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
        C2Bcircuit{
            generators,
            j,
            rcm:rcm.iter().map(|&b|Assignment::known(b)).collect(),
            ba:Assignment::known(ba),
            va:Assignment::known(va),
            addr_sk:addr_sk.iter().map(|&b|Assignment::known(b)).collect(),
            path:path.iter().map(|ref ph| ph.iter().map(|&b| Assignment::known(b)).collect()).collect(),
            loc:loc.iter().map(|&b|Assignment::known(b)).collect(),
            res
        }
    }
}

struct C2BcircuitInput
{
    //H_B
    ba:Num<Bls12>,
    //H_B-next
    va:Num<Bls12>,
    //nullifier
    nullifier:Num<Bls12>,
    //root
    root:Num<Bls12>
}

impl<'a> Input<Bls12> for C2BcircuitInput{
    fn synthesize<CS:PublicConstraintSystem<Bls12>>(self,cs:&mut CS)->Result<(),Error>{
        let ba_input = cs.alloc_input(||{
            Ok(*self.ba.getvalue().get()?)
        })?;
        let nullifier_input = cs.alloc_input(||{
            Ok(*self.nullifier.getvalue().get()?)
        })?;
        let va_input = cs.alloc_input(||{
            Ok(*self.va.getvalue().get()?)
        })?;
        let root_input = cs.alloc_input(||{
            Ok(*self.root.getvalue().get()?)
        })?;

        cs.enforce(
            LinearCombination::zero() + self.ba.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + ba_input
        );
        cs.enforce(
            LinearCombination::zero() + self.nullifier.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + nullifier_input
        );
        cs.enforce(
            LinearCombination::zero() + self.va.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + va_input
        );
        cs.enforce(
            LinearCombination::zero() + self.root.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + root_input
        );

        Ok(())
    }
}

impl<'a> Circuit<Bls12> for C2Bcircuit<'a> {
    type InputMap = C2BcircuitInput;

    fn synthesize<CS: ConstraintSystem<Bls12>>(self, cs: &mut CS) -> Result<Self::InputMap, Error> {
        let mut rcm = Vec::with_capacity(RCMBIT);
        for b in self.rcm.iter() {
            rcm.push(Bit::alloc(cs, *b)?);
        }
        let mut addr_sk = Vec::with_capacity(ADSK);
        for b in self.addr_sk.iter() {
            addr_sk.push(Bit::alloc(cs, *b)?);
        }

        let ba = Num::new(cs,self.ba)?;
        let va = Num::new(cs,self.va)?;
        let bn = ba.add(cs,&va)?;
        let bit_ba = ba.unpack_sized(cs, VBIT)?;
        let bit_va = va.unpack_sized(cs, VBIT)?;
        let bit_bn = bn.unpack_sized(cs, VBIT)?;
        assert_eq!(bit_ba.len(), VBIT);
        assert_eq!(bit_va.len(), VBIT);
        assert_eq!(bit_bn.len(), VBIT);

        if let Ok(x) = bn.getvalue().get(){
            self.res.push(x.into_repr());
        }

        //nullifier = PH(addr_sk|value|rcm)
        let mut rcm2 = rcm.clone();
        let vin = {
            for b in bit_va.iter(){
                rcm.push(*b);
            }
            for b in addr_sk.iter(){
                rcm.push(*b);
            }
            rcm
        };
        assert_eq!(vin.len(), PHIN);
        let nullifier = pedersen_hash(cs, &vin, self.generators, self.j)?;
        if let Ok(x) = nullifier.getvalue().get(){
            self.res.push(x.into_repr());
        }

        assert_eq!(addr_sk.len(), ADSK);
        for _ in 0..(PHIN-ADSK){
            addr_sk.push(Bit::one(cs));
        }
        let addr = pedersen_hash(cs, &addr_sk, self.generators, self.j)?;
        let addr = addr.unpack_sized(cs, PHBIT)?;

        //coin = PH(addr|value|rcm)
        let vin = {
            for b in bit_va.iter(){
                rcm2.push(*b);
            }
            for b in addr.iter(){
                rcm2.push(*b);
            }
            rcm2
        };
        assert_eq!(vin.len(), PHIN);
        let mut phout = pedersen_hash(cs, &vin, self.generators, self.j)?;

        let mut locs = Vec::with_capacity(TREEDEPTH);
        for b in self.loc.iter(){
            locs.push(Bit::alloc(cs,*b)?);
        }

        for (loc,sib) in locs.iter().zip(self.path.iter()){
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

        Ok(C2BcircuitInput{
            ba,
            va,
            nullifier,
            root:phout
        })
    }
}

pub fn c2b_info(rcm:Vec<bool>,ba:&str,va:&str,addr_sk:Vec<bool>,path:Vec<[u64;4]>,loc:Vec<bool>)->Result<(
    (([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
    [u64;4],[u64;4],[u64;4]),Error>{
    let rng = &mut thread_rng();
    let j = JubJub::new();
    let mut res: Vec<FrRepr> = vec![];
    let proof = create_random_proof::<Bls12, _, _, _>(C2Bcircuit::new(
        &ph_generator(),
        &j,
        rcm,
        Fr::from_str(ba).unwrap(),
        Fr::from_str(va).unwrap(),
        addr_sk,
        path,
        loc,
        &mut res
    ), c2b_param()?, rng)?.serial();
    let bn = res[0].serial();
    let nullifier = res[1].serial();
    let root = res[2].serial();
    Ok((proof,bn,nullifier,root))
}

pub fn c2b_verify(ba:&str, va:&str, nullifier:[u64;4], root:[u64;4],
                  proof:(([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
                  )->Result<bool,Error>{
    verify_proof(&c2b_vk()?, &Proof::from_serial(proof), |cs| {
        let va = Fr::from_str(va).unwrap();
        let nullifier = Fr::from_repr(FrRepr::from_serial(nullifier)).unwrap();
        let ba = Fr::from_str(ba).unwrap();
        let root = Fr::from_repr(FrRepr::from_serial(root)).unwrap();
        let ba_var = cs.alloc({||Ok(ba)})?;
        let nullifier_var = cs.alloc({||Ok(nullifier)})?;
        let va_var = cs.alloc({||Ok(va)})?;
        let root_var = cs.alloc({||Ok(root)})?;
        Ok(C2BcircuitInput{
            ba:Num::create(Assignment::known(ba),ba_var),
            nullifier:Num::create(Assignment::known(nullifier),nullifier_var),
            va:Num::create(Assignment::known(va),va_var),
            root:Num::create(Assignment::known(root),root_var)
        })
    })
}

pub fn ensure_c2b_param() ->Result<(),Error>{
    if !Path::new(C2BPARAMPATH).exists(){
        println!("Creating the parameters");
        let rng = &mut thread_rng();
        let params = generate_random_parameters::<Bls12, _, _>(C2Bcircuit::blank(
            &ph_generator(),
            &JubJub::new(),
            &mut vec![]
        ), rng)?;
        params.write(&mut File::create(C2BPARAMPATH).unwrap()).unwrap();
        println!("Just wrote the parameters to disk!");
    }
    Ok(())
}

pub fn c2b_param()->Result<ProverStream,Error>{
    ensure_c2b_param()?;
    let params = ProverStream::new(C2BPARAMPATH).unwrap();
    Ok(params)
}

pub fn c2b_vk()->Result<(PreparedVerifyingKey<Bls12>),Error>{
    ensure_c2b_param()?;
    let mut params = ProverStream::new(C2BPARAMPATH)?;
    let vk2 = params.get_vk(5)?;
    let vk = prepare_verifying_key(&vk2);
    Ok(vk)
}