#[inline(always)]
pub fn decrypt(bytes : &[u8], r : &bool) -> Vec<u8> {
    let mut decrypted_bytes = Vec::new();
    for bit in bytes {
        decrypted_bytes.push(bit ^ *r as u8);
    }
    return decrypted_bytes;
}

#[inline(always)]
pub fn encrypt(bytes : &[u8], r : &bool) -> Vec<u8> {
    let mut encrypted_bytes : Vec<u8> = Vec::new();
    for bit in bytes {
        encrypted_bytes.push(bit ^ *r as u8);
    }
    return encrypted_bytes;
}
