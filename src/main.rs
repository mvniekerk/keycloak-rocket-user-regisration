#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate log;
#[macro_use] extern crate rocket;

extern crate dotenv;
extern crate clap;
extern crate http;

use clap::{App, Arg};
use dotenv::dotenv;
use std::env;

use user_registration::sms::{SmsResponse, SmsMessage, SmsMessageContent, send_sms};
use user_registration::redis::*;
use user_registration::*;
use rocket_contrib::json::Json;
use rocket::response::status;
use user_registration::keycloak::*;
use std::net::ToSocketAddrs;
use rocket::http::Status;

#[post("/register", format = "json", data = "<req>")]
fn register_phone(req: Json<PhoneRequest>) -> String {
    info!("Hello {:?}", req);
    let uuid = uuid::Uuid::new_v4();

    let bv = uuid.to_u128_le();
    let m1 = (bv >> 40) & 0xFFFF;
    let m2 = (bv >> 60) & 0xFFFF;
    let m3 = (bv >> 80) & 0xFFFF;
    let m4 = (bv >> 100) & 0xFFFF;
    let r = ((m1 + m2 + m3 + m4) & 0xFFFF) as u32;
    let mut otp = format!("{:04}", r);
    if otp.len() > 4 {
        otp.truncate(4);
    }

    info!("{:?} OTP {:?}", req.number, otp);
    let uuid = uuid.to_string();
    let phone = &req.number;
    put_phone_on_redis(&otp, &uuid, &phone);
    send_otp_sms(&otp, &req.number);
    uuid
}

#[put("/register/<id>", format = "json", data = "<req>")]
fn phone_validate(id: String, req: Json<PhoneOtpReply>) -> status::Custom<String> {
    let matches = compare_phone_otp(&req.otp, &id, &req.number);
    match matches {
        Ok(v) => {
            if v {
                put_validated_phone_number_on_redis(&id, &req.number);
                status::Custom(Status::Ok, "true".to_string())
            } else {
                status::Custom(Status::BadRequest, "false".to_string())
            }

        },
        Err(_) => {
            status::Custom(Status::BadRequest, "false".to_string())
        }
    }
}

#[post("/register/<id>", format = "json", data = "<req>")]
fn register_user(id: String, req: Json<UserCreate>) -> status::Custom<String> {
    // Default Keycloak setup doesn't have a proper sending email set up. This will fail if not set up correctly.
    let send_verification_email: bool = env::var("KC_SEND_VERIFICATION_EMAIL")
        .unwrap_or("false".to_string()).parse().unwrap();

    let number = number_for_uuid(&id);

    match number {
        Err(_) => status::Custom(Status::NotFound, "Could not find uuid".to_string()),
        Ok(number) => {
            match kc_client_jwt() {
                Ok(jwt) => {
                    let mut m = std::collections::HashMap::new();
                    m.insert("cellphone".to_string(), number.clone());
                    m.insert("reg_uuid".to_string(), id.clone());
                    let kc_registration = KcRegistration {
                        enabled: true,
                        attributes: m,
                        username: req.username.clone(),
                        email_verified: false,
                        email: req.email.clone(),
                        first_name: req.first_name.clone(),
                        last_name: req.last_name.clone()
                    };
                    let registration = kc_user_registration(&jwt, &kc_registration);
                    match registration {
                        Ok(response) => {
                            trace!("Reg: {:}", response);
                            remove_validated_phone_number_on_redis(&id);
                            let mut r = kc_reset_user_password(&jwt, &id, &req.password, false);
                            if send_verification_email {
                                r = r.and_then(|()| kc_send_verification_email(&jwt, &id));
                            }
                            match r {
                                Ok(()) => status::Custom(Status::Created, response.to_string()),
                                Err(e) => {
                                    let status = Status::from_code(e.status).unwrap();
                                    status::Custom(status, e.message)
                                }
                            }
                        },
                        Err(error) => {
                            info!("Error: {:?}", error);
                            let status = Status::from_code(error.status).unwrap();
                            status::Custom(status, error.message)
                        }
                    }
                },
                Err(e) => {
                    error!("Error getting JWT {:?}", e);
                    status::Custom(Status::InternalServerError, "Server error".to_string())
                }
            }
        }
    }
}

fn main() {
    let matches = App::new("user-sms-registration")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Register a user")
        .arg(Arg::with_name("version")
            .help("Prints the version")
            .long("version")
            .short("V")
            .takes_value(false)
            .multiple(false)
        )
        .arg(Arg::with_name("verbose")
            .help("-v for debug logs, -vv for trace logs")
            .long("verbose")
            .short("v")
            .takes_value(false)
            .multiple(true)
        )
        .get_matches();
    setup_log(matches.occurrences_of("verbose"));
    dotenv();

    rocket::ignite()
        .mount("/phone", routes![register_phone, phone_validate])
        .mount("/user", routes![register_user]).launch();
}

fn send_otp_sms(otp: &String, number: &String) -> Result<SmsResponse, reqwest::Error>{
    let msg = SmsMessageContent {
            content: format!("PathfinderZA OTP {:}", otp),
            destination: number.clone()
        };
    #[cfg(all(feature = "sms-portal", not(feature = "twilio")))]
    let msg = SmsMessage {
        messages: vec![msg]
    };
    send_sms(&msg)
}

pub const DEFAULT_FILTER_ENV: &str = "LOG_LEVEL";

pub fn setup_log(verbose: u64) {
    let default_filter = match verbose {
        0 => "info",
        1 => "debug",
        _ => "trace"
    };
    let env = env_logger::Env::default()
        .filter_or(DEFAULT_FILTER_ENV, default_filter);

    env_logger::Builder::from_env(env).init();
}

