use std::fs::*;
use std::io::*;
use std::path::*;

pub fn encode_file(file_path: &String) {
    if let Ok(file_data) = read_file(file_path) {
        let encoded_data = convert_to_monkey(&file_data);
        let file_name = get_file_name(file_path);
        let output_path = format!("{}.monkey", file_name);

        save_file(&output_path, &format!("{}\n{}", file_name, encoded_data));
        println!("File encoded to {}", output_path);
    } else {
        eprintln!("Error reading file {}", file_path);
    }
}

pub fn decode_file(monkey_file_path: &String) {
    if let Ok((original_file_name, encoded_data)) = load_monkey_file(monkey_file_path) {
        let file_data = convert_from_monkey(&encoded_data);
        save_file_raw(&original_file_name, &file_data);
        println!("File decoded to {}", original_file_name);
    } else {
        eprintln!("Error reading monkey file {}", monkey_file_path);
    }
}

fn read_file(filename: &String) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let metadata = file.metadata()?;
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer)?;
    Ok(buffer)
}

fn convert_to_monkey(data: &[u8]) -> String {
    let bin_str = data.iter()
        .map(|&byte| format!("{:08b}", byte))
        .collect::<String>();

    bin_str.chars()
        .map(|bit| if bit == '0' { "oo" } else { "ah" })
        .collect::<String>()
}

fn convert_from_monkey(encoded_data: &String) -> Vec<u8> {
    let bin_str = encoded_data.chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chunk| if chunk == ['o', 'o'] { '0' } else { '1' })
        .collect::<String>();

    bin_str.as_bytes()
        .chunks(8)
        .map(|byte| u8::from_str_radix(&String::from_utf8_lossy(byte), 2).unwrap())
        .collect::<Vec<u8>>()
}

fn load_monkey_file(file_path: &String) -> std::io::Result<(String, String)> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;

    let original_file_name = first_line.trim().to_string();
    let mut encoded_data = String::new();
    reader.read_to_string(&mut encoded_data)?;
    
    Ok((original_file_name, encoded_data))
}

fn save_file(file_path: &String, data: &str) {
    let mut file = File::create(file_path).expect("Unable to create file");
    file.write_all(data.as_bytes()).expect("Unable to write to file");
}

fn save_file_raw(file_path: &String, data: &[u8]) {
    let mut file = File::create(file_path).expect("Unable to create file");
    file.write_all(data).expect("Unable to write to file");
}

fn get_file_name(file_path: &String) -> String {
    Path::new(file_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
