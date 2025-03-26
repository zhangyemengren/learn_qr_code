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
        Self::with_error_correction_level(data, EcLevel::H)
    }

    pub fn with_error_correction_level<D: AsRef<[u8]>>(
        data: D,
        ec_level: EcLevel,
    ) -> QrResult<Self> {
        let bits = bits::encode_auto(data.as_ref(), ec_level)?;
        println!("bits: {:?}", bits);
        Self::with_bits(bits, ec_level)
    }

    /**
     * 根据已编码的位序列创建QR码
     * 
     * 该函数接收已编码的位序列和错误纠正级别，执行以下步骤：
     * 1. 获取QR码版本和将位序列转换为字节数据
     * 2. 构造数据码字和纠错码字
     * 3. 创建QR码画布并绘制所有功能图案
     * 4. 将数据和纠错信息绘制到画布上
     * 5. 应用最佳掩码模式以优化QR码
     * 6. 返回完整的QR码对象
     * 
     * 这是QR码生成的核心函数，处理从位序列到最终QR码图像的转换过程
     */
    pub fn with_bits(bits: bits::Bits, ec_level: EcLevel) -> QrResult<Self> {
        let version = bits.version();
        let data = bits.into_bytes();
        let (encoded_data, ec_data) = ec::construct_codewords(&data, version, ec_level)?;
        println!("encoded_data: {:?}", encoded_data);
        println!("ec_data: {:?}", ec_data);
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