use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let query = &args[1];
    let file_path = &args[1];
    
    let contents = fs::read_to_string(file_path).expect("Something Fucked Up");
    let mut bin = "".to_string();
    let mut monkey = "".to_string();
    for character in contents.clone().into_bytes() {
        bin += &format!("0{:b}", character);
    }
    for char in bin.clone().chars(){
        if char == '0' {
            monkey += "oo ";
        }else{
            monkey += "ah "
        }
    }
    println!("\"{}\" in monkey is {}", contents, monkey);
}
