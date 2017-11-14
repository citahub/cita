extern crate rand;
extern crate zktx;

use rand::{Rng, thread_rng};
use zktx::base::*;
use zktx::c2p::*;
use zktx::p2c::*;

struct Account{
    pub balance: ([u64;4],[u64;4]),//in homomorphic encrpytion, = vP1+rP2
    pub address: ([u64;4],[u64;4]),//address
    v: [u64;2],//private information: balance
    r: [u64;4],//private information: random number
    sk: Vec<bool>//private information: secret_key
}

struct SendMessage{
    proof:(([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
    //hb:([u64;4],[u64;4]),
    coin:[u64;4],
    delt_ba:([u64;4],[u64;4]),
    rp:([u64;4],[u64;4]),
    enc:[u64;4],
    onchain:bool
}

struct PrivateSendMessage{
    v:[u64;2],
    r:[u64;2]
}

impl SendMessage{
    pub fn on_chain(&mut self){
        self.onchain = true;
    }

    pub fn is_on_chain(&self)->bool{
        self.onchain
    }
}

struct ReceiveMessage{
    proof:(([u64; 6], [u64; 6], bool), (([u64; 6], [u64; 6]), ([u64; 6], [u64; 6]), bool), ([u64; 6], [u64; 6], bool)),
    nullifier:[u64;4],
    root:[u64;4],
    delt_ba:([u64;4],[u64;4]),
    onchain:bool
}

impl ReceiveMessage{
    pub fn on_chain(&mut self){
        self.onchain = true;
    }

    pub fn is_on_chain(&self)->bool{
        self.onchain
    }
}

struct PrivateReceiveMessage{
    v:[u64;2],
    r:[u64;2]
}

impl Account{
    pub fn new(v:[u64;2],r:[u64;2])->Self{
        let rng = &mut thread_rng();
        let sk = (0..ADSK).map(|_| rng.gen()).collect::<Vec<bool>>();
        let address = address(&sk);
        let balance = v_p1_add_r_p2(v,r);
        Account{
            balance,
            address,
            v,
            r:[r[0],r[1],0,0],
            sk
        }
    }

    pub fn get_address(&self)->([u64;4],[u64;4]){
        self.address
    }

    pub fn get_balance(&self)->([u64;4],[u64;4]){
        self.balance
    }

    fn add_balance(&mut self,value:([u64;4],[u64;4])){
        self.balance = ecc_add(self.balance,value);
    }

    fn sub_balance(&mut self,value:([u64;4],[u64;4])){
        self.balance = ecc_sub(self.balance,value);
    }

    pub fn send(&self,v:[u64;2],rcm:[u64;2],address:([u64;4],[u64;4]))->(SendMessage,PrivateSendMessage){
        let rng = &mut thread_rng();
        let enc_random = [rng.gen(),rng.gen(),rng.gen(),rng.gen()];
        let (proof,hb,coin,delt_ba,rp,enc) = p2c_info(self.r,rcm,self.v,v,address,enc_random).unwrap();
        assert_eq!(hb,self.get_balance());
        (
            SendMessage{
                proof,
                coin,
                delt_ba,
                rp,
                enc,
                onchain:false
            },
            PrivateSendMessage{
                v,r:rcm
            }
        )
    }

    pub fn send_refresh(&mut self,private_message:&PrivateSendMessage,message:&SendMessage){
        if message.is_on_chain() {
            let pr = private_message.r;
            self.r = u644sub(self.r,[pr[0],pr[1],0,0]);
            let pv = private_message.v;
            let sv = self.v;
            let temp = u644sub([sv[0],sv[1],0,0],[pv[0],pv[1],0,0]);
            self.v = [temp[0],temp[1]];
        }
    }

    pub fn receive(&self,message:SendMessage)->(ReceiveMessage,PrivateReceiveMessage){
        let (va,rcm) = decrypt(message.enc,message.rp,self.sk.clone());
        let rng = &mut thread_rng();
        let path:Vec<[u64;4]> = (0..TREEDEPTH).map(|_| {
            let mut v:[u64;4] = [0;4];
            for i in 0..4{
                v[i] = rng.gen();
            }
            v
        }).collect();
        let locs:Vec<bool> = (0..TREEDEPTH).map(|_| rng.gen()).collect::<Vec<bool>>();
        let (proof,nullifier,root,delt_ba) = c2p_info(rcm,va,self.sk.clone(),path,locs).unwrap();
        (
            ReceiveMessage{
                proof,
                nullifier,
                root,
                delt_ba,
                onchain:false
            },
            PrivateReceiveMessage{
                v:va,
                r:rcm
            }
        )
    }

    pub fn receive_refresh(&mut self,private_message:&PrivateReceiveMessage,message:&ReceiveMessage){
        if message.is_on_chain() {
            let pr = private_message.r;
            self.r = u644add(self.r,[pr[0],pr[1],0,0]);
            let pv = private_message.v;
            let sv = self.v;
            let temp = u644add([sv[0],sv[1],0,0],[pv[0],pv[1],0,0]);
            self.v = [temp[0],temp[1]];
        }
    }

    pub fn state_out(&self,name:&str){
        println!("{}: v = {:?}, r = {:?}",name,self.v,self.r);
    }
}

fn verify_send(message:&mut SendMessage,sender:&mut Account){
    assert!(p2c_verify(sender.get_balance(),message.coin,message.delt_ba,message.rp,message.enc,message.proof).unwrap());
    message.on_chain();
    sender.sub_balance(message.delt_ba);
}

fn verify_receive(message:&mut ReceiveMessage,receiver:&mut Account){
    assert!(c2p_verify(message.nullifier,message.root,message.delt_ba,message.proof).unwrap());
    message.on_chain();
    receiver.add_balance(message.delt_ba);
}

fn round_test(){
    let rng = &mut thread_rng();
    let mut alice = Account::new([1001,0],[rng.gen(),rng.gen()]);
    let mut bob = Account::new([1000,0],[rng.gen(),rng.gen()]);

    let (mut alice_send_message,alice_private_send_message) = alice.send([10,0],[rng.gen(),rng.gen()],bob.get_address());
    verify_send(&mut alice_send_message,&mut alice);
    alice.send_refresh(&alice_private_send_message,&alice_send_message);
    alice.state_out("alice");

    let (mut bob_receive_message,bob_private_receive_message) = bob.receive(alice_send_message);
    verify_receive(&mut bob_receive_message,&mut bob);
    bob.receive_refresh(&bob_private_receive_message,&bob_receive_message);
    bob.state_out("bob");

    let (mut bob_send_message,bob_private_send_message) = bob.send([200,0],[rng.gen(),rng.gen()],alice.get_address());
    verify_send(&mut bob_send_message,&mut bob);
    bob.send_refresh(&bob_private_send_message,&bob_send_message);
    bob.state_out("bob");

    let (mut alice_receive_message,alice_private_receive_message) = alice.receive(bob_send_message);
    verify_receive(&mut alice_receive_message,&mut alice);
    alice.receive_refresh(&alice_private_receive_message,&alice_receive_message);
    alice.state_out("alice");
}

fn main(){
    println!("Round Test:");

    round_test();

    println!("Test End.");
}