#[derive(Clone, Debug)]
pub struct Role {
    // the role name
    name: String,
    // the permissions
    permissions: Vec<String>,
}
