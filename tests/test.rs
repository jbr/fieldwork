#[test]
fn macrotest() {
    println!("`MACROTEST=overwrite cargo test` to accept");
    macrotest::expand("tests/expand/*.rs");
}

#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
