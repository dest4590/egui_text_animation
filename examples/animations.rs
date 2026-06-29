#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]

use eframe::egui;
use egui_text_animation::{AnimationType, TextAnimator};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 320.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Text Animation Example",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    fade_animator: TextAnimator,
    typewriter_animator: TextAnimator,
    hacker_animator: TextAnimator,
    animation_running: bool,
    speed: f32,
    selected_animation: AnimationType, // Store the selected animation type
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            fade_animator: TextAnimator::new(
                "Hello, Fade In!",
                egui::FontId::new(18.0, egui::FontFamily::Proportional),
                egui::Color32::WHITE,
                0.5,
                AnimationType::FadeIn,
            ),

            typewriter_animator: TextAnimator::new(
                "Hello, Typewriter!",
                egui::FontId::new(18.0, egui::FontFamily::Proportional),
                egui::Color32::WHITE,
                0.5,
                AnimationType::Typewriter,
            ),
            hacker_animator: TextAnimator::new(
                "Access Granted",
                egui::FontId::new(18.0, egui::FontFamily::Proportional),
                egui::Color32::GREEN,
                2.0, // Hacker animation often looks better a bit faster
                AnimationType::Hacker,
            ),

            animation_running: false,
            speed: 2.0,                                // Initial speed
            selected_animation: AnimationType::FadeIn, // Default to FadeIn
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            // --- Animation Selection ---
            ui.horizontal(|ui| {
                ui.label("Select Animation:");
                ui.radio_value(
                    &mut self.selected_animation,
                    AnimationType::FadeIn,
                    "Fade In",
                );
                ui.radio_value(
                    &mut self.selected_animation,
                    AnimationType::Typewriter,
                    "Typewriter",
                );
                ui.radio_value(
                    &mut self.selected_animation,
                    AnimationType::Hacker,
                    "Hacker",
                );
            });

            // --- Start/Stop Buttons ---
            ui.horizontal(|ui| {
                if ui.button("Start Animation").clicked() {
                    self.animation_running = true;
                    // Reset the *correct* animator based on selection.
                    match self.selected_animation {
                        AnimationType::FadeIn => self.fade_animator.reset(),
                        AnimationType::Typewriter => self.typewriter_animator.reset(),
                        AnimationType::Hacker => self.hacker_animator.reset(),
                    }
                }
                if ui.button("Stop Animation").clicked() {
                    self.animation_running = false; // Don't reset; just stop.
                }
            });

            // --- Speed Control ---
            ui.horizontal_wrapped(|ui| {
                ui.label("Speed:");
                if ui
                    .add(egui::Slider::new(&mut self.speed, 0.1..=10.0))
                    .changed()
                {
                    // Update speed for *all* animators
                    self.fade_animator.set_speed(self.speed);
                    self.typewriter_animator.set_speed(self.speed);
                    self.hacker_animator.set_speed(self.speed);
                }
            });

            // --- Font Size Control ---
            ui.horizontal(|ui| {
                ui.label("Font Size:");
                let mut font_size = self.fade_animator.font.size;
                if ui
                    .add(egui::Slider::new(&mut font_size, 1.0..=100.0))
                    .changed()
                {
                    self.fade_animator.font.size = font_size;
                    self.typewriter_animator.font.size = font_size;
                    self.hacker_animator.font.size = font_size;
                }
            });

            // --- Controlled Animation ---
            if self.animation_running {
                let ctx = ui.ctx().clone();
                let (animator, finished) = match self.selected_animation {
                    AnimationType::FadeIn => {
                        self.fade_animator.process_animation(&ctx);
                        let finished = self.fade_animator.is_animation_finished();
                        (&mut self.fade_animator, finished)
                    }
                    AnimationType::Typewriter => {
                        self.typewriter_animator.process_animation(&ctx);
                        let finished = self.typewriter_animator.is_animation_finished();
                        (&mut self.typewriter_animator, finished)
                    }
                    AnimationType::Hacker => {
                        self.hacker_animator.process_animation(&ctx);
                        let finished = self.hacker_animator.is_animation_finished();
                        (&mut self.hacker_animator, finished)
                    }
                };
                animator.render(ui);

                if !finished {
                    ctx.request_repaint();
                }
            } else {
                // static render
                match self.selected_animation {
                    AnimationType::FadeIn => self.fade_animator.render(ui),
                    AnimationType::Typewriter => self.typewriter_animator.render(ui),
                    AnimationType::Hacker => self.hacker_animator.render(ui),
                };
            }

            if self.animation_running
                && match self.selected_animation {
                    AnimationType::FadeIn => self.fade_animator.is_animation_finished(),
                    AnimationType::Typewriter => self.typewriter_animator.is_animation_finished(),
                    AnimationType::Hacker => self.hacker_animator.is_animation_finished(),
                }
            {
                ui.label("Animation finished!");
            }
        });
    }
}
