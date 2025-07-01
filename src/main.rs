mod set1_1; 

fn main() {
    // Set 1 Challenge 1: Hex to Base64
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    match set1_1::hex_to_base64(hex) {
        Ok(base64) => println!("Base64: {}", base64),
        Err(e) => eprintln!("Error: {}", e),
    }

    // Set 1 Challenge 2: Fixed XOR
    // Set 1 Challenge 3: Single-byte XOR cipher
    // Set 1 Challenge 4: Detect single-character XOR
    // Set 1 Challenge 5: Repeating-key XOR cipher
    // Set 1 Challenge 6: Break repeating-key XOR
    // Set 1 Challenge 7: AES in ECB mode
    // Set 1 Challenge 8: Detect AES in ECB mode

    // Set 2 Challenge 9: Implement PKCS#7 padding
    // Set 2 Challenge 10: Implement CBC mode
    // Set 2 Challenge 11: An ECB/CBC detection oracle
    // Set 2 Challenge 12: Byte-at-a-time ECB decryption
    // Set 2 Challenge 13: ECB cut-and-paste
    // Set 2 Challenge 14: Byte-at-a-time ECB decryption (Harder)
    // Set 2 Challenge 15: PKCS#7 padding validation
    // Set 2 Challenge 16: CBC bitflipping attacks

}