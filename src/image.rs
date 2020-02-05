use std::fs::File;

pub fn load_png_image(filepath: &str) -> Vec<u8> {
    let decoder = png::Decoder::new(File::open(filepath).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    
    // Allocate the output buffer.
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    buf
}

pub fn reduce_image_to_16bit_color(image_buf: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(240 * 320 * 2);

    for i in (0..image_buf.len()).step_by(3) {
		let b: u16 = ((image_buf[i + 2] as u16) >> 3) & 0x001F;
		let g: u16 = (((image_buf[i + 1] as u16) >> 2) <<  5) & 0x07E0;
		let r: u16 = (((image_buf[i] as u16) >> 3) << 11) & 0xF800;
        let rgb: u16 = r | g | b;
        
        result.push( ((rgb >> 8) & 0x00FF) as u8 );
        result.push( (rgb & 0x00FF) as u8 );
    }

    result
}
