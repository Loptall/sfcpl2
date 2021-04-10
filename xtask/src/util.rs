use std::fs::{File, OpenOptions};

use anyhow::{Context, Result};

pub fn find_cargo_toml(path: &str) -> Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(format!("{}/Cargo.toml", path))
        .with_context(|| "failed to read Cargo.toml")
}

mod tests {
    use std::fs::File;

    use super::find_cargo_toml;
    use anyhow::Result;

    #[test]
    fn read_cargo_toml() {
        let content = find_cargo_toml(".").unwrap();
        dbg!(content);
    }
}
