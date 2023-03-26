use std::path::PathBuf;

use crossbeam::{channel::Receiver, select};
use log::error;

use crate::generator::Generator;

pub struct Watcher<'a> {
    generator: Generator<'a>,
    notes: Receiver<PathBuf>,
    templates: Receiver<PathBuf>,
}

impl<'a> Watcher<'a> {
    pub fn new(
        generator: Generator<'a>,
        notes: Receiver<PathBuf>,
        templates: Receiver<PathBuf>,
    ) -> Self {
        Self {
            generator,
            notes,
            templates,
        }
    }

    pub fn watch(&mut self) {
        loop {
            select! {
                recv(self.notes) -> msg => {
                    if let Ok(path) = msg {
                        if let Err(e) = self.generator.render_path(&path) {
                            error!("Failed to generate note: {}", e);
                        }
                    }
                }
                recv(self.templates) -> msg => {
                    match msg {
                        Ok(path) => {
                            if let Err(e) = self.generator.update_template(&path) {
                                error!("Failed to update templates: {}", e);
                            }
                        }
                        Err(e) => error!("Failed to receive template: {}", e),
                    }
                }
            }
        }
    }
}
