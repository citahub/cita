use bellman::groth16::*;
use pairing::*;
use pairing::bls12_381::{Fr, FrRepr, Bls12};
use bellman::*;
use rand::thread_rng;

use jubjub::*;

use base::*;

use std::fs::File;
use std::path::Path;

struct C2Pcircuit<'a>{
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
    //addr_sk
    addr_sk:Vec<Assignment<bool>>,
    //coin path
    path:Vec<Vec<Assignment<bool>>>,
    //path location
    loc:Vec<Assignment<bool>>,
    //result
    res: &'a mut Vec<FrRepr>
}

impl<'a> C2Pcircuit<'a>{
    fn blank(
        generators: &'a[(Vec<Fr>,Vec<Fr>)],
        j:&'a JubJub,
        res: &'a mut Vec<FrRepr>
    ) -> C2Pcircuit<'a>
    {
        C2Pcircuit{
            generators,
            j,
            rh: (0..RHBIT).map(|_| Assignment::unknown()).collect(),
            rhn: (0..RHBIT).map(|_| Assignment::unknown()).collect(),
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
        rh:Vec<bool>,
        rhn:Vec<bool>,
        rcm:Vec<bool>,
        ba:Fr,
        va:Fr,
        addr_sk:Vec<bool>,
        path:Vec<[u64;4]>,
        loc:Vec<bool>,
        res: &'a mut Vec<FrRepr>
    )->C2Pcircuit<'a>{
        assert_eq!(rh.len(), RHBIT);
        assert_eq!(rhn.len(), RHBIT);
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
        C2Pcircuit{
            generators,
            j,
            rh:rh.iter().map(|&b|Assignment::known(b)).collect(),
            rhn:rhn.iter().map(|&b|Assignment::known(b)).collect(),
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

struct C2PcircuitInput
{
    //H_B
    hb:Num<Bls12>,
    //H_B-next
    hbn:Num<Bls12>,
    //nullifier
    nullifier:Num<Bls12>,
    //root
    root:Num<Bls12>
}

impl<'a> Input<Bls12> for C2PcircuitInput{
    fn synthesize<CS:PublicConstraintSystem<Bls12>>(self,cs:&mut CS)->Result<(),Error>{
        let hb_input = cs.alloc_input(||{
            Ok(*self.hb.getvalue().get()?)
        })?;
        let nullifier_input = cs.alloc_input(||{
            Ok(*self.nullifier.getvalue().get()?)
        })?;
        let hbn_input = cs.alloc_input(||{
            Ok(*self.hbn.getvalue().get()?)
        })?;
        let root_input = cs.alloc_input(||{
            Ok(*self.root.getvalue().get()?)
        })?;

        cs.enforce(
            LinearCombination::zero() + self.hb.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + hb_input
        );
        cs.enforce(
            LinearCombination::zero() + self.nullifier.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + nullifier_input
        );
        cs.enforce(
            LinearCombination::zero() + self.hbn.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + hbn_input
        );
        cs.enforce(
            LinearCombination::zero() + self.root.getvar(),
            LinearCombination::zero() + CS::one(),
            LinearCombination::zero() + root_input
        );

        Ok(())
    }
}

impl<'a> Circuit<Bls12> for C2Pcircuit<'a>{
    type InputMap = C2PcircuitInput;

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

        let zero = PHIN - VBIT - RHBIT;
        //H_B = PH(1*ones|Balance|r_h)
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
                let bit_ph = &b;//phbits
                let bit_sib = &Bit::alloc(cs,*a)?;//ph
                let bit_out = loc.choose_bit(cs,bit_ph,bit_sib)?;
                vin.push(bit_out);
            }
            for (a,b) in sib.iter().zip(phbits.iter()){
                let bit_ph = &b;//phbits
                let bit_sib = &Bit::alloc(cs,*a)?;//ph
                let bit_out = loc.choose_bit(cs,bit_sib,bit_ph)?;
                vin.push(bit_out);
            }
            assert_eq!(vin.len(),PHIN);

            phout = pedersen_hash(cs, &vin, self.generators, self.j)?;
        }
        if let Ok(x) = phout.getvalue().get(){
            self.res.push(x.into_repr());
        }

        Ok(C2PcircuitInput{
            hb,
            nullifier,
            hbn,
            root:phout
        })
    }
}

pub fn c2p_info(rh:Vec<bool>,rhn:Vec<bool>,rcm:Vec<bool>,ba:&str,va:&str,addr_sk:Vec<bool>,path:Vec<[u64;4]>,loc:Vec<bool>)->Result<(
    (([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
    [u64;4],[u64;4],[u64;4],[u64;4]),Error>{
    let rng = &mut thread_rng();
    let j = JubJub::new();
    //TODO:Balance+value<2^vbit
    let mut res: Vec<FrRepr> = vec![];
    let proof = create_random_proof::<Bls12, _, _, _>(C2Pcircuit::new(
        &ph_generator(),
        &j,
        rh,
        rhn,
        rcm,
        Fr::from_str(ba).unwrap(),
        Fr::from_str(va).unwrap(),
        addr_sk,
        path,
        loc,
        &mut res
    ), c2p_param()?, rng)?.serial();
    let hb = res[0].serial();
    let nullifier = res[1].serial();
    let hbn = res[2].serial();
    let root = res[3].serial();
    Ok((proof,hb,nullifier,hbn,root))
}

pub fn c2p_verify(hb:[u64;4], nullifier:[u64;4], hbn:[u64;4], root:[u64;4],
                  proof:(([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
                  )->Result<bool,Error>{
    verify_proof(&c2p_vk()?, &Proof::from_serial(proof), |cs| {
        let hb = Fr::from_repr(FrRepr::from_serial(hb)).unwrap();
        let nullifier = Fr::from_repr(FrRepr::from_serial(nullifier)).unwrap();
        let hbn = Fr::from_repr(FrRepr::from_serial(hbn)).unwrap();
        let root = Fr::from_repr(FrRepr::from_serial(root)).unwrap();
        let hb_var = cs.alloc({||Ok(hb)})?;
        let nullifier_var = cs.alloc({||Ok(nullifier)})?;
        let hbn_var = cs.alloc({||Ok(hbn)})?;
        let root_var = cs.alloc({||Ok(root)})?;
        Ok(C2PcircuitInput{
            hb:Num::create(Assignment::known(hb),hb_var),
            nullifier:Num::create(Assignment::known(nullifier),nullifier_var),
            hbn:Num::create(Assignment::known(hbn),hbn_var),
            root:Num::create(Assignment::known(root),root_var)
        })
    })
}

pub fn ensure_c2p_param() ->Result<(),Error>{
    if !Path::new(C2PPARAMPATH).exists(){
        println!("Creating the parameters");
        let rng = &mut thread_rng();
        let params = generate_random_parameters::<Bls12, _, _>(C2Pcircuit::blank(
            &ph_generator(),
            &JubJub::new(),
            &mut vec![]
        ), rng)?;
        params.write(&mut File::create(C2PPARAMPATH).unwrap()).unwrap();
        println!("Just wrote the parameters to disk!");
    }
    Ok(())
}

pub fn c2p_param()->Result<ProverStream,Error>{
    ensure_c2p_param()?;
    let params = ProverStream::new(C2PPARAMPATH).unwrap();
    Ok(params)
}

pub fn c2p_vk()->Result<(PreparedVerifyingKey<Bls12>),Error>{
    ensure_c2p_param()?;
    let mut params = ProverStream::new(C2PPARAMPATH)?;
    let vk2 = params.get_vk(5)?;
    let vk = prepare_verifying_key(&vk2);
    Ok(vk)
}