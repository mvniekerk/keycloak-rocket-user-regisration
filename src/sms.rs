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

#[cfg(all(feature = "smsportal", not(feature = "twilio")))]
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

#[cfg(all(feature = "twilio", not(feature = "smsportal")))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmsSubresourceUris {
    media: String
}

#[cfg(all(feature = "twilio", not(feature = "smsportal")))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmsResponse {
    account_sid: String,
    api_version: String,
    body: String,
    date_created: String,
    date_sent: String,
    date_updated: String,
    direction: String,
    error_code: Option<i32>,
    error_message: Option<String>,
    from: String,
    messaging_service_sid: Option<String>,
    num_media: String,
    num_segments: String,
    price: Option<String>,
    price_unit: Option<String>,
    sid: String,
    status: String,
    subresource_uris: SmsSubresourceUris,
    to: String,
    uri: String
}

#[cfg(all(feature = "smsportal", not(feature = "twilio")))]
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

#[cfg(all(feature = "twilio", not(feature = "smsportal")))]
pub fn send_sms(message: &SmsMessageContent) -> Result<SmsResponse, reqwest::Error> {
    let twilio_account_ssid = env::var("TWILIO_ACCOUNT_SSID")
        .expect("TWILIO_ACCOUNT_SSID must be set");
    let twilio_auth_token = env::var("TWILIO_AUTH_TOKEN")
        .expect("TWILIO_AUTH_TOKEN must be set");
    let twilio_number = env::var("TWILIO_NUMBER_FROM")
        .expect("TWILIO_NUMBER_FROM must be set");

    let params = [
        ("To", &message.destination),
        ("From", &twilio_number),
        ("Body", &message.content)
    ];
    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{:}/Messages.json", twilio_account_ssid);
    let url = url.as_str();
    client.post(url)
        .basic_auth(twilio_account_ssid, Some(twilio_auth_token))
        .form(&params)
        .send()?.json()
}
