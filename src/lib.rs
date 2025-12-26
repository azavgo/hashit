use sha256::{digest, try_digest};
use std::path::Path; 

#[derive(Debug)]
pub enum HashitError {
    Error(std::io::Error),
}

impl From<std::io::Error> for HashitError {
    fn from(error: std::io::Error) -> Self {
        HashitError::Error(error)
    }
}

pub trait Hashit {
    fn hash_sha256(&self) -> Result<String, HashitError>;
}

pub struct InputFile <'a> {
    pub input: &'a Path,
}

impl <'a> InputFile <'a> {
    pub fn new(input: &'a Path) -> Self {
        Self { input: input }
    }
}

impl <'a> Hashit for InputFile <'a> {
    fn hash_sha256(&self) -> Result<String, HashitError> {
        Ok(try_digest(&self.input)?)
           
    }
}

pub struct InputString{
    pub input: String,
}

impl InputString {
    pub fn new(input: String) -> Self {
        Self { input: input }
    }
}

impl Hashit for InputString {
    fn hash_sha256(&self) -> Result<String, HashitError> {
        Ok(digest(&self.input))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test01_inputstring_hash_sha256() -> Result<(), HashitError> {
        let input = "hello".to_owned();
        let input_string = InputString::new(input);
        let hash = input_string.hash_sha256()?;
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
        Ok(())
    }

    #[test]
    fn test01_inputfile_hash_sha256() -> Result<(), HashitError> {
        let input = Path::new("./input_file_test.txt");
        let input_file = InputFile::new(input);
        let hash = input_file.hash_sha256()?;
        assert_eq!(
            hash,
            "6d2dd541398faa91d8e29d0940e0ad602dcabc9df6b5b6f71247fb326bcd311d"
        );
        Ok(())
    }
}
