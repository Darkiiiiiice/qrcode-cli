# qrcode-cli

[简体中文](../README.md) | [English](README_en.md)

qrcode-cli is a command-line tool for generating QR codes in the terminal. It allows users to quickly generate QR codes in the terminal, which can be used in documents, web pages, advertisements, and other places.

### Installation

qrcode-cli can be easily installed using cargo. Use the following command:

``` bash
cargo install qrcode-cli
```

### Usage

To generate a QR code, run the following command in the terminal:

``` bash
qrcode-cli encode <CONTENT>
```

For example, to generate a QR code and output "Hello, world!" to the terminal, run the following command:

``` bash
qrcode-cli encode "Hello, world!"
```

If you want to save the QR code to a file, use the following command:

``` bash
qrcode-cli encode "Hello, world!" -o ~/images
```

To decode a QR code image, use the following command:

``` bash
qrcode-cli decode ./image/a.png
```

### 选项

qrcode-cli supports the following options:

* encode Generate QR code
  * -o Output PNG image
  * --low Low quality output
  * --medium Medium quality output
  * --high High quality output
* decode Decode QR code image
* help Show help
* -V Version

``` bash
$ qrcode-cli help
qrcode encode and decode

Usage: qrcode-cli <COMMAND>

Commands:
  encode
  decode
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

help encode

``` bash
$ qrcode-cli help encode
Usage: qrcode-cli encode [OPTIONS] <CONTENT>

Arguments:
  <CONTENT>

Options:
  -o, --output <OUTPUT>
      --low
      --medium
      --high
  -h, --help             Print help
```

help decode

``` bash
$ qrcode-cli help decode
Usage: qrcode-cli decode <IMAGE_PATH>

Arguments:
  <IMAGE_PATH>

Options:
  -h, --help  Print help
```
