#[test]
fn test() {
    println!("`MACROTEST=overwrite cargo test` to accept");
    macrotest::expand("tests/expand/*.rs");
}
