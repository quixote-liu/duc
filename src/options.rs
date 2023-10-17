use std::process;

#[derive(Debug)]
pub struct Options{
    format: Option<OptionsFormat>,
    help: Option<bool>,
}

#[derive(Debug)]
enum OptionsFormat {
    Metric,
    Binary,
    Bytes,
    GB,
    Gib,
    MB,
    Mib,
}

impl Options {
    pub fn new(mut args: impl Iterator<Item = String>) -> Options {
        args.next();

        let mut config_format: (Option<OptionsFormat>, bool) = (None, false);
        let mut config_help : Option<bool> = None;

        for arg in args {
            if config_format.1 {
                if config_format.0.is_some() {
                    println!("error: duplicated option {} found", arg.clone());
                    process::exit(1);
                }
                config_format.0 = match arg.as_str() {
                    "metric"  => Some(OptionsFormat::Metric),
                    "binary" => Some(OptionsFormat::Binary),
                    "bytes"  => Some(OptionsFormat::Bytes),
                    "gb" => Some(OptionsFormat::GB),
                    "gib" => Some(OptionsFormat::Gib),
                    "mb" => Some(OptionsFormat::MB),
                    "mib" => Some(OptionsFormat::Mib),
                    _ => None,
                };
                if config_format.0.is_none() {
                    println!("error: invalid option {} found", arg.clone());
                    process::exit(1);
                }
                config_format.1 = false;                
                continue
            }

            if arg == "-f" || arg == "--format" {
                config_format.1 = true;
                continue
            }
            if arg == "-h" || arg == "--help" {
                config_help = Some(true);
                continue
            }

            println!("error: invalid option {} found", arg.clone());
            process::exit(1);
        }

        Options { format: config_format.0, help: config_help }
    }
}