#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

mod gui;

use std::env;
use std::path::PathBuf;
use std::process::exit;

use crate::gui::{error_message, run_gui};

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
  unreachable!() // I don't think this can be the case
}

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

  let new_folder_name = find_new_folder_name(&target);

  if let Err(e) = run_gui(target, new_folder_name) {
    error_message(format!("GUI error: {}", e));
    exit(1);
  }
}
