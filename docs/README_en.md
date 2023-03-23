# qrcli

[简体中文](../README.md) | [English](README_en.md)

qrcli is a command-line tool for generating QR codes in the terminal. It allows users to quickly generate QR codes in the terminal, which can be used in documents, web pages, advertisements, and other places.

### Installation

qrcli can be easily installed using cargo. Use the following command:

``` bash
cargo install qrcli
```

### Usage

To generate a QR code, run the following command in the terminal:

``` bash
qrcli encode <CONTENT>
```

For example, to generate a QR code and output "Hello, world!" to the terminal, run the following command:

``` bash
qrcli encode "Hello, world!"
```

If you want to save the QR code to a file, use the following command:

``` bash
qrcli encode "Hello, world!" -o ~/images
```

To decode a QR code image, use the following command:

``` bash
qrcli decode ./image/a.png
```

### 选项

qrcli supports the following options:

* encode Generate QR code
  * -o Output PNG image
  * --low Low quality output
  * --medium Medium quality output
  * --high High quality output
* decode Decode QR code image
* help Show help
* -V Version

``` bash
$ qrcli help
qrcode encode and decode

Usage: qrcli <COMMAND>

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
$ qrcli help encode
Usage: qrcli encode [OPTIONS] <CONTENT>

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
$ qrcli help decode
Usage: qrcli decode <IMAGE_PATH>

Arguments:
  <IMAGE_PATH>

Options:
  -h, --help  Print help
```
