extern crate config_parser;
use std::fs::File;

#[test]
fn test_parse_typical_file() {
    let file = File::open("tests/test.cfg").unwrap();
    let cfg = config_parser::parse_file(file).unwrap();
    assert_eq!(cfg.name(), "");
    assert_eq!(cfg.inner()[0].name(), "test");
}
