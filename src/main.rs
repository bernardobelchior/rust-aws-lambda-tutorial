extern crate aws_lambda as lambda;
extern crate image;

use image::{DynamicImage, ImageError, ImageOutputFormat};
use image::load_from_memory;

fn encode_png_image(image: DynamicImage) -> Result<Vec<u8>, ImageError> {
    let mut result: Vec<u8> = Vec::new();

    image.write_to(&mut result, ImageOutputFormat::PNG)?;

    Ok(result)
}

fn main() {
    lambda::gateway::start(|req| {
        let base64_image: &str = req.body().as_str()?;

        let image = base64::decode(base64_image)?;
        let image = load_from_memory(&image)?;

        let thumbnail = image.thumbnail(128, 128);

        let encoded_thumbnail = encode_png_image(thumbnail)?;
        let encoded_thumbnail = base64::encode(&encoded_thumbnail);

        let res = lambda::gateway::response()
            .status(200)
            .body(lambda::gateway::Body::from(encoded_thumbnail))?;

        Ok(res)
    })
}
