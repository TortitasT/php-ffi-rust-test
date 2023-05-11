# php-ffi-rust-test

Quickly made Rust png to avif image conversion callable from php. This is super dirty ^^'

## Requirements

- In amd64 you need [NASM](https://nasm.us/). In arm you may not need it or it
  may not even work.

- You need FFI module active in you php.ini.

## Usage

Run main.php

```bash
php main.php
```

It will convert test.png to test.avif.
