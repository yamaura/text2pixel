use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::Rgb888, prelude::*, text::Text};

pub struct Canvas {
    size: Size,
    data: Vec<u8>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            size: (width, height).into(),
            data: vec![0; (width * height * 3) as usize],
        }
    }
}

impl embedded_graphics::draw_target::DrawTarget for Canvas {
    type Error = core::convert::Infallible;
    type Color = Rgb888;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels {
            let width = self.size.width as usize;

            let x = coord.x as usize;
            let y = coord.y as usize;
            let offset = (y * width + x) * 3;
            self.data[offset] = color.r();
            self.data[offset + 1] = color.g();
            self.data[offset + 2] = color.b();
        }
        Ok(())
    }
}

impl embedded_graphics::geometry::OriginDimensions for Canvas {
    fn size(&self) -> Size {
        self.size
    }
}

pub fn text2png(text: &str) -> Vec<u8> {
    

    let (width, height) = text
        .lines()
        .fold((0, 0), |(w, h), line| (w.max(line.len()), h + 1));

    let top = 13;
    let width = 6 * width as u32;
    let height = top + 9 * height as u32;
    let mut display = Canvas::new(width, height);

    let font = embedded_graphics::mono_font::ascii::FONT_6X9;

    Text::new(
        &text,
        Point::new(0, top as i32),
        MonoTextStyle::new(&font, Rgb888::WHITE),
    )
    .draw(&mut display)
    .unwrap();

    let mut result = vec![];
    let mut encoder = png::Encoder::new(std::io::Cursor::new(&mut result), width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&display.data).unwrap();
    writer.finish().unwrap();
    result
}

/*
fn main() {
    use std::io::Read;

    let mut text = Vec::new();
    std::io::stdin().read_to_end(&mut text).unwrap();

    let text = String::from_utf8(text).unwrap();
    let (width, height) = text
        .lines()
        .fold((0, 0), |(w, h), line| (w.max(line.len()), h + 1));

    let top = 13;
    let width = 6 * width as u32;
    let height = top + 9 * height as u32;
    let mut display = Canvas::new(width, height);

    let font = embedded_graphics::mono_font::ascii::FONT_6X9;

    // write "Good luck" into the framebuffer.
    Text::new(
        &text,
        Point::new(0, top as i32),
        MonoTextStyle::new(&font, Rgb888::WHITE),
    )
    .draw(&mut display)
    .unwrap();

    let mut encoder = png::Encoder::new(std::io::stdout().lock(), width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&display.data).unwrap();
}
*/
