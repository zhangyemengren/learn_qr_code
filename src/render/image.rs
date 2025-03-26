use crate::render::{Canvas, Pixel};
use crate::types::Color;

use image::{ImageBuffer, Luma, LumaA, Primitive, Rgb, Rgba};

// need to keep using this macro to implement Pixel separately for each color model,
// otherwise we'll have conflicting impl with `impl Pixel for impl Element` 🤷
macro_rules! impl_pixel_for_image_pixel {
    ($p:ident<$s:ident>: $c:pat => $d:expr) => {
        impl<$s> Pixel for $p<$s>
        where
            $s: Primitive + 'static,
            $p<$s>: image::Pixel<Subpixel = $s>,
        {
            type Image = ImageBuffer<Self, Vec<$s>>;
            type Canvas = (Self, Self::Image);

            fn default_color(color: Color) -> Self {
                match color.select($s::zero(), $s::max_value()) {
                    $c => $p($d),
                }
            }
        }
    };
}

impl_pixel_for_image_pixel! { Luma<S>: p => [p] }
impl_pixel_for_image_pixel! { LumaA<S>: p => [p, S::max_value()] }
impl_pixel_for_image_pixel! { Rgb<S>: p => [p, p, p] }
impl_pixel_for_image_pixel! { Rgba<S>: p => [p, p, p, S::max_value()] }

impl<P: image::Pixel + 'static> Canvas for (P, ImageBuffer<P, Vec<P::Subpixel>>) {
    type Pixel = P;
    type Image = ImageBuffer<P, Vec<P::Subpixel>>;

    fn new(width: u32, height: u32, dark_pixel: P, light_pixel: P) -> Self {
        (
            dark_pixel,
            ImageBuffer::from_pixel(width, height, light_pixel),
        )
    }

    fn draw_dark_pixel(&mut self, x: u32, y: u32) {
        self.1.put_pixel(x, y, self.0);
    }

    fn into_image(self) -> ImageBuffer<P, Vec<P::Subpixel>> {
        self.1
    }
}
