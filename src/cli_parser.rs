use crate::configuration;
use std::env;
use std::result::Result;

enum CLIOptions {
    SourceDestination { source: String, destination: String },
    ExcludePattern { pattern: String },
    Help,
    End,
}

fn get_option_type(args: &Vec<String>, index: usize) -> (usize, Result<CLIOptions, String>) {
    match args.get(index) {
        // Exclude pattern in next element
        Option::None => return (index, Result::Ok(CLIOptions::End)),
        Option::Some(arg0) => match arg0.as_str().trim() {
            "-e" => match args.get(index + 1) {
                Option::None => {
                    return (
                        index,
                        Result::Err(String::from("missing Argument after -e")),
                    )
                }
                Option::Some(arg1) => {
                    return (
                        index + 2,
                        Result::Ok(CLIOptions::ExcludePattern {
                            pattern: String::from(arg1),
                        }),
                    )
                }
            },
            // help tag
            "-?" => return (index + 1, Result::Ok(CLIOptions::Help)),
            // other: source + destination
            _ => match args.get(index + 1) {
                Option::None => return (index, Result::Err(String::from("no destination given"))),
                Option::Some(arg1) => {
                    return (
                        index + 2,
                        Result::Ok(CLIOptions::SourceDestination {
                            source: String::from(arg0),
                            destination: String::from(arg1),
                        }),
                    )
                }
            },
        },
    }
}

fn print_error(index: &usize, msg: String) {
    println!("Error at option at index {}, message: \"{}\"", index, msg);
    println!("use \"-?\" for help");
}

fn print_help_message() {
    println!("usage: backup-tool <-e EXCLUDE_PATTERNS> [Source] [Destination]");
    println!("-e: Pattern to exclude (can be given more than once)");
    println!("-?: help");
    println!("Source: the source for the backup");
    println!("Destination: the destination for the backup");
}

pub fn parse_cli_options() -> configuration::ConfStruct {
    let mut conf_struct = configuration::ConfStruct {
        exclude_patterns: Vec::new(),
        source: String::new(),
        destination: String::new(),
        help: false,
    };

    let args: Vec<String> = env::args().collect();
    // transform all args into conf_struct content
    let mut index: usize = 1; //first arg: program name
    loop {
        let mut error: bool = false;
        let mut finished: bool = false;
        let res = get_option_type(&args, index);

        index = res.0;
        match res.1 {
            Err(s) => {
                print_error(&index, s);
                error = true;
                //use panic!() instead?
            }
            Ok(cli_opt) => match cli_opt {
                CLIOptions::SourceDestination {
                    source,
                    destination,
                } => {
                    if !conf_struct.destination.is_empty() {
                        print_error(&index, String::from("destination given to often"));
                        error = true;
                    } else if !conf_struct.source.is_empty() {
                        print_error(&index, String::from("source given to often"));
                        error = true;
                    } else {
                        conf_struct.destination = destination;
                        conf_struct.source = source;
                    }
                }
                CLIOptions::ExcludePattern { pattern } => {
                    conf_struct.exclude_patterns.push(pattern)
                }
                CLIOptions::End => finished = true,
                CLIOptions::Help => print_help_message(),
            },
        }

        if error {
            break;
        }
        if finished {
            break;
        }
    }

    conf_struct
}
