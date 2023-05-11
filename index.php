<?php
$string = __DIR__ . '\test.png';
png_to_avif($string);

function png_to_avif($input) {
    $ffi_definition = 'void png_to_avif(char *input, int input_len);';
    $ffi_path = __DIR__ . '\target\debug\php_ffi.dll';

    $ffi = FFI::cdef($ffi_definition, $ffi_path);

    $ffi->png_to_avif($input, strlen($input));
}
?>
