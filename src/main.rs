fn hash_md5(s: &str) -> &str{
    unimplemented!();
}
//Will work correctly on the 64-bit OS, not sure about 32-bit OS
//Padding format for the last block of 64 bytes: 
//message, then 1 (binary: 10000000), then zeros, and then final 8 bytes 
//are reserved for the length of the message itself. 
//Example: input: "a", padding output "a10...00000001"
//Padding format in this example: "a" - message = 1 byte, "1" = 1 byte, then 54x"0" = 54 bytes, then "00000001" - 8 bytes, 
//all together 64 bytes
fn padding(input: &str) -> String {
    let input_size = input.len(); //size of the input in bytes

    let input_size_str = input_size.to_string();//size in bytes of the input_size
    let input_size_str_size = input_size_str.len();

    if input_size < 56 {
        let v: Vec<u8> = vec![0; 64 - 1 - input_size - input_size_str_size];
        let v_str: String = v.into_iter().map(|s| s.to_string()).collect();
        format!("{}{}{}{}", input, "1", v_str, input_size_str)
    } else if input_size == 56 {
        let v: Vec<u8> = vec![0; 64 + 64 - 1 - input_size - input_size_str_size];
        let v_str: String = v.into_iter().map(|s| s.to_string()).collect();
        format!("{}{}{}{}", input, "1", v_str, input_size_str)
    } else {
        unimplemented!();
    }
}

fn main() {
    let input = "When the initial input is greater than 448 bits long, it";
    let output = padding(input);
    println!("input = {}, output = {}", input, output);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test01_padding() {
        assert!(padding("abcde") == "abcde10000000000000000000000000000000000000000000000000000000005".to_string());
    }
    #[test]
    fn test02_padding() {
        assert!(padding("They are deterministic") == "They are deterministic100000000000000000000000000000000000000022".to_string());
    }
    #[test]
    fn test01_hash_md5() {
        assert!(hash_md5("") == "d41d8cd98f00b204e9800998ecf8427e");
    }
    #[test]
    fn test02_hash_md5() {
        assert!(hash_md5("a") == "0cc175b9c0f1b6a831c399e269772661");
    }
    #[test]
    fn test03_hash_md5() {
        assert!(hash_md5("abc") == "900150983cd24fb0d6963f7d28e17f72");
    }
    #[test]
    fn test04_hash_md5() {
        assert!(hash_md5("message digest") == "f96b697d7cb7938d525a2f31aaf161d0");
    }
    #[test]
    fn test05_hash_md5() {
        assert!(hash_md5("abcdefghijklmnopqrstuvwxyz") == "c3fcd3d76192e4007dfb496cca67e13b");
    }
    #[test]
    fn test06_hash_md5() {
        assert!(hash_md5("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789") == "d174ab98d277d9f5a5611c2c9f419d9f");
    }
    #[test]
    fn test07_hash_md5() {
        assert!(hash_md5("12345678901234567890123456789012345678901234567890123456789012345678901234567890") == "57edf4a22be3c955ac49da2e2107b67a");
    }   
}


