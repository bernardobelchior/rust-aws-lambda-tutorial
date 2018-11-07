extern crate aws_lambda as lambda;
extern crate failure;
extern crate image;

use failure::{err_msg, Error};
use image::{ColorType, DecodingResult, DynamicImage, ImageBuffer, ImageDecoder, ImageError, ImageOutputFormat, png::PNGDecoder};
use std::collections::HashMap;

fn decode_png_image(image: &[u8]) -> Result<DynamicImage, Error> {
    let mut decoder = PNGDecoder::new(image);
    let color_type = decoder.colortype()?;
    let (width, height) = decoder.dimensions()?;

    let image = match decoder.read_image()? {
        DecodingResult::U8(image) => {
            match color_type {
                ColorType::BGRA(8) => DynamicImage::ImageBgra8(ImageBuffer::from_vec(width, height, image).unwrap()),
                ColorType::BGR(8) => DynamicImage::ImageBgr8(ImageBuffer::from_vec(width, height, image).unwrap()),
                ColorType::RGBA(8) => DynamicImage::ImageRgba8(ImageBuffer::from_vec(width, height, image).unwrap()),
                ColorType::RGB(8) => DynamicImage::ImageRgb8(ImageBuffer::from_vec(width, height, image).unwrap()),
                ColorType::GrayA(8) => DynamicImage::ImageLumaA8(ImageBuffer::from_vec(width, height, image).unwrap()),
                ColorType::Gray(8) => DynamicImage::ImageLuma8(ImageBuffer::from_vec(width, height, image).unwrap()),
                _ => return Err(err_msg("color scheme not supported"))
            }
        }
        DecodingResult::U16(_image) => {
            return Err(err_msg("16-bit images not supported"));
        }
    };

    Ok(image)
}

fn encode_png_image(image: DynamicImage) -> Result<Vec<u8>, ImageError> {
    let mut result: Vec<u8> = Vec::new();

    image.write_to(&mut result, ImageOutputFormat::PNG)?;

    Ok(result)
}

fn main() {
    lambda::gateway::start(|req| {
        let input: HashMap<String, String> = req.body().decode_json()?;

        let base64_image = match input.get("image") {
            Some(image) => image,
            None => return Err(err_msg("Missing image"))
        };

        let image = base64::decode(base64_image)?;
        let image = decode_png_image(image.as_slice())?;

        let thumbnail = image.thumbnail(128, 128);

        let encoded_thumbnail = encode_png_image(thumbnail)?;
        let encoded_thumbnail = base64::encode(&encoded_thumbnail);

        let res = lambda::gateway::response()
            .status(200)
            .body(lambda::gateway::Body::from(encoded_thumbnail))?;

        Ok(res)
    })
}
