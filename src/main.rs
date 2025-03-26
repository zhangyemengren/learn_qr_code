use image::Luma;
use crate::qr_code::QrCode;

mod qr_code;
mod bits;
mod types;
mod optimize;
mod cast;
mod ec;
mod canvas;
mod render;

fn main() {
    let code = QrCode::new(b"http://www.baidu.com").unwrap();

    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("qrcode.png").unwrap();
}
