pub fn run(message: &str) -> Result<(), String> {
    if !message.contains(':') {
        return Err(String::from("No ':' found in subject line"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_should_check_for_colon_in_first_line() {
        let result = run("feat test");
        match result {
            Ok(_) => { panic!("Should fail") }
            Err(error) => { assert_eq!(error, String::from("No ':' found in subject line")) }
        }
    }
}
