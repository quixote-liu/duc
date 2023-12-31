
use std::{process, fs};
use walkdir::WalkDir;

const HELP_TEXT: &str = "A tool to learn about disk usage, fast!

Usage: duc [FLAGS] [OPTIONS] [SUBCOMMAND] [INPUT]...

Commands:
  interactive  Launch the terminal user interface [aliases: i]
  aggregate    Aggregrate the consumed space of one or more directories or files [aliases: a]
  help         Print this message or the help of the given subcommand(s)

Arguments:
  [INPUT]...  One or more input files or directories. If unset, we will use all entries in the current working directory

Options:
  -t, --threads <THREADS>          The amount of threads to use. Defaults to 0, indicating the amount of logical processors. Set to 1 to use only a single thread [default: 0]
  -f, --format <FORMAT>            The format with which to print byte counts [default: binary] [possible values: metric, binary, bytes, gb, gib, mb, mib]
  -A, --apparent-size              Display apparent size instead of disk usage
  -l, --count-hard-links           Count hard-linked files each time they are seen
  -x, --stay-on-filesystem         If set, we will not cross filesystems or traverse mount points
  -i, --ignore-dirs <IGNORE_DIRS>  One or more absolute directories to ignore. Note that these are not ignored if they are passed as input path [default: /proc /dev /sys /run]
  -h, --help                       Print help (see more with '--help')
  -V, --version                    Print version";

#[derive(Debug, Clone)]
struct Options {
    format: Option<OptionsFormat>,
    help: Option<bool>,
}

#[derive(Debug, Clone)]
pub enum OptionsFormat {
    Metric,
    Binary,
    Bytes,
    GB,
    Gib,
    MB,
    Mib,
}

#[derive(Debug,Clone)]
enum SubCommand {
    Aggregate,
    Help,
}

#[derive(Debug, Default)]
pub struct Command {
    options: Option<Options>,
    sub_command: Option<SubCommand>,
    input: Vec<String>,
}

impl Command {
    pub fn from_args(mut args: impl Iterator<Item = String>) -> Command {
        args.next(); // filter the first argument

        // options
        let mut options_ok = false;
        let mut options_format: (Option<OptionsFormat>, bool) = (None, false);
        let mut options_help : Option<bool> = None;

        // subcommand
        let mut sub_command_ok = false;
        let mut sub_command: Option<SubCommand> = None;

        // input
        let mut input: Vec<String> = Vec::new();

        for arg in args {
            if !options_ok {
                if options_format.1 {
                    if options_format.0.is_some() {
                        println!("error: duplicated option {} found\n", arg.clone());
                        process::exit(1);
                    }
                    options_format.0 = match arg.as_str() {
                        "metric"  => Some(OptionsFormat::Metric),
                        "binary" => Some(OptionsFormat::Binary),
                        "bytes"  => Some(OptionsFormat::Bytes),
                        "gb" => Some(OptionsFormat::GB),
                        "gib" => Some(OptionsFormat::Gib),
                        "mb" => Some(OptionsFormat::MB),
                        "mib" => Some(OptionsFormat::Mib),
                        _ => None,
                    };
                    if options_format.0.is_none() {
                        println!("error: invalid option {} found\n", arg.clone());
                        process::exit(1);
                    }
                    options_format.1 = false;                
                    continue
                }
    
                if arg == "-f" || arg == "--format" {
                    options_format.1 = true;
                    continue
                }
                if arg == "-h" || arg == "--help" {
                    options_help = Some(true);
                    continue
                }

                options_ok = true;
                continue
            }

            if !sub_command_ok {
                sub_command = {
                    if arg == "aggregate" {
                        Some(SubCommand::Aggregate)
                    } else if arg == "help" {
                        Some(SubCommand::Help)
                    } else {
                        input.push(arg);
                        None
                    }
                };
                sub_command_ok = true;
                continue;
            }

            input.push(arg);
        }
        let options = Some(Options{
            format: options_format.0,
            help: options_help,
        });
        if sub_command.is_none() {
            sub_command = Some(SubCommand::Aggregate);
        }
        if input.len() == 0 {
            input.push("./".to_string());
        }
        Command { options: options, sub_command: sub_command, input: input }
    }

