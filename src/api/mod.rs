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
