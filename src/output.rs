use std::{
    fmt::Display,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use crate::file_meta::Format;

pub trait OutputFile {
    type File: Write + Read;
    type Error;
    fn open_write(&self, format: &Format) -> Result<Self::File, Self::Error>;
}

pub struct OriginSameOutput {
    origin: PathBuf,
    output_dir: Option<PathBuf>,
}

impl Display for OriginSameOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "origin: {:?}, output dir: {:?}",
            self.origin, self.output_dir
        )
    }
}

impl OriginSameOutput {
    pub fn new(origin: impl AsRef<Path>, output: Option<impl AsRef<Path>>) -> Self {
        Self {
            origin: origin.as_ref().to_path_buf(),
            output_dir: output.map(|path| path.as_ref().to_path_buf()),
        }
    }

    pub fn open(&self) -> io::Result<File> {
        OpenOptions::new().read(true).open(&self.origin)
    }
}

impl OutputFile for OriginSameOutput {
    type File = File;

    type Error = io::Error;

    fn open_write(&self, format: &Format) -> Result<Self::File, Self::Error> {
        let current_filename = self.origin.with_extension(format.to_extension());
        let filename = current_filename.file_name();
        let file_path = self
            .output_dir
            .as_ref()
            .and_then(|path| Some(path.join(filename?)))
            .unwrap_or(current_filename);

        Ok(fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(true)
            .open(file_path)?)
    }
}
