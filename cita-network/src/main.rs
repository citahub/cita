pub mod citaprotocol;
pub mod config;
pub mod mq_client;
pub mod network;
pub mod node_manager;
pub mod p2p_protocol;
pub mod synchronizer;

use crate::config::NetConfig;
use crate::mq_client::MqClient;
use crate::network::{LocalMessage, Network};
use crate::node_manager::{BroadcastReq, NodesManager, DEFAULT_PORT};
use crate::p2p_protocol::{
    node_discovery::{DiscoveryProtocolMeta, NodesAddressManager},
    transfer::TransferProtocolMeta,
    SHandle,
};
use crate::synchronizer::Synchronizer;
use clap::App;
use dotenv;
use futures::prelude::*;
use libproto::router::{MsgType, RoutingKey, SubModules};
use libproto::routing_key;
use libproto::{Message, TryFrom};
use log::{debug, info, trace};
use p2p::{builder::ServiceBuilder, SecioKeyPair};
use pubsub::start_pubsub;
use std::sync::mpsc::channel;
use std::thread;
use util::micro_service_init;
use util::set_panic_handler;

include!(concat!(env!("OUT_DIR"), "/build_info.rs"));

fn main() {
    micro_service_init!("cita-network", "CITA:network");
    info!("Version: {}", get_build_info_str(true));

    // init app
    // todo load config
    let matches = App::new("network")
        .version(get_build_info_str(true))
        .long_version(get_build_info_str(false))
        .author("Cryptape")
        .about("CITA Block Chain Node powered by Rust")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'")
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("config");

    // >>>> Init config
    debug!("config path {:?}", config_path);
    let config = NetConfig::new(&config_path);
    debug!("network config is {:?}", config);
    // <<<< End init config

    // >>>> Init pubsub
    // New transactions use a special channel, all new transactions come from:
    // JsonRpc -> Auth -> Network,
    // So the channel subscribe 'Auth' Request from MQ
    let (ctx_sub_auth, crx_sub_auth) = channel();
    let (ctx_pub_auth, crx_pub_auth) = channel();
    start_pubsub(
        "network_auth",
        routing_key!([Auth >> Request, Auth >> GetBlockTxn, Auth >> BlockTxn]),
        ctx_sub_auth,
        crx_pub_auth,
    );

    // Consensus use a special channel
    let (ctx_sub_consensus, crx_sub_consensus) = channel();
    let (ctx_pub_consensus, crx_pub_consensus) = channel();
    start_pubsub(
        "network_consensus",
        routing_key!([Consensus >> CompactSignedProposal, Consensus >> RawBytes]),
        ctx_sub_consensus,
        crx_pub_consensus,
    );

    // Chain, Jsonrpc and Snapshot use a common channel
    let (ctx_sub, crx_sub) = channel();
    let (ctx_pub, crx_pub) = channel();
    start_pubsub(
        "network",
        routing_key!([
            Chain >> Status,
            Chain >> SyncResponse,
            Jsonrpc >> RequestNet,
            Snapshot >> SnapshotReq
        ]),
        ctx_sub,
        crx_pub,
    );
    let mq_client = MqClient::new(ctx_pub_auth, ctx_pub_consensus, ctx_pub);
    // <<<< End init pubsub

    // >>>> Init p2p protocols
    let mut nodes_mgr = NodesManager::from_config(config.clone());
    let mut synchronizer_mgr = Synchronizer::new(mq_client.clone(), nodes_mgr.client());
    let mut network_mgr = Network::new(
        mq_client.clone(),
        nodes_mgr.client(),
        synchronizer_mgr.client(),
    );
    let discovery_meta =
        DiscoveryProtocolMeta::new(0, NodesAddressManager::new(nodes_mgr.client()));
    let transfer_meta = TransferProtocolMeta::new(1, network_mgr.client());

    let mut service = ServiceBuilder::default()
        .insert_protocol(discovery_meta)
        .insert_protocol(transfer_meta)
        .forever(true)
        .key_pair(SecioKeyPair::secp256k1_generated())
        .build(SHandle::new(nodes_mgr.client()));
    let addr = format!("/ip4/0.0.0.0/tcp/{}", config.port.unwrap_or(DEFAULT_PORT));
    let _ = service.listen(&addr.parse().unwrap());
    nodes_mgr.set_service_task_sender(service.control().clone());
    // <<<< End init p2p protocols

    // >>>> Run system
    // Thread for handle new transactions from MQ
    let nodes_manager_client = nodes_mgr.client();
    thread::spawn(move || loop {
        let (key, body) = crx_sub_auth.recv().unwrap();
        let msg = Message::try_from(&body).unwrap();

        // Broadcast the message to other nodes
        nodes_manager_client.broadcast(BroadcastReq::new(key, msg));
    });

    //Thread for handle consensus message
    let nodes_manager_client = nodes_mgr.client();
    thread::spawn(move || loop {
        let (key, body) = crx_sub_consensus.recv().unwrap();
        let msg = Message::try_from(&body).unwrap();

        // Broadcast the message to other nodes
        nodes_manager_client.broadcast(BroadcastReq::new(key, msg));
    });

    let network_client = network_mgr.client();
    thread::spawn(move || loop {
        let (key, body) = crx_sub.recv().unwrap();
        trace!("[main] Handle delivery from {} payload {:?}", key, body);

        let msg = LocalMessage::new(key, body);
        network_client.handle_local_message(msg);
    });

    thread::spawn(move || nodes_mgr.run());
    thread::spawn(move || network_mgr.run());
    thread::spawn(move || synchronizer_mgr.run());
    tokio::run(service.for_each(|_| Ok(())));
    // <<<< End run system
}
