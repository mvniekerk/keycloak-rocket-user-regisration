use std::env;
use reqwest::header::HeaderMap;
use reqwest::{Error, StatusCode};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KcAuth {
    pub access_token: String,
    pub expires_in: u32,
    pub refresh_expires_in: u32,
    pub refresh_token: String,
    pub token_type: String,
    #[serde(rename = "not-before-policy", default)]
    pub not_before_policy: u32,
    pub session_state: String,
    pub scope: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KcRegistration {
    pub enabled: bool,
    pub attributes: std::collections::HashMap<String, String>,
    pub username: String,
    #[serde(rename = "emailVerified", default)]
    pub email_verified: bool,
    pub email: String,
    #[serde(rename = "firstName", default)]
    pub first_name: String,
    #[serde(rename = "lastName", default)]
    pub last_name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KcErrorMessage {
    #[serde(rename = "errorMessage", default)]
    pub error_message: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KcPasswordReset {
    #[serde(rename = "type", default)]
    pub password_type: String,
    pub value: String,
    pub temporary: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KcUserAccess {
    #[serde(rename = "manageGroupMembership", default)]
    manage_group_membership: bool,
    view: bool,
    #[serde(rename = "mapRoles", default)]
    map_roles: bool,
    impersonate: bool,
    manage: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KcUser {
    id: String,
    #[serde(rename = "createdTimestamp", default)]
    created_timestamp: i64,
    username: String,
    enabled: bool,
    totp: bool,
    #[serde(rename = "emailVerified", default)]
    email_verified: bool,
    #[serde(rename = "firstName", default)]
    first_name: String,
    #[serde(rename = "firstName", default)]
    last_name: String,
    email: String,
    #[serde(rename = "disableableCredentialTypes", default)]
    disableable_credential_types: Vec<String>,
    #[serde(rename = "requiredActions", default)]
    required_actions: Vec<String>,
    #[serde(rename = "notBefore", default)]
    not_before: i32,
    access: KcUserAccess
}

pub fn kc_client_jwt() -> Result<String, reqwest::Error> {
    let kc_base = env::var("KC_BASE")
        .expect("KC_BASE must be set");
    let kc_realm = env::var("KC_REALM")
        .expect("KC_REALM must be set");
    let kc_client_id = env::var("KC_CLIENT_ID")
        .expect("KC_CLIENT_ID must be set");
    let kc_client_secret = env::var("KC_CLIENT_SECRET")
        .expect("KC_CLIENT_SECRET must be set");

    let url = format!("{:}/realms/{:}/protocol/openid-connect/token", kc_base, kc_realm);

    let mut hm = std::collections::HashMap::new();
    hm.insert("grant_type", "client_credentials");
    hm.insert("client_id", kc_client_id.as_str());
    hm.insert("client_secret", kc_client_secret.as_str());

    let client = reqwest::blocking::Client::new();
    let j: KcAuth = client.post(&url)
        .form(&hm)
        .send()?.json()?;
    Ok(j.access_token)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KcError {
    pub status: u16,
    pub message: String
}

impl From<reqwest::Error> for KcError {
    fn from(e: Error) -> Self {
        KcError {
            status: e.status().unwrap_or(StatusCode::from_u16(500).unwrap()).as_u16(),
            message: e.to_string()
        }
    }
}


pub fn kc_user_registration(jwt: &String, user: &KcRegistration) -> Result<String, KcError> {
    let kc_base = env::var("KC_BASE")
        .expect("KC_BASE must be set");
    let kc_realm = env::var("KC_REALM")
        .expect("KC_REALM must be set");
    let url = format!("{:}/admin/realms/{:}/users", kc_base, kc_realm);

    let client = reqwest::blocking::Client::new();
    let user_create: reqwest::blocking::Response = client.post(&url)
        .bearer_auth(jwt)
        .json(&user)
        .header("Accepts", "application/json")
        .send()?;
    let status: reqwest::StatusCode = user_create.status();
    trace!("Sent: {:}", status.as_u16());
    if !status.is_success() {
        let error: KcErrorMessage = user_create.json()?;
        return Err(KcError {
            status: status.as_u16(),
            message: error.error_message
        });
    }
    let headers = user_create.headers();
    let user_url: &reqwest::header::HeaderValue = &headers["location"];
    let user_url = user_url.to_str().unwrap();
    let user_url = user_url.to_string();
    trace!("User url: {:}", user_url);
    let user: KcUser = client.get(&user_url)
        .bearer_auth(jwt)
        .header("Accepts", "application/json")
        .send()?
        .json()?;
    Ok(user.id)
}

pub fn kc_reset_user_password(jwt: &String, uuid: &String, password: &String, temporary: bool) -> Result<(), KcError> {

    let kc_base = env::var("KC_BASE")
        .expect("KC_BASE must be set");
    let kc_realm = env::var("KC_REALM")
        .expect("KC_REALM must be set");

    let url = format!("{:}/admin/realms/{:}/users/{:}/reset-password", kc_base, kc_realm, uuid);

    let json = KcPasswordReset {
        password_type: "Password".to_string(),
        value: password.clone(),
        temporary
    };

    let client = reqwest::blocking::Client::new();
    client.put(&url)
        .bearer_auth(jwt)
        .json(&json)
        .header("Accepts", "application/json")
        .send()?;
    Ok(())
}

pub fn kc_send_verification_email(jwt: &String, uuid: &String) -> Result<(), KcError> {
    let kc_base = env::var("KC_BASE")
        .expect("KC_BASE must be set");
    let kc_realm = env::var("KC_REALM")
        .expect("KC_REALM must be set");

    let url = format!("{:}/admin/realms/{:}/users/{:}/execute-actions-email?lifespan=43200", kc_base, kc_realm, uuid);

    let client = reqwest::blocking::Client::new();
    client.put(&url)
        .bearer_auth(jwt)
        .body("[\"VERIFY_EMAIL\"]")
        .header("Accepts", "application/json")
        .send()?;
    Ok(())
}
