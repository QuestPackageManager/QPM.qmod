use crate::mod_json::ModJson;

use std::{io::Write, path::PathBuf};
use thiserror::Error;
use zip::{write::FileOptions, ZipWriter};

pub struct Qmod {
    pub mod_json: ModJson,
}

#[derive(Error, Debug)]
pub enum QmodError {
    #[error("Failed to create file")]
    FileCreate(#[from] std::io::Error),
    #[error("Failed to Serialize ModJson")]
    Serialize(#[from] serde_json::Error),
    #[error("Failed to Start ZipFile")]
    ZipFile(#[from] zip::result::ZipError),
    #[error("Unknown qmod Error")]
    Unknown,
}

impl Qmod {
    pub fn package(self, path: PathBuf) -> Result<(), QmodError> {
        let file = std::fs::File::create(path)?;
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        let mut z_writer = ZipWriter::new(file);
        z_writer.start_file("mod.json", options)?;
        z_writer.write_all(self.mod_json.serialize()?.as_bytes())?;

        Ok(())
    }
}
