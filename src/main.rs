use {
    clap::{arg, Command},
};

mod helpers;
mod math;

fn cli() -> Command<'static> {
    Command::new("rust_one_time_pad")
        .about("One Time Pad Implementation in Rust.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            Command::new("encrypt")
                .about("Encrypts a file")
                .arg(arg!(<RANDOM_BIT> "Random bit r"))
                .arg(arg!(<INPUT_FILE> "File to encrypt"))
                .arg(arg!(<OUTPUT_FILE> "Encrypted file"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypts a file")
                .arg(arg!(<RANDOM_BIT> "Random bit r"))
                .arg(arg!(<INPUT_FILE> "File to decrypt"))
                .arg(arg!(<OUTPUT_FILE> "Decrypted file"))
                .arg_required_else_help(true)
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("encrypt", sub_matches)) => {
            let _ = helpers::encrypt_file(
                sub_matches.value_of("RANDOM_BIT").expect("required").parse::<u8>().expect("Invalid random bit r"),
                sub_matches.value_of("INPUT_FILE").expect("required"),
                sub_matches.value_of("OUTPUT_FILE").expect("required")
            );
        }
        Some(("decrypt", sub_matches)) => {
            let _ = helpers::decrypt_file(
                sub_matches.value_of("RANDOM_BIT").expect("required").parse::<u8>().expect("Invalid random bit r"),
                sub_matches.value_of("INPUT_FILE").expect("required"),
                sub_matches.value_of("OUTPUT_FILE").expect("required")
            );
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .values_of_os("")
                .unwrap_or_default()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
        }
        _ => unreachable!(),
    }
}
