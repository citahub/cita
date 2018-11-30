// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

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

use cita_crypto::{CreateKey, KeyPair};
use cita_types::Address;
use libexecutor::block::OpenBlock;
use libexecutor::command::Commander;
use libexecutor::executor::Executor;
use libexecutor::fsm::FSM;
use test::Bencher;
use tests::helpers;

fn generate_block(executor: &Executor, txs: u32) -> OpenBlock {
    let keypair = KeyPair::gen_keypair();
    let privkey = keypair.privkey();
    let data = helpers::generate_contract();
    helpers::create_block(&executor, Address::from(0), &data, (0, txs), &privkey)
}

#[bench]
fn test_block_with_10000_tx(b: &mut Bencher) {
    // One block with 10000 tx bench test takes 271.51ms
    let mut executor = helpers::init_executor(vec![]);
    let block = generate_block(&executor, 10000);

    b.iter(|| {
        executor.into_fsm(block.clone());
    });
}

#[bench]
fn test_block_with_30000_tx(b: &mut Bencher) {
    // One block with 30000 tx bench test takes 886.39ms
    let mut executor = helpers::init_executor(vec![]);
    let block = generate_block(&executor, 30000);

    b.iter(|| {
        executor.into_fsm(block.clone());
    });
}

#[bench]
fn test_block_with_50000_tx(b: &mut Bencher) {
    // One block with 50000 tx bench test takes 1424.51ms
    let mut executor = helpers::init_executor(vec![]);
    let block = generate_block(&executor, 50000);

    b.iter(|| {
        executor.into_fsm(block.clone());
    });
}

#[bench]
fn test_block_with_10000_tx_write_db(b: &mut Bencher) {
    // One block with 10000 tx bench test takes 1551.8ms
    let mut executor = helpers::init_executor(vec![]);
    let block = generate_block(&executor, 10000);

    b.iter(|| {
        let closed_block = executor.into_fsm(block.clone());
        executor.grow(closed_block);
    });
}
