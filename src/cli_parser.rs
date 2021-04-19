use crate::configuration::ConfStruct;
use std::env;
use std::path::PathBuf;

use regex::RegexSet;

enum CLIOptions {
    SourceDestination {
        source: PathBuf,
        destination: PathBuf,
    },
    ExcludeString {
        string: String,
    },
    ExcludeRegex {
        regex: String,
    },
    Help,
    Verbose,
    End,
}

fn get_option_type(args: &Vec<String>, index: usize) -> (usize, Result<CLIOptions, String>) {
    match args.get(index) {
        // Exclude pattern in next element
        None => return (index, Ok(CLIOptions::End)),
        Some(arg0) => match arg0.as_str().trim() {
            // Exclude String:
            "-e" => match args.get(index + 1) {
                None => return (index, Err(String::from("missing Argument after -e"))),
                Some(arg1) => {
                    return (
                        index + 2,
                        Ok(CLIOptions::ExcludeString {
                            string: String::from(arg1),
                        }),
                    )
                }
            },
            //Exclude regex
            "-er" => match args.get(index + 1) {
                None => return (index, Err(String::from("missing Argument after -er"))),
                Some(arg1) => {
                    return (
                        index + 2,
                        Ok(CLIOptions::ExcludeRegex {
                            regex: String::from(arg1),
                        }),
                    )
                }
            },
            // help tag
            "-?" => return (index + 1, Ok(CLIOptions::Help)),
            // verbose tag
            "-v" => return (index + 1, Ok(CLIOptions::Verbose)),
            // other: source + destination
            _ => match args.get(index + 1) {
                None => return (index, Err(String::from("no destination given"))),
                Some(arg1) => {
                    return (
                        index + 2,
                        Ok(CLIOptions::SourceDestination {
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
    // Help message:
    println!(
        r#"
    usage: backup-tool [options] <source> <destination>
        <source>        : Sourcepath for the backup
        <destination>   : Destination path for the backup
        options :
            -e          : Folder or File to exclude (can be given more than once)
            -er         : Regular expression to exclude (can be given more than once)
            -v          : verbose output
            -?          : help
    "#
    );
    // End Help message
}

pub fn parse_options(args: Vec<String>) -> ConfStruct {
    // first collect all fields of ConfStruct seperately
    let mut conf_struct = ConfStruct {
        exclude_strings: Vec::new(),
        exclude_regex: RegexSet::new(&[""]).unwrap(), //empty RegexSet, whill be replaced later
        source: PathBuf::new(),
        destination: PathBuf::new(),
        verbose: false,
    };

    // Vec<String> to collect all regex first (RegexSet doesn't have .push())
    let mut conf_exclude_regex: Vec<String> = Vec::new();

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
                    if !conf_struct.destination.as_os_str().is_empty() {
                        print_error(&index, String::from("destination given to often"));
                        error = true;
                    } else if !conf_struct.source.as_os_str().is_empty() {
                        print_error(&index, String::from("source given to often"));
                        error = true;
                    } else {
                        conf_struct.destination = destination;
                        conf_struct.source = source;
                    }
                }
                CLIOptions::ExcludeString { string } => {
                    conf_struct.exclude_strings.push(string);
                }
                CLIOptions::ExcludeRegex { regex } => {
                    conf_exclude_regex.push(regex);
                }
                CLIOptions::End => finished = true,
                CLIOptions::Verbose => conf_struct.verbose = true,
                CLIOptions::Help => {
                    print_help_message();
                }
            },
        }

        if error {
            panic!();
        }
        if finished {
            break;
        }
    }

    if conf_struct.source.as_os_str().is_empty() || conf_struct.destination.as_os_str().is_empty() {
        print_error(&0, String::from("Source or Destination not given"));
        panic!();
    }
    //return ConfStruct
    conf_struct.exclude_regex = RegexSet::new(conf_exclude_regex).unwrap();
    //return
    conf_struct
}

pub fn parse_cli_options() -> ConfStruct {
    let args: Vec<String> = env::args().collect();
    parse_options(args)
}
