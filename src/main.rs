use std::fs;
use std::env;
use std::error::Error;
use std::io::{ self, Write };

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut search_query = String::new();

    if args.len() != 2 {
        print!("[search]> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut search_query).expect("Failed to read line");

        search_query = search_query.trim().to_string();
    } else { search_query = args[1].clone(); }

    let current_dir = ".";

    let _ = recursive_search(current_dir, search_query.as_str());

    println!("[search]> Search completed");
}

fn recursive_search(search_path: &str, search_query: &str) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(search_path)?.filter_map(Result::ok) {
        let file_name = match entry.file_name().into_string() {
            Ok(n) => n,
            Err(_) => continue,
        };

        let path = entry.path();

        if file_name.contains(search_query) {
            println!("{}", path.display())
        }

        if !path.is_dir() { continue; }

        let path = match path.to_str() {
            Some(p) => p,
                None => continue,
            };

        match recursive_search(path, search_query) {
            Ok(_) => {},
            Err(..) => continue,
        }
    }

    Ok(())
}
