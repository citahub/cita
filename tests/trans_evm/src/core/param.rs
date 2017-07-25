//extern crate serde;
//use serde_json::Error;
use std::fs::File;
use std::io::BufReader;
use serde_json;

#[derive(Serialize, Deserialize,Debug)]
pub struct Param{
    pub category: i32,
    pub ipandport: Vec<String>,
    pub txnum: i32,
    pub threads: i32,
    pub code: String,
}


impl Param{

    #[allow(dead_code,unused_variables)]
    pub fn load_from_file(path: &str) -> Self {

        let config_file = File::open(path).unwrap();
        let fconfig = BufReader::new(config_file);
        serde_json::from_reader(fconfig).expect(concat!("json is invalid."))
    }
    
}

