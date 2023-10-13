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

    // Render string along y-axis (rotated 90deg clockwise)
    pub fn render_string_rot90cw(&self, txt: &str, x: u64, y: u64, fontimg: &[u8], outbuf: &mut [u8]) {
        // x position of where character should be rendered
        let mut cur_y = y;
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

                    // write down, as we read across
                    let ox: u64 = (x + 15) - fdy;
                    let oy: u64 = cur_y + fdx; 

                    let outbuf_idx: usize = ((ox + (oy)*240) * 2) as usize;
                    
                    // If the pixel from the font bitmap is not black, write it out to outbuf
                    if fontimg[fidx] != 0x00 && fontimg[fidx+1] != 0x00 {
                        outbuf[outbuf_idx] = fontimg[fidx];
                        outbuf[outbuf_idx + 1] = fontimg[fidx + 1];
                    }
                }
            }
    
            cur_y = cur_y - (ch_width as u64);
        }
    }

    // Render string along y-axis (rotated 90deg counter-clockwise)
    pub fn render_string_rot90ccw(&self, txt: &str, x: u64, y: u64, fontimg: &[u8], outbuf: &mut [u8]) {
        // x position of where character should be rendered
        let mut cur_y = y;
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

                    // write down to up, left to right, as we read across
                    let ox: u64 = x + fdy;
                    let oy: u64 = (cur_y + 15) - fdx;

                    let outbuf_idx: usize = ((ox + (oy)*240) * 2) as usize;

                    // If the pixel from the font bitmap is not black, write it out to outbuf
                    if fontimg[fidx] != 0x00 && fontimg[fidx+1] != 0x00 {
                        outbuf[outbuf_idx] = fontimg[fidx];
                        outbuf[outbuf_idx + 1] = fontimg[fidx + 1];
                    }
                }
            }

            cur_y = cur_y + (ch_width as u64);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_test_font() -> [u8; 131072] {
        let mut px_arr: [u8; 131072] = [0; 131072];

        // top-left pixel for "A"
        let idx_tl: usize = (16 + 64*256) * 2;
        px_arr[idx_tl] = 255;
        px_arr[idx_tl+1] = 255;

        // top-right pixel for "A"
        let idx_tr: usize = (31 + 64*256) * 2;
        px_arr[idx_tr] = 255;
        px_arr[idx_tr+1] = 255;        

        // .. close-to-center pixel for A
        let idx_center: usize = (20 + 73*256) * 2;
        px_arr[idx_center] = 255;
        px_arr[idx_center+1] = 255;

        px_arr
    }

    #[test]
    fn render_string_renders_font_character_correctly() {
        let font = gen_test_font();
        
        let mut background: Vec<u8> = Vec::with_capacity(240 * 320 * 2);
        for _ in 0..((240 * 320 * 2)) {
            background.push(0);
        }

        let tr = TextRenderer::new();
        let render_x = 10;
        let render_y = 5;
        tr.render_string("A", render_x, render_y, &font, &mut background);

        // check top-left pixel of character in output
        let idx_char_tl = ((render_x + render_y*240) * 2) as usize;
        assert_eq!(background[idx_char_tl], 255);
        assert_eq!(background[idx_char_tl+1], 255);

        // check top-right pixel of character in output
        let idx_char_tr = (((render_x+15) + (render_y+0)*240) * 2) as usize;
        assert_eq!(background[idx_char_tr], 255);
        assert_eq!(background[idx_char_tr+1], 255);

        // check close-to-center pixel of character in output
        let idx_char_center = (((render_x+4) + (render_y+9)*240) * 2) as usize;
        assert_eq!(background[idx_char_center], 255);
        assert_eq!(background[idx_char_center+1], 255);        

    }

    #[test]
    fn render_string_rot90cw_renders_font_character_correctly() {
        let font = gen_test_font();
        
        let mut background: Vec<u8> = Vec::with_capacity(240 * 320 * 2);
        for _ in 0..((240 * 320 * 2)) {
            background.push(0);
        }

        let tr = TextRenderer::new();
        let render_x = 10;
        let render_y = 5;
        tr.render_string_rot90cw("A", render_x, render_y, &font, &mut background);

        // check top-right pixel of character in output (this is the top-left pixel in non-rotated output)
        let idx_char_tl = (((render_x + 15) + render_y *240) * 2) as usize;
        assert_eq!(background[idx_char_tl], 255);
        assert_eq!(background[idx_char_tl+1], 255);

        // check bottom-right pixel of character in output (this is the top-right pixel in non-rotated output)
        let idx_char_tr = (((render_x+15) + (render_y+15)*240) * 2) as usize;
        assert_eq!(background[idx_char_tr], 255);
        assert_eq!(background[idx_char_tr+1], 255);

        // check close-to-center pixel of character in output
        // (offset from left becomes offset from top, offset from top becomes offset from right)
        let idx_char_center = (((render_x+(15-9)) + (render_y+4)*240) * 2) as usize;
        assert_eq!(background[idx_char_center], 255);
        assert_eq!(background[idx_char_center+1], 255);
    }

    #[test]
    fn render_string_rot90ccw_renders_font_character_correctly() {
        let font = gen_test_font();
        
        let mut background: Vec<u8> = Vec::with_capacity(240 * 320 * 2);
        for _ in 0..((240 * 320 * 2)) {
            background.push(0);
        }

        let tr = TextRenderer::new();
        let render_x = 10;
        let render_y = 50;
        tr.render_string_rot90ccw("A", render_x, render_y, &font, &mut background);

        // check bottom-left pixel of character in output (this is the top-left pixel in non-rotated output)
        let idx_char_tl = ((render_x + (render_y+15) *240) * 2) as usize;
        assert_eq!(background[idx_char_tl], 255);
        assert_eq!(background[idx_char_tl+1], 255);

        // check top-left pixel of character in output (this is the top-right pixel in non-rotated output)
        let idx_char_tr = ((render_x + render_y*240) * 2) as usize;
        assert_eq!(background[idx_char_tr], 255);
        assert_eq!(background[idx_char_tr+1], 255);


        // check close-to-center pixel of character in output
        // (offset from left becomes offset from top, offset from top becomes offset from right)
        let idx_char_center = (((render_x+9) + ((render_y+15)-4)*240) * 2) as usize;
        assert_eq!(background[idx_char_center], 255);
        assert_eq!(background[idx_char_center+1], 255);
    }

}
