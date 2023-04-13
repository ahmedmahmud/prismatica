#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use magick_rust::{magick_wand_genesis};
use std::sync::Once;
use serde_json::Value;

use crate::image::{convert_and_blob, save_from_blob, convert_and_save};

mod image;

static START: Once = Once::new();

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

            match convert_and_save(file_path, noise, palette).err() {
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
    .invoke_handler(tauri::generate_handler![convert_and_blob, save_from_blob])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}
