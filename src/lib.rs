use ravif::RGBA8;
use std::{fs::File, io::Write};

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[no_mangle]
pub extern "C" fn png_to_avif(input: *const libc::c_uchar, input_len: u32) {
    let path_str = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(input, input_len as usize))
    };

    let decoder = png::Decoder::new(File::open(path_str).unwrap());

    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];

    let width = info.width;
    let height = info.height;

    let pixels: &[RGBA8] = unsafe {
        std::slice::from_raw_parts(
            bytes.as_ptr() as *const RGBA8,
            width as usize * height as usize,
        )
    };
    let width_usize = width as usize;
    let height_usize = height as usize;

    let encoder = ravif::Encoder::new()
        .with_quality(70.)
        .with_speed(4)
        .encode_rgba(ravif::Img::new(pixels, width_usize, height_usize));

    let output_path = path_str.replace(".png", ".avif");

    let mut file = File::create(output_path).unwrap();
    if let Err(e) = encoder {
        println!("Error: {}", e); // This wont print anything i think
        return;
    }

    let ouput_buf = encoder.unwrap().avif_file;

    file.write_all(&ouput_buf).unwrap();
}
