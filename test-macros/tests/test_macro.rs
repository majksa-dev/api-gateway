use std::env;

#[test]
fn test_macro() {
    env::set_var("RUST_BACKTRACE", "0");
    let t = trybuild::TestCases::new();
    t.pass("tests/test_macro/01-parse-attributes.rs");
    t.compile_fail("tests/test_macro/02-missing-servers-attribute.rs");
    t.compile_fail("tests/test_macro/03-missing-config-attribute.rs");
}
