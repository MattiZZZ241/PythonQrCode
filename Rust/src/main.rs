use std::io;
use qrcode_generator::QrCodeEcc;

fn main() {
    println!("HWelcome to the Rust QR Code Generator !\n");

    println!("Please enter the url you want to encode : ");

    let mut url = String::new();

    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");

    qrcode_generator::to_png_to_file(url, QrCodeEcc::Low, 1024, "qr_rust.png").unwrap();
}
