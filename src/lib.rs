pub struct Config{
    format: String,
}

impl Config {
    fn new(&self, mut args: impl Iterator) -> Config {
        args.next();

        let mut config_format = String::new();

        for arg in args.into_iter() {
            
        };

        Config { format: config_format }
    }
}