// MIT License
//
// Copyright (c) 2025 VIFEX
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use argparse::{ArgumentParser, Store};
use ini::Ini;
use notify_rust::Notification;
use std::fs;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn try_send_notification(config_path: &str) -> bool {
    // Check if the configuration file exists
    if !fs::metadata(&config_path).is_ok() {
        return false;
    }

    // Read the configuration file
    let config_content = fs::read_to_string(&config_path).expect("Failed to read config file");
    let config = Ini::load_from_str(&config_content).expect("Failed to parse config file");

    // Get summary, body, and sound_file from the configuration file
    let summary = config
        .section(Some("notification"))
        .and_then(|s| s.get("summary"))
        .unwrap_or("Default Title");
    let body = config
        .section(Some("notification"))
        .and_then(|s| s.get("body"))
        .unwrap_or("Default Content");
    let sound_file = config
        .section(Some("notification"))
        .and_then(|s| s.get("sound_file"))
        .unwrap_or("/usr/share/sounds/freedesktop/stereo/complete.oga");

    // Send notification
    Notification::new()
        .summary(summary)
        .body(body)
        .show()
        .unwrap();

    // Play sound
    Command::new("paplay")
        .arg(sound_file)
        .status()
        .expect("Failed to play sound");

    // Delete the configuration file
    fs::remove_file(&config_path).expect("Failed to delete config file");

    return true;
}

fn main() {
    // Define configuration file path parameter
    let mut config_path = String::from("/tmp/notification-rust.ini");
    let mut duration = 1u64;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Send notification and play sound");
        ap.refer(&mut config_path)
            .add_option(&["-c", "--config"], Store, "Configuration file path");
        ap.refer(&mut duration)
            .add_option(&["-d", "--duration"], Store, "Timing duration (seconds)");
        ap.parse_args_or_exit();
    }

    loop {
        try_send_notification(&config_path);
        thread::sleep(Duration::from_secs(duration));
    }
}
