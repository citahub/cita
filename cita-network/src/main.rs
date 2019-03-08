// CITA
// Copyright 2016-2019 Cryptape Technologies LLC.

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

//! ## Summary
//!
//! One of the CITA's core components is used to implement the peer-to-peer network
//! and provide point-to-point connection interaction.
//!
//! ### Message queuing situation
//!
//! 1. Subscribe channel
//!
//!     |       Queue       | PubModule | Message Type          |
//!     | ----------------- | --------- | --------------------- |
//!     | network_tx        | Auth      | Request               |
//!     | network_consensus | Consensus | CompactSignedProposal |
//!     | network_consensus | Consensus | RawBytes              |
//!     | network           | Chain     | Status                |
//!     | network           | Chain     | SyncResponse          |
//!     | network           | Jonsonrpc | RequestNet            |
//!     | network           | Auth      | GetBlockTxn           |
//!     | network           | Auth      | BlockTxn              |
//!
//! 2. Publish channel
//!
//!     |       Queue       | PubModule | SubModule           | Message Type          |
//!     | ----------------- | --------- | ------------------- | --------------------- |
//!     | network           | Net       | Chain, Executor     | SyncResponse          |
//!     | network           | Net       | Snapshot            | SnapshotResp          |
//!     | network           | Net       | Jsonrpc             | Response              |
//!     | network_tx        | Net       | Auth                | Request               |
//!     | network_consensus | Net       | Consensus           | ComapctSignedProposal |
//!     | network_consensus | Net       | Consensus           | RawBytes              |
//!     | network           | Net       | Auth                | BlockTxn              |
//!     | network           | Net       | Auth                | GetBlockTxn           |
//!
//! ### p2p binary protocol
//! | Start      | Full length | Key length | Key value      | Message value    |
//! | ---------- | ----------- | ---------- | -------------- | ---------------- |
//! | \xDEADBEEF | u32         | u8(byte)   | bytes of a str | a serialize data |
//!
//! full_len = 1 + key_len + body_len
//!
//! ### Key behavoir
//!
//! the key struct:
//!
//! - [`Connection`]
//! - [`NetWork`]
//! - [`Synchronizer`]
//!
//! In addition to the `tokio_server`, there is an `Arc<Connection>` for
//! this structure in almost all the threads of this module to confirm that the node is alive,
//! increase or decrease nodes, consensus message broadcasts, authentication message broadcasts,
//! node status broadcasts, synchronization node blocks Height and so on.
//!
//! About binary protocol encoding and decoding, please look at module `citaprotocol`, the fuction
//! [`pubsub_message_to_network_message`] and [`network_message_to_pubsub_message`].
//!
//! [`Connection`]: ./connection/struct.Connection.html
//! [`NetWork`]: ./network/struct.NetWork.html
//! [`Synchronizer`]: ./synchronizer/struct.Synchronizer.html
//! [`pubsub_message_to_network_message`]: ./citaprotocol/fn.pubsub_message_to_network_message.html
//! [`network_message_to_pubsub_message`]: ./citaprotocol/fn.network_message_to_pubsub_message.html
//!

pub mod cita_protocol;
pub mod config;
pub mod mq_agent;
pub mod network;
pub mod node_manager;
pub mod p2p_protocol;
pub mod synchronizer;

use crate::config::{AddressConfig, NetConfig};
use crate::mq_agent::MqAgent;
use crate::network::Network;
use crate::node_manager::{NodesManager, DEFAULT_PORT};
use crate::p2p_protocol::{
    node_discovery::DiscoveryProtocolMeta, node_discovery::NodesAddressManager,
    transfer::TransferProtocolMeta, SHandle,
};
use crate::synchronizer::Synchronizer;
use clap::App;
use dotenv;
use futures::prelude::*;
use logger::{debug, info};
use std::thread;
use tentacle::{builder::ServiceBuilder, secio::SecioKeyPair};
use util::micro_service_init;
use util::set_panic_handler;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn main() {
    micro_service_init!("cita-network", "CITA:network");
    info!("Version: {}", get_build_info_str(true));

    // init app
    let matches = App::new("network")
        .version(get_build_info_str(true))
        .long_version(get_build_info_str(false))
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'")
        .args_from_usage("-a, --address=[FILE] 'Sets an address file'")
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("config");

    // >>>> Init config
    debug!("Config path {:?}", config_path);
    let config = NetConfig::new(&config_path);
    debug!("Network config is {:?}", config);

    let addr_path = matches.value_of("address").unwrap_or("address");
    let own_addr = AddressConfig::new(&addr_path);
    debug!("Node address is {:?}", own_addr.addr);
    // <<<< End init config

    let mut nodes_mgr = NodesManager::from_config(config.clone(), own_addr.addr);
    let mut mq_agent = MqAgent::default();
    let mut synchronizer_mgr = Synchronizer::new(mq_agent.client(), nodes_mgr.client());
    let mut network_mgr = Network::new(
        mq_agent.client(),
        nodes_mgr.client(),
        synchronizer_mgr.client(),
    );
    mq_agent.set_nodes_mgr_client(nodes_mgr.client());
    mq_agent.set_network_client(network_mgr.client());

    // >>>> Init p2p protocols
    let discovery_meta =
        DiscoveryProtocolMeta::new(0, NodesAddressManager::new(nodes_mgr.client()));
    let transfer_meta = TransferProtocolMeta::new(1, network_mgr.client(), nodes_mgr.client());

    let mut service_cfg = ServiceBuilder::default()
        .insert_protocol(discovery_meta)
        .insert_protocol(transfer_meta)
        .forever(true);
    if nodes_mgr.is_enable_tls() {
        service_cfg = service_cfg.key_pair(SecioKeyPair::secp256k1_generated());
    }
    let mut service = service_cfg.build(SHandle::new(nodes_mgr.client()));

    let addr = format!("/ip4/0.0.0.0/tcp/{}", config.port.unwrap_or(DEFAULT_PORT));
    let _ = service.listen(addr.parse().unwrap());
    nodes_mgr.set_service_task_sender(service.control().clone());
    // <<<< End init p2p protocols

    // >>>> Run system
    mq_agent.run();
    thread::spawn(move || nodes_mgr.run());
    thread::spawn(move || network_mgr.run());
    thread::spawn(move || synchronizer_mgr.run());
    tokio::run(service.for_each(|_| Ok(())));
    // <<<< End run system
}
