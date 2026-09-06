pub mod funciones {

    use clap::Parser;
    use rayon::prelude::*;
    use regex::Regex;
    use std::sync::{Arc, Mutex};
    use std::{fs, process};

    #[derive(Parser, Debug)]
    #[command(name = "file_grep")]
    #[command(about="Busca archivos y carpetas por nombre en el sistema", long_about = None)]
    struct Args {
        pattern: String,

        #[arg(long)]
        here: bool,
    }

    pub fn run() -> () {
        let mut results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

        let args = Args::parse();
        let search_dir: String = match args.here {
            true => String::from("./"),
            false => String::from("/home/gd15/"),
        };

        if args.pattern == " " {
            eprintln!("Uso: \nfile_grep <file | directory>");
            process::exit(200);
        }

        let regex_string = format!(r".*{}.*", regex::escape(&args.pattern));
        let path_regex = Arc::new(Regex::new(&regex_string).unwrap());
        search_though_files(
            &search_dir,
            Arc::clone(&path_regex),
            Arc::clone(&mut results),
        );

        let final_results = results.lock().unwrap();
        for result in final_results.iter() {
            println!("{}", result);
        }
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
