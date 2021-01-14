use twitchchat::config::Config;

fn main(){
    let args = twitchchat::parse_args();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Argument error: {}", err);
        std::process::exit(1);
    });

    if let Err(err) = twitchchat::run(config) {
        eprintln!("Application error: {}", err);
        std::process::exit(1);
    }
}
