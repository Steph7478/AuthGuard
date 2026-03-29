use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KeycloakClaims {
    pub sub: String,
    pub groups: Vec<String>,
    #[serde(rename = "costCenter")]
    pub cost_center: Option<Vec<String>>,
    pub email: Option<String>,
    pub preferred_username: Option<String>,
}
