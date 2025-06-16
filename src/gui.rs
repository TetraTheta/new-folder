use std::fs::create_dir;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;

use eframe::{App, CreationContext, Frame, NativeOptions, run_native};
use egui::text::{CCursor, CCursorRange};
use egui::{Button, CentralPanel, Context, IconData, Id, TextEdit, Vec2, ViewportBuilder, ViewportCommand};
use egui_font::set_font;
use native_dialog::{DialogBuilder, MessageLevel};

struct AppState {
  target: PathBuf,
  new_name: String,
  autofocused: bool,
}

impl AppState {
  fn new(cc: &CreationContext<'_>, target: PathBuf, new_name: String) -> Self {
    set_font(&cc.egui_ctx);

    Self { target, new_name, autofocused: false }
  }

  /// Create new folder under `target` with the name of `new_name` and exit the app.
  fn create_folder_and_exit(&self) {
    let dir = self.target.join(&self.new_name);
    if let Err(e) = create_dir(&dir) {
      error_message(format!("Failed to create '{}': {}", dir.display(), e));
      exit(1);
    }
    exit(0);
  }

  /// Exit the app.
  fn cancel_and_exit(&self) {
    exit(0);
  }
}

impl App for AppState {
  fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
    // set style
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(10.0, 10.0);
    style.spacing.button_padding = Vec2::new(0.0, 0.0); // align text to center for both vertically and horizontally
    ctx.set_style(style);

    CentralPanel::default().show(ctx, |ui| {
      let mut target = remove_unc(self.target.to_string_lossy().to_string());

      // first row
      ui.horizontal(|ui| {
        ui.label("New Folder at: ");
        ui.add(TextEdit::singleline(&mut target).desired_width(f32::INFINITY));
      });
      // second row
      ui.horizontal(|ui| {
        ui.label("New Folder name: ");
        let mut output =
          TextEdit::singleline(&mut self.new_name).desired_width(f32::INFINITY).id(Id::new("new_name")).show(ui);
        let resp = output.response;

        // focus and select text when first draw
        if !self.autofocused {
          resp.request_focus();
          output
            .state
            .cursor
            .set_char_range(Some(CCursorRange::two(CCursor::new(0), CCursor::new(self.new_name.len()))));
          output.state.store(ui.ctx(), resp.id);
          self.autofocused = true;
        }

        // key event
        if resp.lost_focus() {
          if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.create_folder_and_exit();
          } else if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.cancel_and_exit();
          }
        }
      });
      // third row
      ui.horizontal(|ui| {
        // calculate spacer manually :(
        let button_w = 75.0;
        let spacing = ui.spacing().item_spacing.x;
        let spacer = (ui.available_width() - (button_w * 2.0 + spacing)).max(0.0);
        ui.add_space(spacer);

        if ui.add(Button::new("OK").min_size(Vec2::new(75.0, 23.0))).clicked() {
          self.create_folder_and_exit();
        }
        if ui.add(Button::new("Cancel").min_size(Vec2::new(75.0, 23.0))).clicked() {
          self.cancel_and_exit();
        }
      });
    });

    // shrink/resize the GUI
    let mut used = ctx.used_size();
    used.x = 386.0;
    ctx.send_viewport_cmd(ViewportCommand::InnerSize(used));

    ctx.request_repaint_after(Duration::from_millis(17)); // approx. 60FPS
  }
}

pub fn error_message(s: String) {
  DialogBuilder::message().set_level(MessageLevel::Error).set_title("ERROR").set_text(s).alert().show().unwrap();
}

fn load_icon() -> IconData {
  let icon = include_bytes!("../assets/new-folder.png");
  let img = image::load_from_memory(icon).expect("Failed to load icon").to_rgba8();
  let (w, h) = img.dimensions();
  IconData { rgba: img.into_raw(), width: w, height: h }
}

fn remove_unc(s: String) -> String {
  const PREFIX: &str = r"\\?\";
  if s.starts_with(PREFIX) { (&s[PREFIX.len()..]).parse().unwrap() } else { s }
}

pub fn run_gui(target: PathBuf, new_name: String) -> Result<(), eframe::Error> {
  let icon = Arc::new(load_icon());
  let opt = NativeOptions {
    viewport: ViewportBuilder {
      active: Some(true),
      maximize_button: Some(false),
      drag_and_drop: Some(false),
      max_inner_size: Some(Vec2::new(386.0, 1.0)), // this will be resized when update
      min_inner_size: Some(Vec2::new(386.0, 1.0)), // this will be resized when update
      close_button: Some(true),
      minimize_button: Some(true),
      icon: Some(icon),
      resizable: Some(false),
      title: Some("New Folder".to_string()),
      ..Default::default()
    },
    ..Default::default()
  };

  run_native("New Folder", opt, Box::new(move |cc| Ok(Box::new(AppState::new(cc, target, new_name)))))
}
