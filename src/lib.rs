pub const COMMIT_TYPES: [&str; 8] = [
    "feat",
    "fix",
    "docs",
    "style",
    "refactor",
    "perf",
    "test",
    "chore",
];

pub fn run(message: &str) -> Result<(), String> {
    let first_line = message.lines().next().unwrap();

    if !first_line.contains(':') {
        return Err(String::from("No ':' found in subject line"));
    }
    if !first_line.contains(": ") {
        return Err(String::from("No space found after ':' in subject line"));
    }

    let commit_type = first_line.split(": ").next().unwrap();
    if COMMIT_TYPES.contains(&commit_type) {
        return Ok(());
    }

    Err(String::from("Fallthrough"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_should_check_for_colon() {
        let input = r#"feat test"#;
        let result = run(input);
        assert_eq!(result, Err(String::from("No ':' found in subject line")));
    }

    #[test]
    fn test_run_should_check_for_colon_in_first_line() {
        let input = r#"feat test

        but there is a : in the body"#;
        let result = run(input);
        assert_eq!(result, Err(String::from("No ':' found in subject line")));
    }

    #[test]
    fn test_run_should_check_for_type_subject_seperation() {
        let input = r#"feat:test"#;
        let result = run(input);
        assert_eq!(result, Err(String::from("No space found after ':' in subject line")));
    }

    #[test]
    fn test_run_should_check_for_accepted_type_feat() {
        let input = r#"feat: test"#;
        let result = run(input);
        assert_eq!(result, Ok(()));
    }
}
