use rustrict::{Censor, Type};

pub mod guestbook;
pub mod jellyfin;

fn validate_input(input: &str) -> Result<(), String> {
    for c in input.chars() {
        if c == '<' || c == '>' {
            return Err("pls dont try to hack me :c (don't use < or >)".to_string());
        }

        if c.is_control() {
            return Err("control characters are not allowed".to_string());
        }
        if c.is_whitespace() && c != ' ' {
            return Err("only space is allowed as whitespace".to_string());
        }
        if !c.is_ascii() {
            return Err("only ascii characters are allowed".to_string());
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

async fn send_notification(message: String) {
    #[cfg(debug_assertions)]
    {
        println!("skipping notification:\n{}", message);
        return;
    }

    tokio::spawn(async move {
        reqwest::Client::new()
            .put(std::env::var("NOTIFICATION_URL").unwrap())
            .body(serde_json::json!({ "msgtype": "m.text", "body": message }).to_string())
            .send()
            .await
            .map_err(|e| println!("failed to send notification: {}", e))
            .ok();
    });
}
