#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

use std::env;
use std::fs::create_dir;
use std::path::PathBuf;
use std::process::exit;

use crate::gui::{error_message, show_gui};

mod gui;

fn main() {
  let target_raw = env::args_os().nth(1).unwrap_or_else(|| {
    env::home_dir()
      .unwrap_or_else(|| {
        error_message("Failed to get home directory.");
        exit(1)
      })
      .into()
  });
  let target_lstring = target_raw.to_string_lossy();
  let target_clean = target_lstring
    .strip_prefix('"')
    .unwrap_or(&target_lstring)
    .strip_suffix('"')
    .unwrap_or(&target_lstring)
    .replace("\\", "/");

  let name = find_new_folder_name(&PathBuf::from(&target_clean));

  show_gui(&target_clean, name.as_str())
}

pub fn create_folder(target: String, name: String) {
  let dir = PathBuf::from(&target).join(&name);
  if let Err(e) = create_dir(&dir) {
    error_message(format!("Failed to create '{}': {}", dir.display(), e).as_str());
    exit(1);
  }
  exit(0);
}

fn find_new_folder_name(parent: &PathBuf) -> String {
  const BASE: &str = "새 폴더";

  // '새 폴더' exists
  if !parent.join(BASE).exists() {
    return BASE.to_string();
  }

  // search for '새 폴더 (x)'
  for n in 2.. {
    let name = format!("{} ({})", BASE, n);
    if !parent.join(&name).exists() {
      return name;
    }
  }
  "!ERROR!".to_string()
}
