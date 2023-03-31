pub fn run(message: &str) -> Result<(), String> {
    let first_line = message.lines().next().unwrap();

    if !first_line.contains(':') {
        return Err(String::from("No ':' found in subject line"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_should_check_for_colon() {
        let result = run("feat test");
        match result {
            Ok(_) => { panic!("Should fail") }
            Err(error) => { assert_eq!(error, String::from("No ':' found in subject line")) }
        }
    }

    #[test]
    fn test_run_should_check_for_colon_in_first_line() {
        let input = r#"feat test

        but there is a : in the body"#;
        let result = run(input);
        match result {
            Ok(_) => { panic!("Should fail") }
            Err(error) => { assert_eq!(error, String::from("No ':' found in subject line")) }
        }
    }
}
