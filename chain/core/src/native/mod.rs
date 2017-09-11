// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

////////////////////////////////////////////////////////////////////////////////

mod nowpay;
// mod permission;
mod zhongchao;

pub use self::nowpay::nowpay::NowPay;
pub use self::zhongchao::contract::ZcPermission;
use action_params::ActionParams;
use evm::{self, Ext, GasLeft, Error};
use std::boxed::Box;
use std::collections::HashMap;
use util::{H256, U256};

////////////////////////////////////////////////////////////////////////////////
pub type Signature = u32;
pub type Function = Fn(&ActionParams, &mut Ext) -> evm::Result<GasLeft<'static>> + Sync + Send;
pub mod storage;
////////////////////////////////////////////////////////////////////////////////
// Contract
pub trait Contract: Sync + Send {
    fn get_function(&self, hash: &Signature) -> Option<&Box<Function>>;
    fn exec(&self, params: &ActionParams, ext: &mut Ext) {
        if let Some(data) = params.clone().data.unwrap().get(0..4) {
            let signature = data.iter().fold(0u32, |acc, &x| (acc << 8) + (x as u32));
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
