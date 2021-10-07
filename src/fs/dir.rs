use crate::fs::file::File;
use std::convert::TryFrom;
use std::fs;
use std::io;

#[derive(Debug)]
pub struct Dir {
    pub name: String,
    pub contents: Vec<File>,
}

impl TryFrom<File> for Dir {
    type Error = io::Error;

    fn try_from(file: File) -> Result<Self, Self::Error> {
        let name = file.name.clone();

        let mut contents = fs::read_dir(file.path)?
            .map(|res| res.map(|e| e.path()))
            .map(|res| res.map(|p| File::try_from(p).unwrap()))
            .collect::<Result<Vec<File>, io::Error>>()?;

        contents.sort();

        Ok(Dir { name, contents })
    }
}
