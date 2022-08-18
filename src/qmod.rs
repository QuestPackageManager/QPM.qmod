use crate::mod_json::ModJson;

use std::{io::Write, path::PathBuf};
use zip::{write::FileOptions, ZipWriter};

pub struct Qmod {
    pub mod_json: ModJson,
}

impl Qmod {
    // TODO: error types?
    pub fn package(self, path: PathBuf) -> Result<(), ()> {
        let file = std::fs::File::create(path);
        if file.is_err() {
            println!("Could not create file");
            return Err(());
        }

        let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        let mut z_writer = ZipWriter::new(file.unwrap());
        if let Ok(serialized) = self.mod_json.serialize() {
            z_writer.start_file("mod.json", options).unwrap();
            z_writer.write_all(serialized.as_bytes()).unwrap();
        }

        Ok(())
    }
}
