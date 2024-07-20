use std::{env, error::Error};

use base64::prelude::*;
use serde::Deserialize;

static KEY_ARG_NAME: &str = "--key=";

#[derive(Deserialize, Debug)]
struct HelpMeUnpackTask {
    bytes: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let key_arg = env::args().find(|arg| arg.contains(KEY_ARG_NAME));

    match key_arg {
        Some(key_arg) => {
            let key = key_arg.split("=").last().unwrap_or_default();
            let client = reqwest::Client::new();
            let result: HelpMeUnpackTask = client
                .get(format!(
                    "https://hackattic.com/challenges/{}/problem?access_token={}",
                    "help_me_unpack", key
                ))
                .send()
                .await?
                .json()
                .await?;

            let bytes = BASE64_STANDARD.decode(result.bytes);

            match bytes {
                Ok(mut bytes) => {
                    dbg!(&bytes);
                    let i32_bytes: [u8; 4] =
                        bytes.drain(..4).collect::<Vec<u8>>().try_into().unwrap();
                    let i32_value = i32::from_le_bytes(i32_bytes);
                    let u32_bytes: [u8; 4] =
                        bytes.drain(..4).collect::<Vec<u8>>().try_into().unwrap();
                    let u32_value = u32::from_le_bytes(u32_bytes);
                    // we need to take 4, because all values 4 bytes long
                    let i16_bytes: [u8; 4] =
                        bytes.drain(..4).collect::<Vec<u8>>().try_into().unwrap();
                    // use only first two of four for 2 byte integer
                    let i16_value = i16::from_le_bytes([i16_bytes[0], i16_bytes[1]]);
                    let f32_bytes: [u8; 4] =
                        bytes.drain(..4).collect::<Vec<u8>>().try_into().unwrap();
                    let f32_value = f32::from_le_bytes(f32_bytes);
                    let f64_bytes: [u8; 8] =
                        bytes.drain(..8).collect::<Vec<u8>>().try_into().unwrap();
                    let f64_value = f64::from_le_bytes(f64_bytes);
                    let f64_bytes_be: [u8; 8] =
                        bytes.drain(..8).collect::<Vec<u8>>().try_into().unwrap();
                    let f64_value_be = f64::from_be_bytes(f64_bytes_be);

                    let json = &serde_json::json!({
                        "int": i32_value,
                        "uint": u32_value,
                        "short": i16_value,
                        "float": f32_value,
                        "double": f64_value,
                        "big_endian_double": f64_value_be,
                    });
                    let res = client
                        .post(format!(
                            "https://hackattic.com/challenges/{}/solve?access_token={}",
                            "help_me_unpack", key
                        ))
                        .json(&json)
                        .send()
                        .await?
                        .text()
                        .await?;
                    dbg!(&res);
                }
                _ => println!("Can't get bytes from result."),
            }
        }
        _ => println!("Please provide {} argument.", KEY_ARG_NAME),
    }

    Ok(())
}
