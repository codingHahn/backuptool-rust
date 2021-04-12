use crate::configuration;
use std::env;
use std::result::Result;
use std::path::PathBuf;

use regex::RegexSet;

enum CLIOptions {
    SourceDestination { source: PathBuf, destination: PathBuf },
    ExcludeString { string: String },
    ExcludeRegex { regex: String },
    Help,
    End,
}

fn get_option_type(args: &Vec<String>, index: usize) -> (usize, Result<CLIOptions, String>) {
    match args.get(index) {
        // Exclude pattern in next element
        Option::None => return (index, Result::Ok(CLIOptions::End)),
        Option::Some(arg0) => match arg0.as_str().trim() {
            // Exclude String:
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
                        Result::Ok(CLIOptions::ExcludeString {
                            string: String::from(arg1),
                        }),
                    )
                }
            },
            //Exclude regex
            "-er" => match args.get(index + 1) {
                Option::None => {
                    return (
                        index,
                        Result::Err(String::from("missing Argument after -er")),
                    )
                }
                Option::Some(arg1) => {
                    return (
                        index + 2,
                        Result::Ok(CLIOptions::ExcludeRegex {
                            regex: String::from(arg1),
                        }),
                    )
                }
            }
            // help tag
            "-?" => return (index + 1, Result::Ok(CLIOptions::Help)),
            // other: source + destination
            _ => match args.get(index + 1) {
                Option::None => return (index, Result::Err(String::from("no destination given"))),
                Option::Some(arg1) => {
                    return (
                        index + 2,
                        Result::Ok(CLIOptions::SourceDestination {
                            source: PathBuf::from(arg0),
                            destination: PathBuf::from(arg1),
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
    println!("usage: backup-tool <-e EXCLUDE_PATTERNS> <-er Exclude_REGEX> [Source] [Destination]");
    println!("-e: Folder or File to exclude (can be given more than once)");
    println!("-er: Regular expression to exclude (can be given more than once)");
    println!("-?: help");
    println!("Source: the source for the backup");
    println!("Destination: the destination for the backup");
}

pub fn parse_options(args: Vec<String>) -> configuration::ConfStruct {
    // first collect all fields of ConfStruct seperately
    let mut conf_exclude_strings: Vec<String> = Vec::new();
    let mut conf_exclude_regex: Vec<String> = Vec::new();
    let mut conf_source: PathBuf = PathBuf::new();
    let mut conf_dest : PathBuf = PathBuf::new();
    let mut conf_help: bool = false;

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
                    if !conf_dest.as_os_str().is_empty() {
                        print_error(&index, String::from("destination given to often"));
                        error = true;
                    } else if !conf_source.as_os_str().is_empty() {
                        print_error(&index, String::from("source given to often"));
                        error = true;
                    } else {
                        conf_dest = destination;
                        conf_source = source;
                    }
                }
                CLIOptions::ExcludeString { string } => {
                    conf_exclude_strings.push(string);
                },
                CLIOptions::ExcludeRegex { regex } => {
                    conf_exclude_regex.push(regex);
                },
                CLIOptions::End => finished = true,
                CLIOptions::Help => {
                    print_help_message();
                    conf_help = true;
                }
            },
        }

        if error {
            break;
        }
        if finished {
            break;
        }
    }
    
    //return ConfStruct
    configuration::ConfStruct {
        exclude_strings: conf_exclude_strings,
        exclude_regex: RegexSet::new(conf_exclude_regex).unwrap(),
        destination: conf_dest,
        source: conf_source,
        help: conf_help,
    }
}

pub fn parse_cli_options() -> configuration::ConfStruct {
    let args: Vec<String> = env::args().collect();
    parse_options(args)
}
