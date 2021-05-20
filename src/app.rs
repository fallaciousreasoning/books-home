use eframe::{egui::{self, Visuals}, epi};
use std::fs::{self};

use crate::{BookDetails, book_cover, grid};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct BooksHome {
    filter: String,
    books: Vec<String>,
}

impl Default for BooksHome {
    fn default() -> Self {
        let mut test_files: Vec<String> = vec!["Test", "Foo", "Bar", "Hello", "World", "Baz"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let mut files = match fs::read_dir("assets") {
            Ok(dir) => {
                let result = dir
                    .filter_map(|f| f.ok())
                    .filter_map(|f| f.path().into_os_string().into_string().ok())
                    .collect();
                result
            }
            Err(_) => Vec::<String>::new(),
        };
        files.append(&mut test_files);

        Self {
            filter: "".to_owned(),
            books: files,
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

    fn setup(&mut self, ctx: &egui::CtxRef) {
        ctx.set_visuals(Visuals::light());
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, _: &mut epi::Frame<'_>) {
        let BooksHome { filter, books } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Books");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Filter books");
                ui.text_edit_singleline(filter);
            });
            ui.separator();

                let filter = filter.to_lowercase();
                let filtered_books: Vec<BookDetails> = books
                    .iter()
                    .filter(|b| b.to_lowercase().matches(&filter).next() != None)
                    .map(|path| BookDetails {
                        title: path.to_string(),
                        progress: 0.5
                    }).collect();
                grid(ui, egui::vec2(200. / 3. * 2., 200.), filtered_books, book_cover);
        });
    }
}
