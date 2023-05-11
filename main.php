<?php
$png_image_path = __DIR__ . '\tests\stubs\test.png';

$start = microtime(true);

png_to_avif($png_image_path);

echo 'Time: ' . (microtime(true) - $start) . 's' . PHP_EOL;

function png_to_avif($path, $quality = 70.0, $speed = 4) {
    // $ffi_definition = 'void image_to_avif(char *input, int input_len, char *output, int output_len, float quality, uint8_t speed);';
    $ffi_definition = 'void png_to_avif(char *input, int input_len, char *output, int output_len);';

    $ffi_path_extension = match (PHP_OS_FAMILY) {
        'Darwin' => 'dylib',
        'Windows' => 'dll',
        default => 'so',
    }; 

    $ffi_path = realpath(__DIR__) . '/target/debug/php_ffi.' . $ffi_path_extension;

    $ffi = FFI::cdef($ffi_definition, $ffi_path);

    $output_path = str_replace('.png', '.avif', $path);

    // $ffi->image_to_avif($path, strlen($path), $output_path, strlen($output_path), $quality, $speed);
    $ffi->png_to_avif($path, strlen($path), $output_path, strlen($output_path));
}
?>


