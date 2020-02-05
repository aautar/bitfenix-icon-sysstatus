use image::load_png_image;

pub fn load_font_png(filepath: &str) -> Vec<u8> {
    load_png_image(filepath)
}

pub fn build_font_width_vec() -> Vec<u8> {
    let result = vec![
        10, 10, 10, 10, 10, 10, 10, 10, 10, 40, 0, 10, 10, 0, 10, 10,
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 
        4, 4, 5, 8, 7, 11, 7, 3, 5, 5, 7, 8, 4, 5, 4, 5, 
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 5, 5, 8, 8, 8, 6, 
        10, 7, 7, 8, 8, 6, 6, 8, 8, 5, 5, 7, 6, 9, 7, 9, 
        7, 9, 8, 7, 7, 8, 7, 11, 7, 7, 7, 5, 5, 5, 8, 6, 
        6, 7, 7, 5, 7, 7, 4, 7, 7, 3, 3, 7, 3, 11, 7, 7, 
        7, 7, 4, 6, 4, 7, 6, 7, 7, 6, 6, 6, 5, 6, 8, 10, 
        6, 10, 3, 7, 5, 8, 7, 7, 6, 15, 7, 5, 11, 10, 7, 10, 
        10, 3, 3, 5, 5, 5, 7, 10, 6, 10, 6, 5, 10, 10, 6, 7, 
        4, 4, 7, 7, 7, 7, 5, 6, 6, 10, 5, 6, 8, 5, 10, 6, 
        5, 8, 5, 5, 6, 7, 7, 4, 6, 5, 5, 6, 10, 10, 10, 6, 
        7, 7, 7, 7, 7, 7, 10, 8, 6, 6, 6, 6, 5, 5, 5, 5, 
        8, 7, 9, 9, 9, 9, 9, 8, 9, 8, 8, 8, 8, 7, 7, 6, 
        7, 7, 7, 7, 7, 7, 10, 5, 7, 7, 7, 7, 3, 3, 3, 3, 
        6, 7, 7, 7, 7, 7, 7, 8, 7, 7, 7, 7, 7, 6, 7, 6             
    ];

    result
}

pub fn render_string(txt: &str, x: u64, y: u64, fontimg: &[u8], outbuf: &mut [u8]) {
    let font_widths = build_font_width_vec();
    let mut cur_x = x;
    for ch in txt.chars() {
        let ch_idx = ch as u64;
        let ch_width = font_widths[(ch as u32 % 256)  as usize];
        let ch_x = (ch_idx % 16) * 16;
        let ch_y = ((ch_idx as f32 / 16.0) as u64) * 16;

        for fy in ch_y..ch_y+16 {
            for fx in ch_x..ch_x+16 {
                let fidx: usize = ((fx + fy*256) * 2) as usize;
                let fdx = fx - ch_x;
                let fdy = fy - ch_y;
                let outbuf_idx: usize = (((cur_x + fdx) + (y + fdy)*240) * 2) as usize;

                if fontimg[fidx] != 0x00 && fontimg[fidx+1] != 0x00 {
                    outbuf[outbuf_idx] = fontimg[fidx];
                    outbuf[outbuf_idx] = fontimg[fidx + 1];
                }
            }
        }

        cur_x = cur_x + (ch_width as u64);
    }
}
