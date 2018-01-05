use util::H256;

//TODO respone contain error
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TxResponse {
    pub hash: H256,
    pub status: String,
}

impl TxResponse {
    pub fn new(hash: H256, status: String) -> Self {
        TxResponse { hash, status }
    }
}
