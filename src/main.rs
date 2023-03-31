use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file path");
        return ExitCode::FAILURE;
    }

    let file_path: &str = &args[1];

    println!("File path: {}", file_path);
    ExitCode::SUCCESS
}
