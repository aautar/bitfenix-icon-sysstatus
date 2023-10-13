extern crate hidapi;

mod text_renderer;
mod image;
mod web;

use std::io::{self};
use std::{thread, time::Duration, time::SystemTime};
use hidapi::HidApi;
use hidapi::HidDevice;
use text_renderer::TextRenderer;
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

fn get_hostname() -> String {
    match hostname::get() {
        Ok(hn) => hn.into_string().unwrap(),
        Err(_) => String::from("Hostname unknown")
    }
}

fn main() -> io::Result<()> {
    loop {
        // Refresh time
        let refresh_start_time = chrono::Local::now();
        let refresh_start_time_str = refresh_start_time.to_rfc2822();
        
        let refresh_start_time_display = ["Last refresh: ", &refresh_start_time_str].concat();

        // Hostname
        let hostname = get_hostname();
        let hostname_print_out = ["Host: ", &hostname].concat();

        // Web connection status
        let web_connect_status = web::http_get();
        let mut web_connect_display = "FAIL";
        if web_connect_status == "200" {
            web_connect_display = "OK";
        }

        let web_connect_status = ["Internet: ", web_connect_display].concat();

        // CPU temp

        println!("Loading assets...");

        // Background image needs to be 240x320 (24bpp, no alpha channel)
        let mut background_image = reduce_image_to_16bit_color(&load_png_image("assets/2.png"));

        let font_image = reduce_image_to_16bit_color(&load_png_image("assets/fonts/font1.png"));

        println!("Opening device...");
        let hid = HidApi::new().unwrap();
        let bitfenix_icon_device = hid.open(0x1fc9, 0x100b).unwrap();

        //let toggle_backlight_code: [u8; 2] = [0x0, 0x4];
        //bitfenix_icon_device.write(&toggle_backlight_code).unwrap();
        print_device_info(&bitfenix_icon_device);

        let tr = TextRenderer::new();
        tr.render_string_rot90ccw(refresh_start_time_display.as_str(), 10, 300, &font_image, &mut background_image);
        tr.render_string_rot90ccw(hostname_print_out.as_str(), 25, 300, &font_image, &mut background_image);
        tr.render_string_rot90ccw(web_connect_status.as_str(), 40, 300, &font_image, &mut background_image);

        println!("Writing new image...");
        clear_display(&bitfenix_icon_device); // needs to be done or you end up with weird overwriting on top of exiting image
        write_image_to_display(&bitfenix_icon_device, &background_image);

        println!("Refreshing display...");
        refresh_display(&bitfenix_icon_device);

        thread::sleep(Duration::from_millis(300000));
    }

    Ok(())
}
