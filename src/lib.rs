pub fn run(_message: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = run("Hello World!");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }
}
