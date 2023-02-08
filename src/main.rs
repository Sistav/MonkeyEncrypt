use std::fs;
use std::env;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let request = &args[1];
    let file_path = &args[2];
    
    let bytes_or_something_idk = convert_to_bytes(file_path);
    let mut monkey = "".to_string();
    let mut bin = "".to_string();
    for character in bytes_or_something_idk.clone(){
        bin += &format!("{:b}", character);
    }
    if request == "-w"{
        
        for char in bin.clone().chars(){
            if char == '0' {
                monkey += "oo ";
            }else{
                monkey += "ah "
            }
        }
        println!("{}", monkey);
    }
    else if request == "-r" {
        println!("read");
    }
    else {
        println!("Not a valid option");
    }
   
}

fn convert_to_bytes(filename: &String) -> Vec<u8> {
    let mut f = fs::File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read file");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buff");
    return buffer;
}
