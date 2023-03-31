use std::{ process::ExitCode, fs };

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file path");
        return ExitCode::FAILURE;
    }

    let file_path: &str = &args[1];

    println!("File path: {}", file_path);

    let file_content = fs
        ::read_to_string(file_path)
        .expect("Could not read commit message file content");

    let result = git_hook_commit_msg::run(&file_content);
    match result {
        Ok(_) => { ExitCode::SUCCESS }
        Err(error) => {
            println!("Error: {}", error);
            ExitCode::FAILURE
        }
    }
}
