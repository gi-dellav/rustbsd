use puffyrs::flags::FlagParser;

fn args(v: &[&str]) -> Vec<String> {
    let mut all: Vec<String> = vec!["program".to_string()];
    all.extend(v.iter().map(|s| s.to_string()));
    all
}

#[test]
fn no_args() {
    let p = FlagParser::new().bool('n', false).parse(args(&[]));
    assert!(!p.bool('n'));
    assert!(p.args().is_empty());
}

#[test]
fn single_flag() {
    let p = FlagParser::new()
        .bool('n', false)
        .parse(args(&["-n", "hello"]));
    assert!(p.bool('n'));
    assert_eq!(p.args(), &["hello"]);
}

#[test]
fn bundled_flags() {
    let p = FlagParser::new()
        .bool('n', false)
        .bool('e', false)
        .parse(args(&["-ne", "hello"]));
    assert!(p.bool('n'));
    assert!(p.bool('e'));
    assert_eq!(p.args(), &["hello"]);
}

#[test]
fn separate_flags() {
    let p = FlagParser::new()
        .bool('n', false)
        .bool('e', false)
        .parse(args(&["-n", "-e", "hello"]));
    assert!(p.bool('n'));
    assert!(p.bool('e'));
    assert_eq!(p.args(), &["hello"]);
}

#[test]
fn double_dash_terminator() {
    let p = FlagParser::new()
        .bool('n', false)
        .parse(args(&["--", "-n", "hello"]));
    assert!(!p.bool('n'));
    assert_eq!(p.args(), &["-n", "hello"]);
}

#[test]
fn dash_as_positional() {
    let p = FlagParser::new()
        .bool('n', false)
        .parse(args(&["-", "hello"]));
    assert!(!p.bool('n'));
    assert_eq!(p.args(), &["-", "hello"]);
}

#[test]
fn unknown_flag_stops_parsing() {
    let p = FlagParser::new()
        .bool('n', false)
        .parse(args(&["-n", "-x", "hello"]));
    assert!(p.bool('n'));
    assert_eq!(p.args(), &["-x", "hello"]);
}

#[test]
fn unknown_bundled_flag_stops_parsing() {
    let p = FlagParser::new()
        .bool('n', false)
        .parse(args(&["-nx", "hello"]));
    assert!(!p.bool('n'));
    assert_eq!(p.args(), &["-nx", "hello"]);
}

#[test]
fn flag_with_default_true() {
    let p = FlagParser::new().bool('E', true).parse(args(&["hello"]));
    assert!(p.bool('E'));
}

#[test]
fn multiple_independent_flags() {
    let p = FlagParser::new()
        .bool('n', false)
        .bool('e', false)
        .parse(args(&["-ne", "hello"]));
    assert!(p.bool('n'));
    assert!(p.bool('e'));
    assert_eq!(p.args(), &["hello"]);
}

#[test]
fn only_dash_dash() {
    let p = FlagParser::new().bool('n', false).parse(args(&["--"]));
    assert!(!p.bool('n'));
    assert!(p.args().is_empty());
}

#[test]
fn empty_string_arg() {
    let p = FlagParser::new()
        .bool('n', false)
        .parse(args(&["", "hello"]));
    assert!(!p.bool('n'));
    assert_eq!(p.args(), &["", "hello"]);
}

#[test]
fn just_a_dash_flag() {
    let p = FlagParser::new().bool('n', false).parse(args(&["-", "-n"]));
    assert!(!p.bool('n'));
    assert_eq!(p.args(), &["-", "-n"]);
}

#[test]
fn non_flag_stops_parsing() {
    let p = FlagParser::new()
        .bool('n', false)
        .parse(args(&["hello", "-n"]));
    assert!(!p.bool('n'));
    assert_eq!(p.args(), &["hello", "-n"]);
}
