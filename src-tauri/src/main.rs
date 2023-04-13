#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use magick_rust::{MagickWand, magick_wand_genesis, MagickError};
use serde_json::Value;
use std::{sync::Once, path::Path};
use include_dir::{include_dir, Dir};

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/flavors");
static START: Once = Once::new();

fn gen_new_name(file_path: String, palette: String) -> String {
  let path = Path::new(&file_path);
  let file_stem = path.file_stem().unwrap().to_str().unwrap();
  let extension = path.extension().unwrap().to_str().unwrap();

  format!("{palette}-{file_stem}.{extension}")
}

// Convert image using hald clut
fn convert(file_path: String, noise: String, palette: String) -> Result<(), MagickError> {
  let image_wand = MagickWand::new();
  image_wand.read_image(&file_path)?;
  
  let lut_wand = MagickWand::new();
  let lut_bytes = PROJECT_DIR.get_file(format!("noise-{noise}/{palette}.png")).unwrap().contents();
  lut_wand.read_image_blob(lut_bytes)?;

  image_wand.hald_clut_image(&lut_wand)?;

  let file_name = gen_new_name(file_path, palette);
  image_wand.write_image(&file_name)
}

fn main() {
  // Initialize magick wand
  START.call_once(|| {
    magick_wand_genesis();
  });

  tauri::Builder::default()
    .setup(|app| {
      let handle = app.handle();
      match app.get_cli_matches() {
        Ok(mut matches) => {
          let file_path = matches.args.remove("source").unwrap();
          let noise = matches.args.remove("noise").unwrap();
          let palette = matches.args.remove("palette").unwrap();

          if file_path.occurrences != 0 || noise.occurrences != 0 || palette.occurrences != 0 {
            let file_path = match file_path.value {
              Value::String(v) => v,
              _ => panic!("File path provided is not a string")
            };

            let noise = match noise.value {
              Value::String(v) => v,
              _ => panic!("Noise provided is not a string")
            };

            let palette = match palette.value {
              Value::String(v) => v,
              _ => panic!("Palette provided is not a string")
            };

            match convert(file_path, noise, palette).err() {
              Some(err) => panic!("{err}"),
              None => {}
            };

            handle.exit(0);
          }
          // No args provided, continue with frontend launch
        }
        Err(err) => {
          panic!("{err}");
        }
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}
