#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde;
#[macro_use] extern crate log;
extern crate redis as redis_main;
extern crate reqwest;

pub mod keycloak;
pub mod sms;
pub mod redis;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhoneRequest {
    pub number: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhoneOtpReply {
    pub number: String,
    pub otp: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserCreate {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String
}

