use chrono::offset::Utc;
use chrono::DateTime;
use std::fs::File;
use std::io::{self, Write};
use std::time::SystemTime;
use uuid::Uuid;
fn main() {
    loop {
        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();
        let now_time: String = datetime.format("%Y-%m-%d %T").to_string();
        println!("Please enter the number of IDs to be generated and press Enter:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let num = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Please enter the number!!");
                continue;
            }
        };
        println!("Please enter the name of the file to be generated or press the Enter key to generate it automatically:");
        let mut file_name = String::new();
        io::stdin()
            .read_line(&mut file_name)
            .expect("Failed to read line");
        file_name = file_name.trim().to_string();
        file_name += &now_time;
        let result = creator(num);
        println!("{:#?}", result);
        result_to_file(result, file_name);
        print!("Happy Hacking");
        break;
    }
}
fn creator(num: u32) -> Vec<(u32, String)> {
    let mut vec: Vec<(u32, String)> = Vec::new();
    for i in 1..(num + 1) {
        let id = Uuid::new_v4();
        vec.push((i, id.to_string()));
    }
    vec
}

fn result_to_file(result: Vec<(u32, String)>, file_name: String) {
    let mut file =
        File::create(format!("{}{}", file_name, ".txt")).expect("Failed to create file!!");
    let content: String = format!("{:#?}", result);
    file.write_all(content.as_bytes())
        .expect("Failed to write file!!");
    println!("{}file generated.", file_name);
}
