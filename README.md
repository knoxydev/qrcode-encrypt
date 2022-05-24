# qrcode-encrypt

### Encryption
- the key doesn't require (if the key isn't required, write `--key` in the `<key>` clause)
    - base64
    - hex
    - morse
    - rot13
    - text
- the key is required
    - caesar - key type: Number
    - scytale - key type: Number
    - vigenere - key type: String
    - porta - key type: String
    
---

### If you want to create a QRcode
- cargo run `create` `<encryption>` `<key>` `<text>`
### If you want to read a QRcode
- cargo run `scan` `<encryption>` `<key>` `<filename>`

---

### For Example
- Create/Read a QRcode with key
    - `cargo run create caesar 4 some-text`
    - `cargo run scan vigenere abc qrcode.png`
- Create/Read a QRcode without key
    - `cargo run create base64 --key some-text`
    - `cargo run scan morse --key qrcode.png`

---

### Attention
- if you enter the arguments correctly when starting the project to create a qr code, you will get a `qrcode.png`
- if you enter the arguments correctly when reading the qr code, you will get the `result-qrcode.txt` with the result inside the file