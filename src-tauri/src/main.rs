#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use magick_rust::{MagickWand, magick_wand_genesis, MagickError};
use std::sync::Once;

static START: Once = Once::new();

// Convert image using hald clut
fn convert() -> Result<(), MagickError> {
  let image_wand = MagickWand::new();
  let lut_wand = MagickWand::new();
  
  image_wand.read_image("test.jpg")?;
  lut_wand.read_image("flavors/noise-4/oled.png")?;

  image_wand.hald_clut_image(&lut_wand)?;

  image_wand.write_image("oled-test.jpg")
}

fn main() {
  // Initialize magick wand
  START.call_once(|| {
    magick_wand_genesis();
  });

  convert();

  // tauri::Builder::default()
  //   .run(tauri::generate_context!())
  //   .expect("error while running tauri application");

}
