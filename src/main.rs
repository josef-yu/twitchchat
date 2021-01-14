use twitchchat::config::Config;

fn main(){
    let args = twitchchat::parse_args();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Argument error: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = twitchchat::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
