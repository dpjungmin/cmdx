use super::{get_app, get_string_from_match};

#[test]
fn test_basic() {
    let app = get_app();
    let args = ["echo", "hello", "world"];
    let matches = app.get_matches_from(args);
    let actual = get_string_from_match(&matches);
    let expected = String::from("hello world\n");
    assert_eq!(expected, actual);
}

#[test]
fn test_basic_with_option() {
    let app = get_app();
    let args = ["echo", "-n", "hello", "world"];
    let matches = app.get_matches_from(args);
    let actual = get_string_from_match(&matches);
    let expected = String::from("hello world");
    assert_eq!(expected, actual);
}

#[test]
fn test_empty() {
    let app = get_app();
    let args = ["echo"];
    let matches = app.get_matches_from(args);
    let actual = get_string_from_match(&matches);
    let expected = String::from("\n");
    assert_eq!(expected, actual);
}

#[test]
fn test_empty_with_option() {
    let app = get_app();
    let args = ["echo", "-n"];
    let matches = app.get_matches_from(args);
    let actual = get_string_from_match(&matches);
    let expected = String::from("");
    assert_eq!(expected, actual);
}
