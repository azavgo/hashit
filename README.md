## Library to calculate SHA256 hash of a string or a file 

### This library uses [sha256 = "1.6.0"](https://crates.io/crates/sha256)

### Features: 
1. SHA256 hash of a string: `InputString::new(input: String) -> Self`; `fn hash_sha256(&self) -> Result<String, HashitError>`  
1. SHA256 hash of a file: `InputFile::new(input: &'a Path) -> Self`; `fn hash_sha256(&self) -> Result<String, HashitError>`  

### How to use this library: 
1. Add to Cargo.toml: 
```Toml
    [dependencies]
    hashit = {git = "https://github.com/azavgo/hashit", branch = "main"}
```
2. Generate SHA256 hash of a string:  
```Rust
    use hashit::*;

    fn main() -> Result<(), HashitError>{
        let input = "hello".to_owned();
        let input_string = InputString::new(input);
        let hash = input_string.hash_sha256()?;
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
        Ok(())
    }

```
3. Generate SHA256 hash of a file input_file_test.txt: 
```Rust
    use hashit::*;
    
    fn main() -> Result<(), HashitError> {
        let input = Path::new("./input_file_test.txt");
        let input_file = InputFile::new(input);
        let hash = input_file.hash_sha256()?;
        assert_eq!(
            hash,
            "6d2dd541398faa91d8e29d0940e0ad602dcabc9df6b5b6f71247fb326bcd311d"
        );
        Ok(())
    }
```
