extern crate config_parser;
use std::fs::File;

#[test]
fn test_parse_typical_file() {
    let file = File::open("tests/test.cfg").unwrap();
    let cfg = config_parser::parse_file(file).unwrap();
    assert_eq!(cfg.name(), "");
    assert_eq!(cfg.len(), 0);
    let i = &cfg.inner()[0];
    assert_eq!(i.name(), "test");
    assert_eq!(i.len(), 1);
    assert_eq!(i.get(0), "shit");
}
