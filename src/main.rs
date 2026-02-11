#![cfg_attr(not(test), windows_subsystem = "windows")]

use arboard::Clipboard;
use ctrl_tex::convert_latex_to_unicode;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
#[cfg(target_os = "windows")]
use inputbot::KeybdKey::*;
use std::{thread, time::Duration};

#[cfg(target_os = "windows")]
fn main() {
    LKey.bind(|| {
        if LControlKey.is_pressed() && LShiftKey.is_pressed() {
            handle_conversion();
        }
    });
    inputbot::handle_input_events();
}

#[cfg(target_os = "linux")]
fn main() {
    handle_conversion();
}

fn handle_conversion() {
    // wait for hotkey release
    thread::sleep(Duration::from_millis(50));

    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();

    // copy selection
    let _ = enigo.key(Key::Control, Direction::Press);
    thread::sleep(Duration::from_millis(20));
    #[cfg(target_os = "windows")]
    let _ = enigo.key(Key::C, Direction::Click);
    #[cfg(not(target_os = "windows"))]
    let _ = enigo.key(Key::Unicode('c'), Direction::Click);
    thread::sleep(Duration::from_millis(20));
    let _ = enigo.key(Key::Control, Direction::Release);
    thread::sleep(Duration::from_millis(150));

    if let Ok(latex_text) = clipboard.get_text() {
        let unicode_text = convert_latex_to_unicode(&latex_text);

        if let Err(e) = clipboard.set_text(unicode_text) {
            eprintln!("Failed to set clipboard: {}", e);
            return;
        }
        thread::sleep(Duration::from_millis(50));

        // paste
        let _ = enigo.key(Key::Control, Direction::Press);
        thread::sleep(Duration::from_millis(20));
        #[cfg(target_os = "windows")]
        let _ = enigo.key(Key::V, Direction::Click);
        #[cfg(not(target_os = "windows"))]
        let _ = enigo.key(Key::Unicode('v'), Direction::Click);
        thread::sleep(Duration::from_millis(20));
        let _ = enigo.key(Key::Control, Direction::Release);
    } 

    thread::sleep(Duration::from_secs(2));
}

