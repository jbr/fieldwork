use std::{
    env,
    fs::{self, File},
    path::Path,
};

use super::derive_fieldwork_internal;

#[test]
fn code_coverage() {
    let tests_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests");
    let paths = vec![tests_dir.join("expand"), tests_dir.join("ui")];

    for path in paths {
        for file in fs::read_dir(path).unwrap() {
            let direntry = file.unwrap();
            let path = direntry.path();
            if path.extension().is_some_and(|x| x == "rs")
                && !path.to_string_lossy().contains(".expanded")
            {
                let file = File::open(&path).unwrap();
                runtime_macros::emulate_derive_macro_expansion(
                    file,
                    &[
                        ("fieldwork::Fieldwork", derive_fieldwork_internal),
                        ("Fieldwork", derive_fieldwork_internal),
                    ],
                )
                .unwrap();
            }
        }
    }
}
