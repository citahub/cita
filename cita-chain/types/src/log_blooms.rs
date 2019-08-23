use bloomchain::group::BloomGroup;
use bloomchain::Bloom;
use cita_types::Bloom as LogBloom;
use rlp::{Decodable, DecoderError, Encodable, RlpStream, UntrustedRlp};

#[derive(Debug, Clone)]
pub struct LogBloomGroup {
    blooms: Vec<LogBloom>,
}

impl From<BloomGroup> for LogBloomGroup {
    fn from(group: BloomGroup) -> Self {
        let blooms = group
            .blooms
            .into_iter()
            .map(|x| LogBloom::from(Into::<[u8; 256]>::into(x)))
            .collect();
        LogBloomGroup { blooms }
    }
}

impl Into<BloomGroup> for LogBloomGroup {
    fn into(self) -> BloomGroup {
        let blooms = self
            .blooms
            .into_iter()
            .map(|x| Bloom::from(Into::<[u8; 256]>::into(x)))
            .collect();
        BloomGroup { blooms }
    }
}

impl Decodable for LogBloomGroup {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        let blooms = rlp.as_list()?;
        let group = LogBloomGroup { blooms };
        Ok(group)
    }
}

impl Encodable for LogBloomGroup {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append_list(&self.blooms);
    }
}
