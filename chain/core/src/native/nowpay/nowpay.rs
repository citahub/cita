/// NowPay native contract implettion
///
///
use super::*;

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
        let mut contract = NowPay { functions: HashMap::<Signature, Box<Function>>::new() };
        contract.functions.insert(0, Box::new(NowPay::set_value));
        contract
    }

    pub fn set_value(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Some(ref data) = params.data {
            if let Some(data) = data.get(4..32) {
                let _ = ext.set_storage(H256::from(0), H256::from(data));
            }
        }
        Ok(GasLeft::Known(U256::from(0)))
    }
}
