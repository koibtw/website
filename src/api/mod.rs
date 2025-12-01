use rustrict::{Censor, Type};
use sqlx::types::chrono::Utc;
use std::error::Error;

pub mod guestbook;
pub mod jellyfin;

fn validate_input(input: &str) -> Result<(), String> {
    for c in input.chars() {
        if c.is_control() {
            return Err("control characters are not allowed".to_string());
        }
        if c.is_whitespace() && c != ' ' {
            return Err("only space is allowed as whitespace".to_string());
        }
    }

    Ok(())
}

// i really wish i wouldnt have to do this
fn censor_input(input: &str) -> Result<String, String> {
    let (censored, analysis) = Censor::from_str(input)
        .with_censor_threshold(Type::OFFENSIVE & Type::MODERATE_OR_HIGHER)
        .with_censor_replacement('#')
        .censor_and_analyze();

    if analysis.is(Type::MEAN & Type::SEVERE) {
        return Err("thats too offensive bruh".to_string());
    }

    Ok(censored)
}

pub async fn send_notification(message: String) {
    tokio::spawn(async move {
        #[cfg(debug_assertions)]
        {
            println!("skipping notification:\n{}", message);
            return;
        }

        let res = reqwest::Client::new()
            .put(format!(
                "{}/{}",
                std::env::var("MATRIX_URL").unwrap(),
                Utc::now().naive_utc()
            ))
            .json(&serde_json::json!({ "msgtype": "m.text", "body": message }))
            .header(
                "Authorization",
                format!("Bearer {}", std::env::var("MATRIX_TOKEN").unwrap()),
            )
            .send()
            .await
            .map_err(|e| eprintln!("failed to send notification: {}", e.source().unwrap_or(&e)))
            .ok();

        if let Some(r) = res
            && !r.status().is_success()
        {
            eprintln!("notification request failed: {}", r.status());
            eprintln!("{}", r.text().await.unwrap_or_default());
        }
    });
}
