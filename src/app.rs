use eframe::{egui::{self, Visuals}, epi};
use epub::doc::EpubDoc;
use std::fs::{self};

use crate::{BookDetails, book_cover, grid};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct BooksHome {
    filter: String,
    book_paths: Vec<String>,
    books: Vec<BookDetails>
}

impl Default for BooksHome {
    fn default() -> Self {
        let files = match fs::read_dir("assets") {
            Ok(dir) => {
                let result = dir
                    .filter_map(|f| f.ok())
                    .filter_map(|f| f.path().into_os_string().into_string().ok())
                    .collect();
                result
            }
            Err(_) => Vec::<String>::new(),
        };

        Self {
            filter: "".to_owned(),
            book_paths: files,
            books: Vec::new()
        }
    }
}

impl BooksHome {
    pub fn load_books(&mut self) {
        self.books.reserve(self.book_paths.len());

        for path in &self.book_paths {
            let book = match EpubDoc::new(&path) {
                Ok(result) => result,
                Err(_) => continue
            };

            let title = match book.metadata.get("title") {
                Some(value) => value.get(0).unwrap_or(path),
                None => path
            };

            let author = match book.metadata.get("creator") {
                Some(value) => match value.get(0) {
                    Some(value) => Some(value.clone()),
                    None => None
                },
                None => None
            };
            
            let info = BookDetails {
                author: author,
                path: path.clone(),
                title: title.clone(),
                progress: 0.
            };
            self.books.push(info);
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
        let BooksHome { filter, book_paths, books } = self;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Books");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Filter books");
                ui.text_edit_singleline(filter);
            });
            ui.separator();

                let filter = filter.to_lowercase();
                let filtered_books: Vec<&BookDetails> = books
                    .iter()
                    .filter(|b| b.matches(&filter))
                    .collect();
                grid(ui, egui::vec2(200. / 3. * 2., 200.), filtered_books, book_cover);
        });
    }
}
