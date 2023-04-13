use magick_rust::{MagickWand, MagickError};
use std::path::Path;
use include_dir::{include_dir, Dir};

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/flavors");

fn gen_new_name(file_path: String, palette: String) -> String {
  let path = Path::new(&file_path);
  let file_stem = path.file_stem().unwrap().to_str().unwrap();
  let extension = path.extension().unwrap().to_str().unwrap();

  format!("{palette}-{file_stem}.{extension}")
}

// Convert image using hald clut
fn convert(file_path: String, noise: String, palette: String) -> Result<MagickWand, MagickError> {
  let image_wand = MagickWand::new();
  image_wand.read_image(&file_path)?;
  
  let lut_wand = MagickWand::new();
  dbg!(format!("noise-{noise}/{palette}.png"));
  let lut_bytes = PROJECT_DIR.get_file(format!("noise-{noise}/{palette}.png")).unwrap().contents();
  lut_wand.read_image_blob(lut_bytes)?;

  image_wand.hald_clut_image(&lut_wand)?;

  Ok(image_wand)
}

pub fn convert_and_save(file_path: String, noise: String, palette: String) -> Result<(), MagickError> {
  let wand = convert(file_path.clone(), noise, palette.clone())?;

  let file_name = gen_new_name(file_path, palette);
  wand.write_image(&file_name)
}

#[tauri::command(async rename_all = "snake_case")]
pub fn convert_and_blob(file_path: String, noise: String, palette: String) -> Vec<u8> {
  dbg!(&file_path);

  match convert(file_path, noise, palette) {
    Ok(wand) => wand.write_image_blob("png").unwrap(),
    Err(_) => panic!("Failed to convert image"),
  }
}

#[tauri::command(rename_all = "snake_case")]
pub fn save_from_blob(file_path: String, palette:String, blob: Vec<u8>) {
  let wand = MagickWand::new();
  wand.read_image_blob(blob);

  let file_name = gen_new_name(file_path, palette);
  wand.write_image(&file_name);
}
