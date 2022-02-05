use uuid::Uuid;

/// Generate the API Key
pub fn generate_api_string(salt: &str) -> String {
    Uuid::new_v5(&Uuid::NAMESPACE_OID, salt.as_bytes())
        .as_simple()
        .to_string()
}
