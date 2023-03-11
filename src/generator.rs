#![allow(dead_code)]
use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

use comrak::{markdown_to_html, ComrakOptions};
use handlebars::{Handlebars, RenderError, TemplateFileError};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("this file is not a regular file: {0}")]
    NotAFile(PathBuf),
    #[error("this file is not a .md file: {0}")]
    WrongExtension(PathBuf),
    #[error("this file is missing file name: {0}")]
    MissingFilename(PathBuf),
    #[error("this path is not valid unicode: {0}")]
    InvalidUnicode(PathBuf),
    #[error("file read failed")]
    IoError(#[from] io::Error),
    #[error("failed to register templates directory")]
    TemplateDirError(#[from] Box<TemplateFileError>),
    #[error("failed to render handlebars template")]
    TemplateRenderError(#[from] Box<RenderError>),
}

impl From<TemplateFileError> for GeneratorError {
    // clippy complained about size of unboed TemplateFileError variant
    fn from(value: TemplateFileError) -> Self {
        Self::from(Box::new(value))
    }
}

impl From<RenderError> for GeneratorError {
    fn from(value: RenderError) -> Self {
        Self::from(Box::new(value))
    }
}

pub struct Generator<'a> {
    notes: Vec<Note>,
    output_dir: PathBuf,
    handlebars: Handlebars<'a>,
}

impl<'a> Generator<'a> {
    pub fn new(
        notes_dir: impl AsRef<Path>,
        output_dir: impl AsRef<Path>,
        template_dir: impl AsRef<Path>,
    ) -> Result<Self, GeneratorError> {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars.register_templates_directory(".hbs", template_dir)?;

        Ok(Self {
            notes: fs::read_dir(notes_dir)?
                .map(|entry| Note::open(entry?.path()))
                .collect::<Result<Vec<Note>, GeneratorError>>()?,
            output_dir: output_dir.as_ref().into(),
            handlebars,
        })
    }

    pub fn render(&self) -> Result<(), GeneratorError> {
        for note in self.notes.iter() {
            let markdown = markdown_to_html(&note.contents, &ComrakOptions::default());
            let mut output = self.output_dir.as_path().join(note.filename.as_str());

            output.set_extension("html");

            let data = json!({
                "note": {
                    "contents": markdown,
                },
            });

            let data = self.handlebars.render("note", &data)?;

            fs::write(output, data)?;
        }

        Ok(())
    }
}

struct Note {
    filename: String,
    contents: String,
}

impl Note {
    fn open(path: impl AsRef<Path>) -> Result<Self, GeneratorError> {
        let path = path.as_ref();

        if !path.is_file() {
            return Err(GeneratorError::NotAFile(path.into()));
        }

        if path.extension() != Some(OsStr::new("md")) {
            return Err(GeneratorError::WrongExtension(path.into()));
        }

        Ok(Self {
            filename: path
                .file_stem()
                .ok_or(GeneratorError::MissingFilename(path.into()))?
                .to_str()
                .ok_or(GeneratorError::InvalidUnicode(path.into()))?
                .to_string(),
            contents: fs::read_to_string(path)?,
        })
    }
}
