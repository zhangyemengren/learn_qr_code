use crate::{bits, canvas, ec};
use crate::cast::As;
use crate::render::{Pixel, Renderer};
use crate::types::{Color, EcLevel, QrResult, Version};

#[derive(Clone)]
pub struct QrCode {
    content: Vec<Color>,
    version: Version,
    ec_level: EcLevel,
    width: usize,
}

impl QrCode{
    pub fn new<D: AsRef<[u8]>>(data: D) -> QrResult<Self> {
        Self::with_error_correction_level(data, EcLevel::L)
    }

    pub fn with_error_correction_level<D: AsRef<[u8]>>(
        data: D,
        ec_level: EcLevel,
    ) -> QrResult<Self> {
        let bits = bits::encode_auto(data.as_ref(), ec_level)?;
        println!("bits: {:?}", bits);
        Self::with_bits(bits, ec_level)
    }

    pub fn with_bits(bits: bits::Bits, ec_level: EcLevel) -> QrResult<Self> {
        let version = bits.version();
        let data = bits.into_bytes();
        let (encoded_data, ec_data) = ec::construct_codewords(&data, version, ec_level)?;
        let mut canvas = canvas::Canvas::new(version, ec_level);
        canvas.draw_all_functional_patterns();
        canvas.draw_data(&encoded_data, &ec_data);
        let canvas = canvas.apply_best_mask();
        Ok(Self {
            content: canvas.into_colors(),
            version,
            ec_level,
            width: version.width().as_usize(),
        })
    }
    pub fn render<P: Pixel>(&self) -> Renderer<P> {
        let quiet_zone = if self.version.is_micro() { 2 } else { 4 };
        Renderer::new(&self.content, self.width, quiet_zone)
    }
}