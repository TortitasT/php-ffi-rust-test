# php-ffi-rust-test

Quickly made Rust png to avif image conversion callable from php.
This is super dirty ^^'

## Requirements

- In amd64 you need [NASM](https://nasm.us/). In arm I've not been able to get
  it working.

- You need FFI module active in you php.ini.

## Usage

Build the rust library then run main.php

```bash
cargo build
php main.php
```

It will convert test.png to test.avif.
