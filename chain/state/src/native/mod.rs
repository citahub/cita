////////////////////////////////////////////////////////////////////////////////
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////
use action_params::{ActionParams};
use evm::{self, Ext, GasLeft};
use util::{H256, U256};

////////////////////////////////////////////////////////////////////////////////
pub type Signature = u32;
pub type Function = Fn(&ActionParams, &mut Ext) -> evm::Result<GasLeft<'static>> + Sync + Send;

////////////////////////////////////////////////////////////////////////////////
// Contract
pub trait Contract : Sync + Send { 
	fn get_function(&self, hash: &Signature) -> Option<&Box<Function>>;
	fn exec(&self, params: & ActionParams, mut ext: &mut Ext) {		
		if let Some(data) = params.clone().data.unwrap().get(0..4) {						
			let signature = data.iter().fold(0u32, |acc, &x| (acc<<8 )+(x as u32));  
			if let Some(exec_call) = self.get_function(&signature) {					
				//let cost = self.engine.cost_of_builtin(&params.code_address, data);
				let cost = U256::from(100);
				if cost <= params.gas {
					let _ = exec_call(params, ext);
					//self.state.discard_checkpoint();
					return;
				}
			}
		}
	}
}

////////////////////////////////////////////////////////////////////////////////
// NowPay
pub struct NowPay {
	functions: HashMap<Signature, Box<Function>>,
}

impl Contract for NowPay {
	fn get_function(&self, hash: &Signature) -> Option<&Box<Function>> {
		self.functions.get(hash)
	}
}

impl NowPay {
	pub fn new() -> Self {
		let mut contract = NowPay {
			functions: HashMap::<Signature, Box<Function>>::new(),
		};
		contract.functions.insert(0, Box::new(NowPay::set_value));
		contract
	}
	pub fn set_value(params: & ActionParams,  ext: & mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Some(ref data) = params.data {
            if let Some(data) = data.get(4..32) {
		        let _ = ext.set_storage(H256::from(0), H256::from(data)); 
            }
        }
        Ok(GasLeft::Known(U256::from(0)))
	}
}

