use near_sdk::env;
use serde_json::json;

// * LOG GENERATOR
// ? This is implemented in case we want to use TheGraph, which might be a good idea.

pub fn create_log(result: &str, method: &str, slug: &str, msg: &str) {
    let log_msg = json!({
        "result": &result,
        "method": &method,
        "guild": &slug,
        "user": env::predecessor_account_id(),
        "date": env::block_timestamp(),
        "message": msg
    });
    env::log(format!("{}", log_msg.to_string()).as_bytes());
}
