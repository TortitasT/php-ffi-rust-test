use libc::c_uchar;
use ravif::RGBA8;
use std::{fs::File, io::Write};

// #[no_mangle]
// pub extern "C" fn image_to_avif(
//     input: *const c_uchar,
//     input_len: u32,
//     output: *const c_uchar,
//     output_len: u32,
//     quality: f32,
//     speed: u8,
// ) {
//     let input_path = input_to_string(input, input_len);
//
//     let output_path = input_to_string(output, output_len);
//
//     let image_format = input_path.split('.').last().unwrap();

// let (pixels, width, height) = match image_format {
//     "png" => get_image_png(input_path),
//     "jpeg" | "jpg" => get_image_jpeg(input_path),
//     _ => panic!("Unsupported image format"),
// };

// panic!(
//     "input {} ouput {} qual {} speed {} width {} height {} format {} len {}",
//     input_path, output_path, quality, speed, width, height, image_format, input_len
// );

//     let encoder = ravif::Encoder::new()
//         .with_quality(quality)
//         .with_speed(speed)
//         .encode_rgba(ravif::Img::new(pixels, width, height));
//
//     let mut file = File::create(output_path).unwrap();
//     if let Err(e) = encoder {
//         panic!("Error: {}", e)
//     }
//
//     let ouput_buf = encoder.unwrap().avif_file;
//
//     file.write_all(&ouput_buf).unwrap();
// }

fn input_to_string(input: *const c_uchar, input_len: u32) -> &'static str {
    return unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(input, input_len as usize))
    };
}

fn get_image_png(path: &str) -> (&[RGBA8], usize, usize) {
    let decoder = png::Decoder::new(File::open(path).unwrap());

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

    return (&pixels, width_usize, height_usize);
}

fn get_image_jpeg(path: &str) -> (&[RGBA8], usize, usize) {
    let decoder = jpeg_decoder::Decoder::new(File::open(path).unwrap());

    panic!("Unsupported image format");
}

#[no_mangle]
pub extern "C" fn png_to_avif(input: *const libc::c_uchar, input_len: u32) {
    let path_str = input_to_string(input, input_len);

    let (pixels, width_usize, height_usize) = get_image_png(&path_str);

    for pixel in pixels {
        println!("{:?}", pixel);
    }

    let encoder = ravif::Encoder::new()
        .with_quality(70.)
        .with_speed(4)
        .encode_rgba(ravif::Img::new(&pixels, width_usize, height_usize));

    let output_path = path_str.replace(".png", ".avif");

    let mut file = File::create(output_path).unwrap();
    if let Err(e) = encoder {
        println!("Error: {}", e); // This wont print anything i think
        return;
    }

    let ouput_buf = encoder.unwrap().avif_file;

    file.write_all(&ouput_buf).unwrap();
}
