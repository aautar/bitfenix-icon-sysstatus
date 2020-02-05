extern crate hidapi;

mod text_renderer;
mod image;

use std::io::{self};
use hidapi::HidApi;
use hidapi::HidDevice;
use text_renderer::render_string;
use text_renderer::load_font_png;
use image::load_png_image;
use image::reduce_image_to_16bit_color;

fn print_device_info(bitfenix_icon_device: &HidDevice) {
    let device_manuf_string = bitfenix_icon_device.get_manufacturer_string().unwrap().unwrap();    
    let device_prod_string = bitfenix_icon_device.get_product_string().unwrap().unwrap();
    println!("{}: {}", device_manuf_string.as_str(), device_prod_string.as_str());    
}

fn clear_display(bitfenix_icon_device: &HidDevice) {
    let erase_flash_code: [u8; 6] = [0x0, 0x1, 0xde, 0xad, 0xbe, 0xef];
    bitfenix_icon_device.write(&erase_flash_code).unwrap();
}

fn refresh_display(bitfenix_icon_device: &HidDevice) {
    let refresh_code: [u8; 2] = [0x0, 0x3];
    bitfenix_icon_device.write(&refresh_code).unwrap();        
}

fn write_image_to_display(bitfenix_icon_device: &HidDevice, image_buf: &[u8]) {
    let num_image_bytes_per_write = 61; /* +3 bytes for the header, note that the device only accepts writes in 64 bytes chunks */
    let num_writes = ((image_buf.len() as f64 / num_image_bytes_per_write as f64).ceil()) as usize;

    for i in 0..num_writes {
        let start = i * num_image_bytes_per_write;
        let mut length = num_image_bytes_per_write;
        if i == (num_writes-1) {
            length = image_buf.len() - ((num_writes - 1) * num_image_bytes_per_write);
        }

        let mut image_data_with_header: Vec<u8> = Vec::with_capacity(length + 3);

        image_data_with_header.push(0x0);
        image_data_with_header.push(0x2);
        image_data_with_header.push(length as u8);        

        for image_byte_idx in start..start+length {
            image_data_with_header.push(image_buf[image_byte_idx]);
        }

        bitfenix_icon_device.write(&image_data_with_header).unwrap();
    }
}

fn main() -> io::Result<()> {

    println!("Loading assets...");

    // Image needs to be 240x320 (24bpp, no alpha channel)
    let src_image = load_png_image("assets/1.png");
    let mut reduced_color_img = reduce_image_to_16bit_color(&src_image);

    let font_image = reduce_image_to_16bit_color(&load_font_png("assets/fonts/font1.png"));

    println!("Opening device...");
    let hid = HidApi::new().unwrap();
    let bitfenix_icon_device = hid.open(0x1fc9, 0x100b).unwrap();

    //let toggle_backlight_code: [u8; 2] = [0x0, 0x4];
    //bitfenix_icon_device.write(&toggle_backlight_code).unwrap();    
    print_device_info(&bitfenix_icon_device);

    render_string("Avishkar was here", 10, 20, &font_image, &mut reduced_color_img);

    println!("Writing new image...");
    clear_display(&bitfenix_icon_device); // needs to be done or you end up with weird overwriting on top of exiting image
    write_image_to_display(&bitfenix_icon_device, &reduced_color_img);

    println!("Refreshing display...");
    refresh_display(&bitfenix_icon_device);

    Ok(())
}
