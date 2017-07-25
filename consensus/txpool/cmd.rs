use bincode::{serialize, deserialize, Infinite};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Command {
    SpawnBlk(u64),
    PoolSituation(u64, Option<Vec<u8>>, Option<Vec<u8>>),
}

pub fn encode(cmd: &Command) -> Vec<u8> {
    serialize(cmd, Infinite).unwrap()
}

pub fn decode(bin: &[u8]) -> Command {
    deserialize(bin).unwrap()
}
