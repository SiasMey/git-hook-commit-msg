pub fn run(_message: &str) -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let result = run("Hello World!");
        match result {
            Ok(_) => { panic!("Should fail") }
            Err(error) => { assert_eq!(error, String::from("No commit type provided")) }
        }
    }
}
