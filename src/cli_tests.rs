use crate::cli_parser;

use regex::RegexSet;

#[test]
#[should_panic]
fn no_src_dst() {
    //backup-tool -e yx -er
    //source and destination missing -> panic
    let args: Vec<String> = vec![
        String::from("backup-tool"), //program name
        String::from("-e"),
        String::from("yx"),
        String::from("-er"),
    ];
    let _var = cli_parser::parse_options(args);
}

#[test]
fn minimal() {
    //backup-tool ./ ./backup
    let args: Vec<String> = vec![
        String::from("backup-tool"), //program name
        String::from("./"),          //source
        String::from("./backup"),    //destination
    ];
    let conf = cli_parser::parse_options(args);

    assert!(conf.exclude_strings.is_empty());
    assert_eq!(conf.source.to_str().unwrap(), "./");
    assert_eq!(conf.destination.to_str().unwrap(), "./backup");
    assert!(!conf.verbose);
}

#[test]
fn all() {
    //backup-tool -e ".bin" -er "[a-z]*" ./ ./backup -v
    let args: Vec<String> = vec![
        String::from("backup-tool"), //program name
        String::from("-e"),
        String::from(".bin"), //exclude string
        String::from("-er"),
        String::from("[a-z]*"),   //exclude regex
        String::from("./"),       //source
        String::from("./backup"), //destination
        String::from("-v"),       //verbose = true
    ];

    let conf = cli_parser::parse_options(args);

    assert_eq!(conf.exclude_strings, vec![".bin"]);
    assert!(conf.exclude_regex.is_match("az"));
    assert_eq!(conf.source.to_str().unwrap(), "./");
    assert_eq!(conf.destination.to_str().unwrap(), "./backup");
    assert!(conf.verbose);
}
