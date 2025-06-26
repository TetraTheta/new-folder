#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

use std::env;
use std::fs::create_dir;
use std::path::PathBuf;
use std::process::exit;

use crate::gui::{error_message, show_gui};

mod gui;

fn main() {
  let args: Vec<String> = env::args().collect();
  let target = if args.len() > 1 {
    PathBuf::from(&args[1])
  } else {
    env::home_dir().unwrap_or_else(|| {
      error_message("Failed to get home directory.".to_string());
      exit(1)
    })
  };
  let name = find_new_folder_name(&target);

  let target_string = target.to_string_lossy();
  let target_normalized = target_string.replace("\\", "/");
  let name_str = name.as_str();

  show_gui(&target_normalized, name_str)
}

pub fn create_folder(target: String, name: String) {
  let dir = PathBuf::from(&target).join(&name);
  if let Err(e) = create_dir(&dir) {
    error_message(format!("Failed to create '{}': {}", dir.display(), e));
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
