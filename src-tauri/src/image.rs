use image::{RgbaImage, GenericImageView};
use include_dir::{include_dir, Dir};
use magick_rust::{MagickError, MagickWand};
use serde::{Deserialize, Serialize};
use std::{path::Path, io::Cursor};

static NOISE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/noise");

#[derive(Serialize, Deserialize)]
pub struct OutputImage {
    source: InputImage,
    blob: Vec<u8>,
}

impl OutputImage {
    fn from(source: InputImage, blob: Vec<u8>) -> Self {
        OutputImage { source, blob }
    }

    pub fn save(self) {
        let wand = MagickWand::new();
        let output_name = format!(
            "{}-{}-n{}.{}",
            self.source.stem, self.source.palette, self.source.noise, self.source.ext
        );

        wand.read_image_blob(self.blob);
        wand.write_image(&output_name);
    }
}

#[derive(Serialize, Deserialize)]
struct InputImage {
    path: String,
    stem: String,
    ext: String,
    theme: String,
    palette: String,
    noise: String,
}

impl InputImage {
    fn from(path_s: &str, theme: &str, palette: &str, noise: &str) -> Self {
        let path = Path::new(path_s);
        let stem = path.file_stem().unwrap().to_str().unwrap().to_string();
        let ext = path.extension().unwrap().to_str().unwrap().to_string();

        InputImage {
            path: path_s.to_owned(),
            stem,
            ext,
            theme: theme.to_owned(),
            palette: palette.to_owned(),
            noise: noise.to_owned(),
        }
    }
}

// MagickError does not derive Serialize, so we make our own custom Error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Magick(#[from] MagickError),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

// Return a wand of `image` that has been converted
// TODO: handle errors properly
#[tauri::command(async)]
pub fn convert(path: &str, theme: &str, palette: &str, noise: &str) -> Result<OutputImage, Error> {
    let image = InputImage::from(path, theme, palette, noise);

    let input = image::open(&image.path).unwrap();
    let clut_bytes =  NOISE_DIR
        .get_file(format!(
            "{}/{}/noise_{}.png",
            image.theme, image.palette, image.noise
        ))
        .unwrap()
        .contents();
    let clut = image::load_from_memory(clut_bytes).unwrap();

    let mut output = RgbaImage::new(input.width(), input.height());

    for (x, y, pixel) in input.pixels() {
        let r = pixel[0] as u32 / 4;
        let g = pixel[1] as u32 / 4;
        let b = pixel[2] as u32 / 4;

        let c_x = r % 64 + (g % 8) * 64;
        let c_y = b * 8 + g / 8;

        let pixel_out = clut.get_pixel(c_x, c_y);
        output.put_pixel(x, y, pixel_out)
    }

    let mut buf = Cursor::new(Vec::new());
    output.write_to(&mut buf, image::ImageOutputFormat::Png);

    Ok(OutputImage::from(image, buf.into_inner()))
}

#[tauri::command(rename_all = "snake_case")]
pub fn save_output(image: OutputImage) {
    image.save();
}
