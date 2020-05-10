use std::env;
use redis_main::{Client, RedisError};

pub fn put_phone_on_redis(otp: &str, uuid: &str, number: &str) -> Result<(), RedisError>{
    let redis_host = env::var("REDIS_HOST")
        .expect("REDIS_HOST must be set");
    use redis::Commands;
    let client = Client::open(redis_host.as_str())?;
    let mut con = client.get_connection()?;
    let id = format!("phone_reg:{:}{:}", uuid, number);
    let _ : () = con.set(id.as_str(), otp)?;
    let _ : () = con.expire(id.as_str(), 300)?;
    Ok(())
}

pub fn compare_phone_otp(otp: &str, uuid: &str, number: &str) -> Result<bool, RedisError> {
    let redis_host = env::var("REDIS_HOST")
        .expect("REDIS_HOST must be set");
    use redis::Commands;
    let client = Client::open(redis_host.as_str())?;
    let mut con = client.get_connection()?;
    let id = format!("phone_reg:{:}{:}", uuid, number);
    let g: String = con.get(id)?;
    Ok(g.eq(otp))
}

pub fn put_validated_phone_number_on_redis(uuid: &str, number: &str) -> Result<(), RedisError> {
    let redis_host = env::var("REDIS_HOST")
        .expect("REDIS_HOST must be set");
    use redis::Commands;
    let client = Client::open(redis_host.as_str())?;
    let mut con = client.get_connection()?;
    let id = format!("phone_val:{:}", uuid);
    let _ : () = con.set(id.as_str(), number)?;
    let _ : () = con.expire(id.as_str(), 172_800)?;    // Let it expire after 2 days
    let id = format!("phone_reg:{:}{:}", uuid, number);
    let _ : () = con.del(id)?;
    Ok(())
}

pub fn remove_validated_phone_number_on_redis(uuid: &str) -> Result<(), RedisError> {
    let redis_host = env::var("REDIS_HOST")
        .expect("REDIS_HOST must be set");
    use redis::Commands;
    let client = Client::open(redis_host.as_str())?;
    let mut con = client.get_connection()?;
    let id = format!("phone_val:{:}", uuid);
    let _ : () = con.del(id.as_str())?;
    Ok(())
}

pub fn number_for_uuid(uuid: &str) -> Result<String, RedisError> {
    let redis_host = env::var("REDIS_HOST")
        .expect("REDIS_HOST must be set");
    use redis::Commands;
    let client = Client::open(redis_host.as_str())?;
    let mut con = client.get_connection()?;
    let id = format!("phone_val:{:}", uuid);
    con.get(id.as_str())
}
