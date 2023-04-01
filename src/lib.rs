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

    let subject: Vec<&str> = first_line.split(": ").collect();
    if subject[1].is_empty() {
        return Err(String::from("Empty subject not accepted"));
    }

    let commit_type_and_scope = subject[0];

    let commit_type = if commit_type_and_scope.contains('(') {
        commit_type_and_scope.split('(').next().unwrap()
    } else {
        commit_type_and_scope
    };

    if COMMIT_TYPES.contains(&commit_type.trim_end_matches('!')) {
        return Ok(());
    }

    Err(format!("Commit type '{}' not accepted", &commit_type))
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

    #[test]
    fn test_run_should_check_for_accepted_type_fix() {
        let input = r#"fix: test"#;
        let result = run(input);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_run_should_check_for_accepted_type_docs() {
        let input = r#"docs: test"#;
        let result = run(input);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_run_should_check_for_accepted_type_style() {
        let input = r#"style: test"#;
        let result = run(input);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_run_should_check_for_accepted_type_refactor() {
        let input = r#"refactor: test"#;
        let result = run(input);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_run_should_check_for_accepted_type_perf() {
        let input = r#"perf: test"#;
        let result = run(input);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_run_should_check_for_accepted_type_test() {
        let input = r#"test: test"#;
        let result = run(input);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_run_should_check_for_accepted_type_chore() {
        let input = r#"chore: test"#;
        let result = run(input);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_run_should_check_for_fail_unaccepted_type_other() {
        let input = r#"other: test"#;
        let result = run(input);
        assert_eq!(result, Err(String::from("Commit type 'other' not accepted")));
    }

    #[test]
    fn test_run_should_allow_optional_scope() {
        let input = r#"feat(scope): test"#;
        let result = run(input);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_run_should_allow_optional_exclamation_breaking_change() {
        let input = r#"feat!: test"#;
        let result = run(input);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_run_should_allow_optional_exclamation_breaking_change_with_scope() {
        let input = r#"feat(scope)!: test"#;
        let result = run(input);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_run_should_deny_empty_subject() {
        let input = r#"feat: "#;
        let result = run(input);
        assert_eq!(result, Err(String::from("Empty subject not accepted")));
    }
}
