use libc::c_uchar;
use ravif::RGBA8;
use std::{fs::File, io::Write};

#[no_mangle]
pub extern "C" fn image_to_avif(
    input: *const c_uchar,
    input_len: u32,
    output: *const c_uchar,
    output_len: u32,
    quality: f32,
    speed: u8,
) {
    let input_path = input_to_string(input, input_len);

    let output_path = input_to_string(output, output_len);

    let image_format = input_path.split('.').last().unwrap();

    let (pixels, width, height) = match image_format {
        "png" => get_image_png(input_path),
        "jpeg" | "jpg" => get_image_jpeg(input_path),
        _ => panic!("Unsupported image format"),
    };

    let encoder = ravif::Encoder::new()
        .with_quality(quality)
        .with_speed(speed)
        .encode_rgba(ravif::Img::new(&pixels, width, height));

    let mut file = File::create(output_path).unwrap();
    if let Err(e) = encoder {
        panic!("Error: {}", e)
    }

    let ouput_buf = encoder.unwrap().avif_file;

    file.write_all(&ouput_buf).unwrap();
}

fn input_to_string(input: *const c_uchar, input_len: u32) -> &'static str {
    return unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(input, input_len as usize))
    };
}

fn get_image_png(path: &str) -> (Vec<RGBA8>, usize, usize) {
    let decoder = png::Decoder::new(File::open(path).unwrap());

    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];

    let width = info.width;
    let height = info.height;

    let pixels: Vec<RGBA8> = bytes
        .chunks_exact(4)
        .map(|chunk| RGBA8 {
            r: chunk[0],
            g: chunk[1],
            b: chunk[2],
            a: chunk[3],
        })
        .collect();
    let width_usize = width as usize;
    let height_usize = height as usize;

    return (pixels, width_usize, height_usize);
}

fn get_image_jpeg(path: &str) -> (Vec<RGBA8>, usize, usize) {
    let decoder = jpeg_decoder::Decoder::new(File::open(path).unwrap());

    panic!("Unsupported image format");
}
