pub mod generator {
    extern crate base64;
    extern crate cipher_crypt;
    use cipher_crypt::{Caesar, Cipher, Porta, Rot13, Scytale, Vigenere};

    use qrcode_png::{Color, QrCode, QrCodeEcc};

    pub fn encode_data(algorithm: &str, message: &String, output: &str) {
        match algorithm {
            "txt" => generate_qrcode(message, output),
            "hex" => generate_qrcode(&hex::encode(message), output),
            "rot13" => generate_qrcode(&Rot13::encrypt(message), output),
            "morse" => generate_qrcode(&crypto_morse::encode(message), output),

            "base64" => {
                let msg_bytes = String::from(message).into_bytes();
                generate_qrcode(&base64::encode(msg_bytes), output);
            }

            _ => {
                panic!("Unknown encoding algorithm!");
            }
        };
    }

    pub fn encrypt_data(algorithm: &str, message: &str, key: &String, output: &str) {
        let encrypted_data = match algorithm {
            "vigenere" => {
                let cipher = Vigenere::new((&key).to_string());
                cipher.encrypt(message).unwrap()
            }

            "porta" => {
                let cipher = Porta::new((&key).to_string());
                cipher.encrypt(message).unwrap()
            }

            "caesar" => {
                let num = key.parse::<i64>().unwrap();
                let cipher = Caesar::new(num.try_into().unwrap());
                cipher.encrypt(message).unwrap()
            }

            "scytale" => {
                let num = key.parse::<i64>().unwrap();
                let cipher = Scytale::new(num.try_into().unwrap());
                cipher.encrypt(message).unwrap()
            }

            _ => {
                panic!("Unknown encrypting algorithm!");
            }
        };

        generate_qrcode(&encrypted_data, output);
    }

    fn generate_qrcode(encoded_data: &str, output_file: &str) {
        let mut qrcode = QrCode::new(encoded_data, QrCodeEcc::Medium).unwrap();

        qrcode.margin(12).zoom(12);

        let color = Color::Grayscale(0, 255);
        let image_data = qrcode.generate(color).unwrap();
        std::fs::write(output_file, image_data).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use std::ops::Add;
    use std::path::PathBuf;

    #[test]
    fn test_encode_data() {
        let message = String::from("there is test message to encode!");
        for algorithm in ["base64", "hex", "morse", "rot13", "txt"] {
            let output = String::from("resources/")
                .add(algorithm)
                .add("/test_qrcode.png");

            generator::encode_data(&algorithm.to_string(), &message, &output);
            assert!(PathBuf::from(output).exists())
        }
    }

    #[test]
    fn test_encrypt_data() {
        let message = String::from("there is test message to encode!");
        let algorithms = dict!(
            "caesar" => "24",
            "scytale" => "102",
            "porta" => "nskdjfbskddggegr",
            "vigenere" => "nskdjfbskddggegr"
        );

        for (algorithm, key) in algorithms {
            let output = String::from("resources/")
                .add(algorithm)
                .add("/test_qrcode.png");

            generator::encrypt_data(&algorithm.to_string(), &message, &key.to_string(), &output);

            assert!(PathBuf::from(output).exists())
        }
    }
}
