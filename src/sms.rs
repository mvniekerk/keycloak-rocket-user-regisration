use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmsPortalAuth {
    pub token: String,
    pub schema: String,
    #[serde(rename = "expiresInMinutes", default)]
    pub expires_in_minutes: u32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmsMessageContent {
    #[serde(rename = "Content", default)]
    pub content: String,
    pub destination: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmsMessage {
    #[serde(rename = "Messages", default)]
    pub messages: Vec<SmsMessageContent>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmsCostBreakdown {
    pub quantity: i32,
    pub cost: f32,
    pub network: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmsErrorFault {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmsErrorReport {
    #[serde(rename = "noNetwork", default)]
    pub no_network: i32,
    pub duplicates: i32,
    #[serde(rename = "optedOuts", default)]
    pub opted_outs: i32,
    pub faults: Vec<SmsErrorFault>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmsResponse {
    pub cost: f32,
    #[serde(rename = "remainingBalance", default)]
    pub remaining_balance: f32,
    #[serde(rename = "eventId", default)]
    pub event_id: i32,
    pub sample: String,
    #[serde(rename = "costBreakdown")]
    pub cost_breakdown: Vec<SmsCostBreakdown>,
    pub messages: i32,
    pub parts: i32,
    #[serde(rename = "errorReport")]
    pub error_report: SmsErrorReport
}

pub fn send_sms(message: &SmsMessage) -> Result<SmsResponse, reqwest::Error> {
    let sms_portal_client_id = env::var("SMS_PORTAL_CLIENT_ID")
        .expect("SMS_PORTAL_CLIENT_ID must be set");
    let sms_portal_secret = env::var("SMS_PORTAL_SECRET")
        .expect("SMS_PORTAL_SECRET must be set");


    let client = reqwest::blocking::Client::new();
    let res: SmsPortalAuth = client.get("https://rest.smsportal.com/v1/Authentication")
        .basic_auth(sms_portal_client_id.as_str(), Some(sms_portal_secret.as_str()))
        .send()?
        .json()?;
    let jwt = res.token;
    trace!("SMS token {:?}", jwt);

    client.post("https://rest.smsportal.com/v1/bulkmessages")
        .bearer_auth(jwt)
        .json(message)
        .send()?.json()
}
