#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rodio::Decoder;
use rodio::OutputStream;
use rodio::Sink;
use std::fs::File;
use std::io::BufReader;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn slot_audio() {
    std::thread::Builder::new()
        .spawn(|| {
            play_audio("/home/alex/Desktop/projects/jihad/src/slot.mp3");
        })
        .unwrap();
}

fn play_audio(file_name: &str) {
    //create_file(file_name);

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let decoder = Decoder::new(reader).unwrap();

    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    sink.append(decoder);
    sink.sleep_until_end();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, slot_audio])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
