use std::fs;
use std::env;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let request = &args[1];
    let file_path = &args[2];
    
    let bin = convert_to_bytes(file_path);
    let mut monkey = "".to_string();

    if request == "-w"{
        for char in bin.clone(){
            if char == 0 {
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
