use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: ./monkeyencrypt -w|-r file_path");
        return;
    }

    let operation = &args[1];
    let file_path = &args[2];

    match operation.as_str() {
        "-w" => monkeyencrypt::encode_file(file_path),
        "-r" => monkeyencrypt::decode_file(file_path),
        _ => eprintln!("Invalid option. Use -w to write or -r to read."),
    }
}

