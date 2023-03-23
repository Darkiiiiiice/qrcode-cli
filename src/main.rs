use clap::{Parser, Subcommand};
use qrcode::{render::unicode, QrCode};
use rqrr;

// define a struct to store args
// contain Commands enum
#[derive(Debug, Parser)]
#[command(name = "qrcode")]
#[command(version = "0.1.0")]
#[command(author = "MarioMang <darkiiiiiice@gmail>")]
#[command(about = "qrcode encode and decode")]
struct Args {
    #[clap(subcommand)]
    commands: Commands,
}

// define Commands enum for Args struct
// encode(EncodeCommand), encode command
// decode(DecodeCommand), decode command
#[derive(Subcommand, Debug)]
enum Commands {
    #[command(name = "encode")]
    Encode(EncodeCommand),
    #[command(name = "decode")]
    Decode(DecodeCommand),
}

// define EncodeCommand struct for Commands enum
// content: String, content to encode
#[derive(Parser, Debug)]
struct EncodeCommand {
    #[clap(required = true)]
    content: String,
    #[clap(short, long)]
    output: Option<String>,

    #[clap(long)]
    low: bool,

    #[clap(long)]
    medium: bool,

    #[clap(long)]
    high: bool,
}

// define DecodeCommand struct for Commands enum
// image_path: String, path of image
#[derive(Parser, Debug)]
struct DecodeCommand {
    #[clap(required = true)]
    image_path: String,
}

// define De

// implement a function for Args struct to new a Args struct
impl Args {
    fn new() -> Self {
        Self::parse()
    }
}

// function read args from stdin flag is
// -d for decode qrcode and
// -e for encode qrcode
// last arg is the path of image use clap,
// return Args struct
fn read_args() -> Args {
    Args::new()
}

// function encode qrcode from content
// return a Result type with Unicode String
fn encode_qrcode_stdout(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    // encode content to qrcode
    let code = QrCode::new(content.as_bytes())?;

    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

    Ok(image)
}

// function encode qrcode from content output to file
// return a Result type with Vec<u8>
fn encode_qrcode_image(
    content: &str,
    ecl: qrcode_png::QrCodeEcc,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut qrcode = qrcode_png::QrCode::new(content, ecl)?;

    qrcode.margin(12);
    qrcode.zoom(16);

    let buf = qrcode.generate(qrcode_png::Color::Rgb([0u8, 0u8, 0u8], [255, 255, 255]))?;
    Ok(buf)
}

// function decode qrcode from image path use rqrr
// return a Result type with String
fn decode_qrcode(image_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let image = image::open(image_path)?;
    let image = image.to_luma8();
    let mut prepare = rqrr::PreparedImage::prepare(image);

    let grids = prepare.detect_grids();

    if let Some(first) = grids.first() {
        let (_meta, content) = first.decode()?;
        return Ok(content);
    }
    return Err("Error: Cannot find qrcode".into());
}

fn main() {
    let args = read_args();

    // match args commands
    match &args.commands {
        Commands::Encode(encode_command) => {
            // match output flag
            // if output flag is not None, write qrcode to file
            // else print qrcode to stdout
            match &encode_command.output {
                Some(output) => {
                    let (low, medium, high) = (
                        encode_command.low,
                        encode_command.medium,
                        encode_command.high,
                    );

                    let ecl = match (low, medium, high) {
                        (true, false, false) => qrcode_png::QrCodeEcc::Low,
                        (false, true, false) => qrcode_png::QrCodeEcc::Medium,
                        (false, false, true) => qrcode_png::QrCodeEcc::High,
                        _ => qrcode_png::QrCodeEcc::High,
                    };

                    let image = encode_qrcode_image(&encode_command.content, ecl).unwrap();
                    // write qrcode to svg file
                    std::fs::write(output, image).unwrap();
                }
                None => {
                    let image = encode_qrcode_stdout(&encode_command.content).unwrap();
                    println!("{}", image);
                }
            }
        }
        Commands::Decode(decode_command) => {
            let content = decode_qrcode(&decode_command.image_path).unwrap();
            println!("{}", content);
        }
    }
}
