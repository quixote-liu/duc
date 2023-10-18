
use std::process;

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

#[derive(Debug)]
struct Options {
    format: Option<OptionsFormat>,
    help: Option<bool>,
}

#[derive(Debug)]
pub enum OptionsFormat {
    Metric,
    Binary,
    Bytes,
    GB,
    Gib,
    MB,
    Mib,
}

#[derive(Debug)]
enum SubCommand {
    Aggregate,
    Help,
}

#[derive(Debug)]
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

    pub fn run(&self) {

    }
}

