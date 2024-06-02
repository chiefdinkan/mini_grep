use std::env;
use mini_grep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing in arguments!");
        std::process::exit(1);
    }); 
    if let Err(e) = mini_grep::run(config){
        eprintln!("application error {}", e);
        std::process::exit(1);
    }
}
