use json;
use std::collections::BTreeMap;
use std::process::Command;

pub struct Solc;

impl Solc {
    pub fn get_contracts_data(file_path: String, contract_name: &str) -> BTreeMap<String, String> {
        let output = Command::new("solc")
            .arg(file_path.clone())
            .arg("--allow-paths")
            .arg(".")
            .arg("--optimize")
            .arg("--combined-json")
            .arg("abi,bin,userdoc,hashes,devdoc")
            .output()
            .expect("solc command fail to execute");
        let output = String::from_utf8(output.stdout).unwrap();
        let compiled = json::parse(&output).unwrap();
        let index = [&file_path, ":", contract_name].concat();

        let bin = &compiled["contracts"][&index]["bin"];
        let abi = &compiled["contracts"][&index]["abi"];
        let hashes = &compiled["contracts"][&index]["hashes"];
        let userdoc = &compiled["contracts"][&index]["userdoc"];
        let devdoc = &compiled["contracts"][&index]["devdoc"];

        let mut data = BTreeMap::new();
        data.insert("bin".to_string(), bin.to_string());
        data.insert("abi".to_string(), abi.to_string());
        data.insert("hashes".to_string(), hashes.to_string());
        data.insert("userdoc".to_string(), userdoc.to_string());
        data.insert("devdoc".to_string(), devdoc.to_string());

        data
    }

    pub fn compiler_version() -> bool {
        let output = Command::new("solc")
            .arg("--version")
            .output()
            .expect("solc compiler not exist !");
        println!(
            "Solc version: {:?}",
            String::from_utf8(output.stdout).unwrap()
        );
        output.status.success()
    }
}
