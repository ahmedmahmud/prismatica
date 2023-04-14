#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use magick_rust::magick_wand_genesis;
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
                Ok(matches) => {
                    let args = ["source", "theme", "palette", "noise"]
                        .into_iter()
                        .map(|arg| {
                            (
                                arg,
                                matches
                                    .args
                                    .get(arg)
                                    .expect(&format!("{} arg does not exist", arg)),
                            )
                        })
                        .collect::<Vec<_>>();

                    if args.iter().any(|(_, arg)| arg.occurrences > 0) {
                        let values = args
                            .iter()
                            .map(|(name, arg)| {
                                arg.value
                                    .as_str()
                                    .expect(&format!("{} arg is not a string", name))
                            })
                            .collect::<Vec<_>>();

                        let output = convert(values[0], values[1], values[2], values[3]);
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
