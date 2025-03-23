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
    let string = code
        .render::<char>()
        .dark_color('#')
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();

    println!("{}", string);
}
