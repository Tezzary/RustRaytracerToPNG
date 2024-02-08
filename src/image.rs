pub struct Image {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}

impl Image {
    pub fn blank(width: u32, height: u32) -> Image {
        let mut bytes = Vec::with_capacity((width * height * 4) as usize);
        for _ in 0..(width * height) {
            bytes.push(0);
            bytes.push(0);
            bytes.push(0);
            bytes.push(255);
        };
        Image {
            width: width,
            height: height,
            bytes: bytes,
        }
    }
    pub fn write_to_pixel(&mut self, x: u32, y:u32, rgba: [u8; 4]) {
        let index  = ((y * self.width) + x) as usize * 4;
        self.bytes[index] = rgba[0];
        self.bytes[index + 1] = rgba[1];
        self.bytes[index + 2] = rgba[2];
        self.bytes[index + 3] = rgba[3];
    }

    pub fn save_to_file(&mut self, filename: &str) {
        let mut file = std::fs::File::create(format!("images/{}.png", filename)).unwrap();
        let mut encoder = png::Encoder::new(&mut file, self.width, self.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&self.bytes).unwrap();
    }
    
}
pub fn create_unused_filename() -> String {
    let mut index = 0;
    loop {
        let filename = format!("{}", index);
        if !std::path::Path::new(&format!("images/{}.png", filename)).exists() {
            return filename;
        }
        index += 1;
    }
}