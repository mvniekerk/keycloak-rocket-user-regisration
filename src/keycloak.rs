use std::env;

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

pub fn kc_user_registration(jwt: &String, user: &KcRegistration) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let kc_base = env::var("KC_BASE")
        .expect("KC_BASE must be set");
    let kc_realm = env::var("KC_REALM")
        .expect("KC_REALM must be set");
    let url = format!("{:}/admin/realms/{:}/users", kc_base, kc_realm);

    let client = reqwest::blocking::Client::new();
    client.post(&url)
        .bearer_auth(jwt)
        .json(&user)
        .header("Accepts", "application/json")
        .send()
}

pub fn kc_reset_user_password(jwt: &String, uuid: &String, password: &String, temporary: bool) -> Result<(), reqwest::Error> {

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

pub fn kc_send_verification_email(jwt: &String, uuid: &String) -> Result<(), reqwest::Error> {
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
