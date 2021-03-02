use std::io::{self, prelude::*};
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    print!("Enter license key: ");
    io::stdout().flush();

    let mut license_key = String::new();
    io::stdin().read_line(&mut license_key);

    let validation: serde_json::Value = reqwest::Client::new()
        .post(&format!(
            "https://api.keygen.sh/v1/accounts/{account_id}/licenses/actions/validate-key",
            account_id = env::var("KEYGEN_ACCOUNT_ID").unwrap()
        ))
        .json(&serde_json::json!({
            "meta": {
                "key": license_key.trim_end()
            }
        }))
        .send()
        .await?
        .json()
        .await?;

    if validation["meta"]["valid"].as_bool().unwrap() {
        println!("License key is valid: code={validation_code} id={license_id}", license_id = validation["data"]["id"], validation_code = validation["meta"]["constant"]);
    } else {
        println!("License key is invalid: code={validation_code}", validation_code = validation["meta"]["constant"]);
    }

    Ok(())
}