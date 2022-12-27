pub mod extracter {
    extern crate cipher_crypt;
    use cipher_crypt::{Caesar, Cipher, Porta, Rot13, Scytale, Vigenere};

    use image;
    use rqrr::PreparedImage;
    use std::fs::File;
    use std::io::Write;

    pub fn decode_qrcode(algorithm: &str, file_path: &String) -> String {
        let contents: Vec<String> = extract_qrcode_data(file_path);
        let extracted = contents[0].to_string();

        match algorithm {
            "txt" => extracted,
            "rot13" => Rot13::decrypt(&extracted),
            "morse" => crypto_morse::decode(&extracted),

            "base64" => {
                let decoded_data = base64::decode(extracted).unwrap();
                String::from_utf8_lossy(&decoded_data).to_string()
            }

            "hex" => {
                let decoded_data = hex::decode(extracted);
                String::from_utf8_lossy(&decoded_data.unwrap()).to_string()
            }

            _ => {
                panic!("Unknown encoding algorithm!");
            }
        }
    }

    pub fn decrypt_qrcode(algorithm: &str, file_path: &String, key: &String) -> String {
        let contents: Vec<String> = extract_qrcode_data(file_path);

        match algorithm {
            "caesar" => {
                let num = key.parse::<i64>().unwrap();
                let cipher = Caesar::new(num.try_into().unwrap());
                cipher.decrypt(&contents[0]).unwrap()
            }

            "porta" => {
                let cipher = Porta::new((&key).to_string());
                cipher.decrypt(&contents[0]).unwrap()
            }

            "vigenere" => {
                let cipher = Vigenere::new((&key).to_string());
                cipher.decrypt(&contents[0]).unwrap()
            }

            "scytale" => {
                let num = key.parse::<i64>().unwrap();
                let cipher = Scytale::new(num.try_into().unwrap());
                cipher.decrypt(&contents[0]).unwrap()
            }

            _ => {
                panic!("Unknown encrypting algorithm!");
            }
        }
    }

    fn extract_qrcode_data(file_path: &String) -> Vec<String> {
        let gray_image = image::open(file_path).unwrap().to_luma8();
        let mut prepared_image = PreparedImage::prepare(gray_image);
        prepared_image
            .detect_grids()
            .into_iter()
            .map(|grid| {
                let (_, content) = grid
                    .decode()
                    .unwrap_or_else(|err| panic!("Failed while reading data from QrCode: {err}"));
                content
            })
            .collect()
    }

    pub fn dump_results_to_file(output: &String, result: &String) {
        let mut output_file = File::create(output).unwrap();
        write!(output_file, "{}", &result).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use std::ops::Add;

    #[test]
    fn test_decode_qrcode() {
        for algorithm in ["base64", "hex", "rot13", "txt", "morse"] {
            let file_path = String::from("resources/")
                .add(algorithm)
                .add("/test_qrcode.png");

            let msg = extracter::decode_qrcode(&algorithm.to_string(), &file_path);
            assert!(msg.eq("there is test message to encode!"))
        }
    }

    #[test]
    fn test_decrypt_qrcode() {
        let algorithms = dict!(
            "caesar" => "24",
            "scytale" => "102",
            "porta" => "nskdjfbskddggegr",
            "vigenere" => "nskdjfbskddggegr"
        );

        for (algorithm, key) in algorithms {
            let file_path = String::from("resources/")
                .add(algorithm)
                .add("/test_qrcode.png");

            let msg =
                extracter::decrypt_qrcode(&algorithm.to_string(), &file_path, &key.to_string());

            assert!(msg.eq("there is test message to encode!"))
        }
    }
}
