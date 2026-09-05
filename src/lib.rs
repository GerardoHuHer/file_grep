pub mod funciones {

    use rayon::prelude::*;
    use regex::Regex;
    use std::sync::{Arc, Mutex};
    use std::{env, fs, process};

    pub fn run() -> () {
        let mut results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let path: String = match get_argument() {
            Some(a) => a,
            None => String::from(" "),
        };

        if path == " " {
            eprintln!("Uso: \nfile_grep <file | directory>");
            process::exit(200);
        }

        let regex_string = format!(r".*{}.*", regex::escape(&path));
        let path_regex = Arc::new(Regex::new(&regex_string).unwrap());
        search_though_files(
            "/home/gd15/",
            Arc::clone(&path_regex),
            Arc::clone(&mut results),
        );

        let final_results = results.lock().unwrap();
        for result in final_results.iter() {
            println!("{}", result);
        }
    }

    fn get_argument() -> Option<String> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            return None;
        }
        let name = String::clone(&args[1]);
        return Some(name);
    }

    fn search_though_files(
        dir: &str,
        path_regex: Arc<Regex>,
        results: Arc<Mutex<Vec<String>>>,
    ) -> () {
        let entries: Vec<_> = match fs::read_dir(dir) {
            Ok(e) => e.flatten().collect(),
            Err(_) => return,
        };
        entries.par_iter().for_each(|entry| {
            let is_real_dir = entry.file_type().map(|m| m.is_dir()).unwrap_or(false);

            if path_regex.is_match(&entry.file_name().to_string_lossy()) {
                let full_path = entry.path().to_string_lossy().to_string();
                results.lock().unwrap().push(full_path.clone());
            }

            if is_real_dir {
                let dir_name = entry.file_name().to_string_lossy().to_string();
                if !should_skip(&dir_name) {
                    let full_path = entry.path().to_string_lossy().to_string();
                    search_though_files(&full_path, Arc::clone(&path_regex), Arc::clone(&results));
                }
            }
        });
    }

    fn should_skip(name: &str) -> bool {
        matches!(
            name,
            ".steam" | ".cache" | "node_modules" | ".git" | "proc" | "sys" | "deps" | ".local"
        )
    }
}
