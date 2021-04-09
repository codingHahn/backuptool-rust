use crate::configuration;
use std::env;
use std::result::Result;

enum CLIOptions {
    SourceDestination(String, String),
    ExcludePattern(String),
    Help,
    End,
}

fn get_option_type(args: &Vec<String>, index: usize) -> (usize, Result<CLIOptions, String>) {
    match args.get(index) {
        // Exclude pattern in next element
        Option::None => return (index, Result::Ok(CLIOptions::End)),
        Option::Some(arg0) => match arg0.as_str() {
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
                        Result::Ok(CLIOptions::ExcludePattern(String::from(arg1))),
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
                        Result::Ok(CLIOptions::SourceDestination(
                            String::from(arg0),
                            String::from(arg1),
                        )),
                    )
                }
            },
        },
    }
}
