use sha256::digest;

fn hash_sha256(s: &str) -> String {
    let input = s.to_owned();
    digest(input)
}

fn main() {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test01_hash_sha256() {
        let s = "hello";
        let hash = hash_sha256(s);
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }
}
