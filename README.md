# hashit
Library of hash functions
## This library allows to create a hash of a string slice
2. The default algorithm will be SHA256. The explicit flag will also be added as sha256
3. Alternatively one can use SHA512 with sha512 flag
4. Later on SHA254 may be added too, the flag will be sha254
1. MD5 hash function algorithm (not recommended, however possible with the md5 flag): https://www.comparitech.com/blog/information-security/md5-algorithm-with-examples/

How to use this program: 
hashit "My Password"
The output will be a string representing the SHA256 hash of the string "My Password"

hashit -sha512 "My Password"
The output will be a string representing the SHA512 hash of the string "My Password" 

hashit -c "My Password" "hash string" 
The output will be either "Pass" or "Fail". The utility will then compare (flag -c) the string representing 
a password, check the supplied hash string to determin which hashing algorithm to be used. Then it will 
calculate the hash string of the "My Password" string and compare the result with the supplied hash string.

