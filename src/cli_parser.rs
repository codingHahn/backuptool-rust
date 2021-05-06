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

fn error_message(index: &usize, msg: String) -> String {
    String::from("Error at option at index")
        + index.to_string().as_str()
        + String::from("message: ").as_str()
        + msg.as_str()
        + String::from("\nuse \"-?\" for help").as_str()
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

pub fn parse_options(args: Vec<String>) -> Result<ConfStruct, String> {
    // first collect all fields of ConfStruct seperately
    let mut conf_struct = ConfStruct::new(
        Vec::new(),
        RegexSet::new(&[""]).unwrap(), //empty RegexSet, whill be replaced later
        PathBuf::new(),
        PathBuf::new(),
        false,
    );

    // Vec<String> to collect all regex first (RegexSet doesn't have .push())
    let mut conf_exclude_regex: Vec<String> = Vec::new();

    // transform all args into conf_struct content
    let mut index: usize = 1; //first arg: program name
    loop {
        let mut finished: bool = false;
        let res = get_option_type(&args, index);

        index = res.0;
        match res.1 {
            Err(s) => {
                return Err(error_message(&index, s));
            }
            Ok(cli_opt) => match cli_opt {
                CLIOptions::SourceDestination {
                    source,
                    destination,
                } => {
                    if !conf_struct.destination.as_os_str().is_empty() {
                        return Err(error_message(
                            &index,
                            String::from("destination given to often"),
                        ));
                    } else if !conf_struct.source.as_os_str().is_empty() {
                        return Err(error_message(&index, String::from("source given to often")));
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
        if finished {
            break;
        }
    }

    if conf_struct.source.as_os_str().is_empty() || conf_struct.destination.as_os_str().is_empty() {
        return Err(error_message(
            &0,
            String::from("Source or Destination not given"),
        ));
    }
    //return ConfStruct
    conf_struct.exclude_regex = RegexSet::new(conf_exclude_regex).unwrap();
    //return
    Ok(conf_struct)
}

// ConfStruct: Struct with configuration for Backup tool
// String: Error Message
pub fn parse_cli_options() -> Result<ConfStruct, String> {
    let args: Vec<String> = env::args().collect();
    parse_options(args)
}
