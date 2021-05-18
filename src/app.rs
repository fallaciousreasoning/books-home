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
