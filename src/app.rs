use eframe::{egui, epi};

use crate::{BookDetails, book_cover};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct BooksHome {
    filter: String,
    books: Vec<String>
}

impl Default for BooksHome {
    fn default() -> Self {
        Self {
            filter: "".to_owned(),
            books: vec![
                "assets/The Count of Monte Cristo, Illu - Alexandre Dumas.epub".to_owned()
            ]
        }
    }
}

impl epi::App for BooksHome {
    fn name(&self) -> &str {
        "Books Home"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let BooksHome {
            filter,
            books,
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Books");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Filter books");
                ui.text_edit_singleline(filter);
            });
            ui.separator();
            book_cover(ui, BookDetails {
                title: "A book".to_owned(),
                progress: 0.5
            });
        });
    }
}

// ----------------------------------------------------------------------------

/// Example code for painting on a canvas with your mouse
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
struct Painting {
    lines: Vec<Vec<egui::Pos2>>,
    stroke: egui::Stroke,
}

impl Default for Painting {
    fn default() -> Self {
        Self {
            lines: Default::default(),
            stroke: egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE),
        }
    }
}

impl Painting {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            egui::stroke_ui(ui, &mut self.stroke, "Stroke");
            ui.separator();
            if ui.button("Clear Painting").clicked() {
                self.lines.clear();
            }
        })
        .response
    }

    pub fn ui_content(&mut self, ui: &mut egui::Ui) -> egui::Response {
        use egui::emath::{Pos2, Rect, RectTransform};

        let (mut response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap_finite(), egui::Sense::drag());

        let to_screen = RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();

        if self.lines.is_empty() {
            self.lines.push(vec![]);
        }

        let current_line = self.lines.last_mut().unwrap();

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = from_screen * pointer_pos;
            if current_line.last() != Some(&canvas_pos) {
                current_line.push(canvas_pos);
                response.mark_changed();
            }
        } else if !current_line.is_empty() {
            self.lines.push(vec![]);
            response.mark_changed();
        }

        let mut shapes = vec![];
        for line in &self.lines {
            if line.len() >= 2 {
                let points: Vec<Pos2> = line.iter().map(|p| to_screen * *p).collect();
                shapes.push(egui::Shape::line(points, self.stroke));
            }
        }
        painter.extend(shapes);

        response
    }
}
