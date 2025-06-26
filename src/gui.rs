use std::process::exit;

use fltk::app::{IdleHandle, add_idle3, remove_idle3};
use fltk::button::Button;
use fltk::enums::{Event, Font, Key};
use fltk::frame::Frame;
use fltk::input::Input;
use fltk::prelude::*;
use fltk::window::Window;
use fltk::{app, image};
use fltk_theme::{ThemeType, WidgetTheme};
use native_dialog::{DialogBuilder, MessageLevel};

use crate::create_folder;

pub fn error_message(s: String) {
  DialogBuilder::message().set_level(MessageLevel::Error).set_title("ERROR").set_text(s).alert().show().unwrap();
}

pub fn show_error(e: FltkError, msg: &str) {
  error_message(format!("{}\n{}", e, msg));
}

pub fn show_gui(target: &str, name: &str) {
  // app with theme
  let app = app::App::default().with_scheme(app::Scheme::Gtk).load_system_fonts();
  let widget_theme = WidgetTheme::new(ThemeType::Dark);
  widget_theme.apply();

  // window
  let mut win = Window::new(100, 100, 342, 101, "New Folder");

  // font
  let fonts = app::fonts();
  let font_list = vec![("Noto Sans KR", 14), ("Malgun Gothic", 14)];

  if let Some((family, size)) = font_list.into_iter().find(|(family, _)| fonts.iter().any(|f| f == family)) {
    Font::set_font(Font::Helvetica, &family);
    app::set_font_size(size);
  }

  // window position
  win.set_pos((app::screen_size().0 / 2.0) as i32, (app::screen_size().1 / 2.0) as i32);

  // window icon
  let icon = image::PngImage::from_data(include_bytes!("../assets/new-folder.png")).unwrap_or_else(|e| {
    show_error(e, "Setting window icon failed");
    exit(1);
  });
  win.set_icon(Some(icon));

  // first row
  let _lbl_target = Frame::new(12, 15, 88, 12, "New Folder at:");
  let mut inp_target = Input::new(130, 12, 200, 21, "");
  inp_target.set_readonly(true);
  inp_target.set_tab_nav(false);
  inp_target.set_value(target);
  inp_target.set_position(target.len() as i32).unwrap_or_else(|e| {
    show_error(e, "Moving cursor of Input Target failed");
    exit(1);
  });

  // second row
  let _lbl_name = Frame::new(12, 42, 112, 12, "New Folder Name:");
  let mut inp_name = Input::new(130, 39, 200, 21, "");
  inp_name.set_value(name);
  inp_name.take_focus().unwrap_or_else(|e| {
    show_error(e, "Taking focus of Input Name failed");
    exit(1);
  });
  // text selection and focus will be done later

  // third row
  let mut btn_ok = Button::new(174, 66, 75, 23, "OK");
  let mut btn_cancel = Button::new(255, 66, 75, 23, "Cancel");

  // callback
  btn_ok.set_callback({
    let inp_target = inp_target.clone();
    let inp_name = inp_name.clone();
    move |_| {
      let target = inp_target.value();
      let name = inp_name.value();
      create_folder(target, name);
      app.quit();
    }
  });
  btn_cancel.set_callback(move |_| {
    app.quit();
  });

  inp_name.handle({
    let mut btn_ok = btn_ok.clone();
    let mut btn_cancel = btn_cancel.clone();
    move |_, ev| {
      if ev == Event::KeyDown {
        match app::event_key() {
          Key::Enter => {
            btn_ok.do_callback();
            true
          },
          Key::Escape => {
            btn_cancel.do_callback();
            true
          },
          _ => false,
        }
      } else {
        false
      }
    }
  });

  win.end();
  win.show();

  // select text
  let len = name.len() as i32;
  let _ = add_idle3({
    let mut inp_name = inp_name.clone();
    move |handle: IdleHandle| {
      let _ = inp_name.set_position(len);
      let _ = inp_name.set_mark(0);

      remove_idle3(handle);
    }
  });

  // Windows: dark title bar
  #[cfg(target_os = "windows")]
  {
    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::Graphics::Dwm::{DWMWA_USE_IMMERSIVE_DARK_MODE, DwmSetWindowAttribute};
    use windows_sys::core::BOOL;

    let hwnd = win.raw_handle() as HWND;
    let dark = 1;
    unsafe {
      DwmSetWindowAttribute(
        hwnd,
        DWMWA_USE_IMMERSIVE_DARK_MODE as u32,
        &dark as *const BOOL as *const _,
        size_of_val(&dark) as u32,
      );
    }
  }

  app.run().unwrap_or_else(|e| {
    show_error(e, "Executing app failed");
    exit(1);
  });
}
