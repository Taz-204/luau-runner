#![windows_subsystem = "windows"] // hides the console window in release builds

use eframe::egui;
use mlua::{Lua, MultiValue, VmState};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

struct App {
    code: String,
    output: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            code: "local sum = 0\nfor i = 1, 10 do\n    sum += i\nend\nprint(\"sum:\", sum)".into(),
            output: String::new(),
        }
    }
}

/// Runs a script in a fresh Luau VM and returns everything it printed,
/// plus any error message. A timeout stops runaway loops.
fn run_script(code: &str, timeout: Duration) -> String {
    let captured = Arc::new(Mutex::new(String::new()));
    let lua = Lua::new();

    // Replace `print` so its output goes to our output panel.
    let sink = captured.clone();
    let print = match lua.create_function(move |_, args: MultiValue| {
        let parts: Vec<String> = args
            .iter()
            .map(|v| v.to_string())
            .collect::<mlua::Result<_>>()?;
        let mut out = sink.lock().unwrap();
        out.push_str(&parts.join("\t"));
        out.push('\n');
        Ok(())
    }) {
        Ok(f) => f,
        Err(e) => return format!("Setup error: {e}"),
    };
    if let Err(e) = lua.globals().set("print", print) {
        return format!("Setup error: {e}");
    }

    // Stop the script if it runs longer than the timeout.
    let deadline = Instant::now() + timeout;
    lua.set_interrupt(move |_| {
        if Instant::now() > deadline {
            Err(mlua::Error::RuntimeError("Script timed out".into()))
        } else {
            Ok(VmState::Continue)
        }
    });

    let mut result = match lua.load(code).exec() {
        Ok(()) => String::new(),
        Err(e) => format!("Error: {e}\n"),
    };

    let printed = captured.lock().unwrap().clone();
    if !printed.is_empty() {
        result = format!("{printed}{result}");
    }
    if result.is_empty() {
        result = "(no output)".into();
    }
    result
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Luau Runner");

            ui.add(
                egui::TextEdit::multiline(&mut self.code)
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .desired_rows(18),
            );

            ui.horizontal(|ui| {
                if ui.button("▶ Execute").clicked() {
                    self.output = run_script(&self.code, Duration::from_secs(2));
                }
                if ui.button("Clear output").clicked() {
                    self.output.clear();
                }
            });

            ui.separator();
            ui.label("Output:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(egui::RichText::new(&self.output).monospace());
            });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Luau Runner",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}
