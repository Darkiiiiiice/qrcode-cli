# qrcode-cli

[简体中文](README.md) | [English](docs/README_en.md)

qrcode-cli 是一个命令行工具，用于在终端上生成 QR 码。它可以在终端上快速生成 QR 码，方便用户将其用于文档、网页、广告等地方。

### 安装

使用 cargo 可以轻松地安装 qrcode-cli，使用以下命令即可：

``` bash
cargo install qrcode-cli
```

### 使用方法

要生成QR码，请在终端中运行以下命令：

``` bash
qrcode-cli encode <CONTENT>
```

例如，要生成QR码并将“Hello, world!”输出到终端，请运行以下命令：

``` bash
qrcode-cli encode "Hello, world!"
```

如果您想将QR码保存到文件中，可以使用以下命令：

``` bash
qrcode-cli encode "Hello, world!" -o ~/images
```

如果要对QR码图片进行解码，可以使用以下命令:

``` bash
qrcode-cli decode ./image/a.png
```

### 选项

qrcode-cli支持以下选项：

* encode 生成QRCODE
  * -o 输出png图像
  * --low 低质量输出
  * --medium 中质量输出
  * --high 高质量输出
* decode 解码QRCODE图片
* help 显示帮助
* -V 显示版本

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
