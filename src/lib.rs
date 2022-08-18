pub mod mod_json;
pub mod qmod;

#[cfg(test)]
mod tests {
    use super::*;
    use stopwatch::Stopwatch;
    #[test]
    fn write_qmod() -> Result<(), Box<dyn std::error::Error>> {
        let mut stopwatch = Stopwatch::start_new();
        println!("Trying to write qmod");
        let qmod = qmod::Qmod {
            mod_json: mod_json::ModJson {
                schema_version: semver::Version::new(1, 0, 0),
                name: "test".to_string(),
                id: "test".to_string(),
                ..Default::default()
            },
        };
        println!(
            "Creating qmod in memory took {}ms",
            stopwatch.elapsed().as_millis()
        );
        stopwatch.reset();

        let path = std::path::PathBuf::from("results/test.qmod");
        std::fs::create_dir_all(&path.parent().unwrap()).unwrap();
        qmod.package(path)?;
        println!("Writing to file took {}ms", stopwatch.elapsed().as_millis());

        Ok(())
    }
}
