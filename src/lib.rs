use std::{
    ffi::CString,
    fs::File,
    io::{BufWriter, Write},
};

use ravif::RGBA8;

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[no_mangle]
pub extern "C" fn png_to_avif(input: *const libc::c_uchar, input_len: u32) {
    let path_str = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(input, input_len as usize))
    };

    // write path to file
    let mut file = File::create("./testt.txt").unwrap();

    // write to file

    let decoder = png::Decoder::new(File::open(path_str).unwrap());

    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();
    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];
    // Inspect more details of the last read frame.
    let in_animation = reader.info().frame_control.is_some();

    let width = info.width;
    let height = info.height;

    let infos = format!("{} {} {}", width, height, in_animation);

    file.write_all(infos.as_bytes()).unwrap();

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

    let mut file = File::create("./test.avif").unwrap();

    if let Err(e) = encoder {
        println!("Error: {}", e);
        return;
    }

    let ouput_buf = encoder.unwrap().avif_file;

    file.write_all(&ouput_buf).unwrap();
}
