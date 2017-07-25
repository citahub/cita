#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Id {
    /// No id (notification)
    Null,
    /// String id
    Str(String),
    /// Numeric id
    Num(u64),
}
