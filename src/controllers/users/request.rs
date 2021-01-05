use serde::Deserialize;

// AddUserParam is the data required to add a user.
#[derive(Debug, Deserialize)]
pub struct AddUserParams {
    pub email: String,
    pub smothing: Option<String>,
}