    fn exist_with_error(message: &str) {
        println!("{message}");
        process::exit(1);
    }

    fn print_help_text() {
        println!("{HELP_TEXT}");
        process::exit(0);
    }

    pub fn run(&self) {
        let opts: Option<Options> = self.options.clone();

        // if the option is help
        if let Some(opts) = opts {
            if opts.help.is_some_and(|e| e == true) {
                Self::print_help_text();
            }
            match self.sub_command.clone() {
                Some(sub_cmd) => {
                    match sub_cmd {
                        SubCommand::Help => {
                            Self::print_help_text();
                        },
                        SubCommand::Aggregate => {
                            // read size from file
                            let mut file_info = Vec::new();
                            for file_path in self.input.clone() {
                                match fs::metadata(file_path.as_str()) {
                                    Ok(_) => {
                                        let file_size = count_file_size(file_path.as_str());
                                        let file_size_format = format_file_size(opts.clone(), file_size);
                                        let element = (file_path, file_size_format);
                                        file_info.push(element);
                                    },
                                    Err(e) => {
                                        let message = format!("the file {} error: {:?}", file_path, e);
                                        Self::exist_with_error(message.as_str());
                                    },
                                }
                            };
                            // output file size message
                            file_info.into_iter().for_each(|element| {
                                let mut line = String::new();
                                line.push_str(&element.0);
                                line.push_str("\t");
                                line.push_str(&element.1); // input file name
                                line.push_str("\n");
                                println!("{}", line);
                            })
                        },
                    }
                },
                None => {},
            }
        }
    }
}

fn count_file_size(file_path: &str) -> u64 {
    if file_path == "" {
        return 0;
    }
    if let Ok(metadata) = fs::metadata(file_path) {
        if metadata.is_dir() {
            let mut file_total_size: u64 = 0;
            WalkDir::new(file_path).into_iter().for_each(|file| {
                if let Ok(f) = file {
                    let file_info = f.metadata().unwrap();
                    if file_info.is_dir() {
                        if let Some(child_file_path) = f.path().to_str() {
                            let size = count_file_size(child_file_path);
                            file_total_size += size;
                        }
                    } else {
                        file_total_size += file_info.len();
                    }
                }
            });
            return file_total_size;
        } else {
            return metadata.len();
        }
    }
    0
}

fn format_file_size(options: Options, mut file_size: u64) -> String {
    let mut unit = String::new();
    if let Some(format) = options.format {
        match format {
            OptionsFormat::Metric => {
                file_size /= 1000; 
                unit.push_str("KB");
            },
            OptionsFormat::Binary => {
                file_size /= 1024; 
                unit.push_str("KiB");
            },
            OptionsFormat::Bytes => {
                unit.push_str("b");
            },
            OptionsFormat::GB => {
                file_size /= 1000*1000*1000; 
                unit.push_str("GB");
            },
            OptionsFormat::Gib => {
                file_size /= 1024*1024*1024; 
                unit.push_str("GiB");
            },
            OptionsFormat::MB => {
                file_size /= 1000*1000; 
                unit.push_str("MB");
            },
            OptionsFormat::Mib => {
                file_size /= 1024*1024; 
                unit.push_str("MiB");
            },
        }
    };
    let mut file_size_formated = file_size.to_string();
    file_size_formated.push_str(&unit);
    return file_size_formated;
}

#[cfg(test)]
mod tests {
    use crate::cmd;
    
    #[test]
    fn test_count_file_size() {
        let res = cmd::count_file_size("../src");
        println!("the result = {}", res);
    }
}