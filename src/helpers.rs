use {
    std::io::Read,
    std::io::Write,
    std::fs::File,
};

use crate::math;

pub fn decrypt_file(r_ : u8, input_filename : &str, output_filename: &str) -> Result<(), std::io::Error> {
    let r = if r_ == 0 { false } else { true };
    let mut file = File::open(input_filename)?;
    let mut encrypted_bytes : Vec<u8> = Vec::new();
    file.read_to_end(&mut encrypted_bytes)?;
    let mut file = File::create(output_filename)?;
    let bytes = math::decrypt(&encrypted_bytes, &r);
    file.write_all(&bytes)?;
    println!("Decrypted file written to {}", output_filename);
    Ok(())
}

pub fn encrypt_file(r_ : u8, input_filename : &str, output_filename: &str) -> Result<(), std::io::Error> {
    let r = if r_ == 0 { false } else { true };
    let mut file = File::open(input_filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut file = File::create(output_filename)?;
    let encrypted_bytes = math::encrypt(&contents.as_bytes(), &r);
    file.write_all(&encrypted_bytes)?;
    println!("Encrypted file written to {}", output_filename);
    Ok(())
}
