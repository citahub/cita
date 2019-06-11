mod common;
mod contracts;
mod genesis;
mod params;

pub mod miner;
pub mod solc;

use clap::{App, Arg};
use genesis::GenesisCreator;

fn main() {
    let matches = App::new("CITA genesis creator")
        .version("1.0")
        .author("Cryptape Technologies")
        .arg(
            Arg::with_name("contract_dir")
                .help("The directory of contracts.")
                .required(true),
        )
        .arg(
            Arg::with_name("contract_docs_dir")
                .help("The directory of generated documents for contracts.")
                .required(true),
        )
        .arg(
            Arg::with_name("params_path")
                .help("Path of the file for initialization data of contracts.")
                .required(true),
        )
        .arg(
            Arg::with_name("genesis_path")
                .help("Set created genesis path")
                .required(true),
        )
        .arg(
            Arg::with_name("timestamp")
                .help("Specify a timestamp to use.")
                .required(true),
        )
        .arg(
            Arg::with_name("init_token")
                .help("Init token for this chain")
                .required(true),
        )
        .arg(
            Arg::with_name("prevhash")
                .help("Prevhash of genesis.")
                .required(true),
        )
        .get_matches();

    let contract_dir = matches.value_of("contract_dir").unwrap();
    let contract_docs_dir = matches.value_of("contract_docs_dir").unwrap();
    let params_path = matches.value_of("params_path").unwrap();
    let genesis_path = matches.value_of("genesis_path").unwrap();
    let timestamp = matches.value_of("timestamp").unwrap();
    let init_token = matches.value_of("init_token").unwrap();
    let prevhash = matches.value_of("prevhash").unwrap();
    let mut creator = GenesisCreator::new(
        contract_dir,
        contract_docs_dir,
        params_path,
        genesis_path,
        timestamp.parse::<u64>().unwrap(),
        init_token,
        prevhash,
    );

    creator.create();
}
