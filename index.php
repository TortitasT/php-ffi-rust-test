<?php

// $ffi = FFI::cdef('int add(int a, int b);', __DIR__ . '\target\debug\php_ffi.dll');

$ffi_definition = 'void png_to_avif(char *input, int input_len);';
$ffi = FFI::cdef($ffi_definition, __DIR__ . '\target\debug\php_ffi.dll');

$string = __DIR__ . '\test.png';

$ffi->png_to_avif($string, strlen($string));

?>
