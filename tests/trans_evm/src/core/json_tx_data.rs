#[derive(Clone,Debug)]
pub enum Methods{
    Sendtx(Trans),
    Height,
    Blockbyheiht(u64),
    Trans(String),
}

#[derive(Clone,Debug)]
pub enum Data{
    Sendtx(String),
    Height(String),
    Blockbyheiht(String),
    Trans(String),
}

#[derive(Clone,Debug)]
pub struct Jsontxdata{

    pub txdata: Data;

}

impl Jsontxdata{

    pub fn new(data: String) -> Self{

        Jsontxdata{
            txdata: Sendtx(data),
        }
    }

    pub fn generate_tx_data($self, method: Methods) -> Self{

        let txdata = match method{
            Methods::Sendtx(tx) => {
                let txdata = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_sendTransaction\",\"params\":[\"{}\"],\"id\":2}}",tx.write_to_bytes().unwrap().to_hex());
                Data::Sendtx(txdata);
            },
            Methods::Height => {
                let txdata = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_blockHeight\",\"params\":[],\"id\":2}}");
                Data::Height(txdata);
            },
            Methods::Blockbyheiht(h) => {
                let txdata = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_getBlockByHeight\",\"params\":[{},false],\"id\":2}}", h);
                Data::Blockbyheiht(txdata);
            },
            Methods::Trans(hash) => {
                let txdata = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"cita_getTransaction\",\"params\":[\"{}\"],\"id\":2}}",hash);
                Data::Trans(txdata);
            },
        }
        Jsontxdata{
           txdata: Sendtx(txdata), 
        }
        //Self::new(txdata)
    }

}
