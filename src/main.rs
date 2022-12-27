use clap::{arg, ArgMatches, Command};
use std::path::Path;

include!("dict.rs");

mod extract;
pub use crate::extract::extracter;

mod generate;
pub use crate::generate::generator;

fn generate_qrcode(matches: &ArgMatches) {
    let algorithm = matches
        .get_one::<String>("encode")
        .expect("The encode parameter is missing.\n");

    let message = matches
        .get_one::<String>("msg")
        .expect("The message parameter is missing.\n");

    let default_output = String::from("/tmp/qrcode.png");
    let output = match matches.get_one::<String>("output") {
        Some(path) => path,
        None => &default_output,
    };

    match matches.get_one::<String>("key") {
        Some(key) => generator::encrypt_data(algorithm, message, key, output),
        None => generator::encode_data(algorithm, message, output),
    }

    println!("QrCode has been generated successful!");
}

fn extract_qrcode(matches: &ArgMatches) {
    let algorithm = matches
        .get_one::<String>("encode")
        .expect("The encode parameter is missing.\n");

    let file_path = matches
        .get_one::<String>("file")
        .expect("The file parameter is missing.\n");

    if !Path::new(&file_path).exists() {
        panic!("Incorrect file name or file path :(");
    };

    let default_output = String::from("/tmp/qrcode.png");
    let output = match matches.get_one::<String>("output") {
        Some(path) => path,
        None => &default_output,
    };

    let result = match matches.get_one::<String>("key") {
        Some(key) => extracter::decrypt_qrcode(algorithm, file_path, key),
        None => extracter::decode_qrcode(algorithm, file_path),
    };

    extracter::dump_results_to_file(output, &result);
    println!("{}", &result);
}

fn main() {
    let matches = Command::new("QrCode-Encrypt")
        .version("0.1.1")
        .about("Just encode data to QrCode and extract it :)")
        .arg(arg!(<MODE> "The application mode: generate or extract").required(true))
        .arg(arg!(-e --encode <CIPHER> "The encoding algorithm.").required(true))
        .arg(arg!(-k --key <VALUE> "The key to apply cipher.").required(false))
        .arg(arg!(-f --file <PATH> "Path to file with QrCode").required(false))
        .arg(arg!(-o --output <PATH> "The output file path.").required(false))
        .arg(arg!(-m --msg <VALUE> "The message to encode.").required(false))
        .get_matches();

    let mode = matches
        .get_one::<String>("MODE")
        .expect("The mode parameter is required");

    match mode.as_str() {
        "generate" => generate_qrcode(&matches),
        "extract" => extract_qrcode(&matches),
        _ => {
            println!("There are no available modes and options. See usage: ");
        }
    }
}
