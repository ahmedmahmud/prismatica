#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use magick_rust::magick_wand_genesis;
use serde_json::Value;
use std::sync::Once;

mod image;
use crate::image::{convert, save_output};

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

                    if file_path.occurrences != 0
                        || noise.occurrences != 0
                        || palette.occurrences != 0
                    {
                        let file_path = match file_path.value {
                            Value::String(v) => v,
                            _ => panic!("File path provided is not a string"),
                        };

                        let noise = match noise.value {
                            Value::String(v) => v,
                            _ => panic!("Noise provided is not a string"),
                        };

                        let palette = match palette.value {
                            Value::String(v) => v,
                            _ => panic!("Palette provided is not a string"),
                        };

                        let output = convert(file_path, "catppuccin".to_owned(), palette, noise);
                        output.map_or_else(|err| panic!("{err}"), |o| o.save());

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
        .invoke_handler(tauri::generate_handler![convert, save_output])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
