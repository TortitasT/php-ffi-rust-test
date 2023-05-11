<?php
$png_image_path = __DIR__ . '/test.png';

$start = microtime(true);

png_to_avif($png_image_path);

echo 'Time: ' . (microtime(true) - $start) . 's' . PHP_EOL;

function png_to_avif($input) {
    $ffi_definition = 'void png_to_avif(char *input, int input_len);';

    $ffi_path_extension = match (PHP_OS_FAMILY) {
        'Darwin' => 'dylib',
        'Windows' => 'dll',
        default => 'so',
    }; 

    $ffi_path = realpath(__DIR__) . '/target/debug/php_ffi.' . $ffi_path_extension;

    $ffi = FFI::cdef($ffi_definition, $ffi_path);

    $ffi->png_to_avif($input, strlen($input));
}
?>


