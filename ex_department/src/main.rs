use std::{io, collections::HashMap};

fn main() {
    println!("Add NAME to DEPARTMENT");

    let mut directory: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Write your input as above:");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line.");

        if let Some((name, department)) = extract_info(&buffer) {
            let names = directory.entry(department).or_insert(Vec::new());
            names.push(name);
        }
        println!("{:#?}", directory);
    }
}

fn extract_info(buffer: &str) -> Option<(String, String)> {
    let v: Vec<&str> = buffer.trim().split(' ').collect();
    if v.len() < 4 {
        println!("Too few words!");
        return None;
    }
    if v[0].to_lowercase() != "add" || v[2].to_lowercase() != "to" {
        println!("Missing words: Add or To");
        return None;
    }
    Some((v[1].to_string(), v[3].to_string()))
}
