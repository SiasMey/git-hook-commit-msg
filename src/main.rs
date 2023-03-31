use std::process::ExitCode;
use std::fs;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file path");
        return ExitCode::FAILURE;
    }

    let file_path: &str = &args[1];

    println!("File path: {}", file_path);

    let _file_content = fs
        ::read_to_string(file_path)
        .expect("Could not read commit message file content");
    ExitCode::SUCCESS
}
