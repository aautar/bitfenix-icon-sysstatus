fn build_font_width_vec() -> Vec<u8> {
    let result = vec![
        7, 7, 7, 7, 7, 7, 7, 7, 7, 30, 0, 7, 7, 0, 7, 7,
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 
        5, 3, 5, 9, 7, 18, 8, 3, 5, 5, 7, 9, 4, 8, 3, 5, 
        8, 4, 7, 7, 8, 7, 7, 7, 7, 7, 3, 4, 6, 9, 6, 7, 
        9, 8, 8, 8, 8, 8, 8, 8, 8, 5, 7, 8, 7, 9, 9, 9, 
        8, 9, 8, 8, 9, 8, 9, 10, 9, 9, 8, 4, 6, 4, 8, 9, 
        5, 7, 7, 6, 7, 7, 6, 7, 7, 3, 5, 7, 3, 9, 7, 7, 
        7, 7, 6, 6, 6, 7, 7, 10, 7, 7, 6, 5, 3, 5, 8, 7, 
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 
        7, 3, 3, 5, 5, 3, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 
        7, 3, 6, 7, 7, 13, 3, 11, 10, 13, 7, 7, 7, 7, 7, 7, 
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 
    ];

    result
}

pub struct TextRenderer {
    font_widths: Vec<u8>,
}

impl TextRenderer {
    pub fn new() -> TextRenderer {
        TextRenderer {
            font_widths: build_font_width_vec()
        }
    }

    pub fn render_string(&self, txt: &str, x: u64, y: u64, fontimg: &[u8], outbuf: &mut [u8]) {
        // x position of where character should be rendered
        let mut cur_x = x;
        for ch in txt.chars() {
            // From codepoint, lookup x, y of character in font and width of character from self.font_widths
            let ch_idx = ch as u64;
            let ch_width = self.font_widths[(ch as u32 % 256)  as usize];
            let ch_x = (ch_idx % 16) * 16;
            let ch_y = ((ch_idx as f32 / 16.0) as u64) * 16;
    
            // For each character, copy the 16x16 block of pixels into outbuf
            for fy in ch_y..ch_y+16 {
                for fx in ch_x..ch_x+16 {
                    let fidx: usize = ((fx + fy*256) * 2) as usize;
                    let fdx = fx - ch_x;
                    let fdy = fy - ch_y;
                    let outbuf_idx: usize = (((cur_x + fdx) + (y + fdy)*240) * 2) as usize;
                    
                    // If the pixel from the font bitmap is not black, write it out to outbuf
                    if fontimg[fidx] != 0x00 && fontimg[fidx+1] != 0x00 {
                        outbuf[outbuf_idx] = fontimg[fidx];
                        outbuf[outbuf_idx + 1] = fontimg[fidx + 1];
                    }
                }
            }
    
            cur_x = cur_x + (ch_width as u64);
        }
    }    
}
