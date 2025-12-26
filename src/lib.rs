use sha256::digest;

pub trait Hashit {
    fn hash_sha256(&self) -> String;
}

pub struct InputString{
    pub input: String,
}

impl InputString {
    pub fn new(input: String) -> Self {
        InputString { input: input }
    }
}

impl Hashit for InputString {
    fn hash_sha256(&self) -> String {
        digest(&self.input)    
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test01_hash_sha256() {
        let input = "hello".to_owned();
        let input_string = InputString::new(input);
        let hash = input_string.hash_sha256();
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }
}
