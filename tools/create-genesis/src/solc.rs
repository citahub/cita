use json;
use std::collections::BTreeMap;
use std::process::Command;

pub struct Solc;

impl Solc {
    pub fn get_contracts_data<'a>(
        file_path: String,
        contract_name: &'a str,
    ) -> BTreeMap<&'a str, String> {
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
        data.insert("bin", bin.to_string());
        data.insert("abi", abi.to_string());
        data.insert("hashes", hashes.to_string());
        data.insert("userdoc", userdoc.to_string());
        data.insert("devdoc", devdoc.to_string());

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
