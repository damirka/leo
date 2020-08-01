//! The `program.out` file.

use crate::errors::OutputsFileError;

use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

pub static OUTPUTS_DIRECTORY_NAME: &str = "outputs/";
pub static OUTPUTS_FILE_EXTENSION: &str = ".out";

pub struct OutputsFile {
    pub package_name: String,
}

impl OutputsFile {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
        }
    }

    pub fn exists_at(&self, path: &PathBuf) -> bool {
        let path = self.setup_file_path(path);
        path.exists()
    }

    /// Reads the outputs from the given file path if it exists.
    pub fn read_from(&self, path: &PathBuf) -> Result<String, OutputsFileError> {
        let path = self.setup_file_path(path);

        let outputs = fs::read_to_string(&path).map_err(|_| OutputsFileError::FileReadError(path.clone()))?;
        Ok(outputs)
    }

    /// Writes output to a file.
    pub fn write(&self, path: &PathBuf, bytes: &[u8]) -> Result<(), OutputsFileError> {
        // create output file
        let path = self.setup_file_path(path);
        let mut file = File::create(&path)?;
        log::info!("Writing to output registers...");

        Ok(file.write_all(bytes)?)
    }

    fn setup_file_path(&self, path: &PathBuf) -> PathBuf {
        let mut path = path.to_owned();
        if path.is_dir() {
            if !path.ends_with(OUTPUTS_DIRECTORY_NAME) {
                path.push(PathBuf::from(OUTPUTS_DIRECTORY_NAME));
            }
            path.push(PathBuf::from(format!(
                "{}{}",
                self.package_name, OUTPUTS_FILE_EXTENSION
            )));
        }
        path
    }
}