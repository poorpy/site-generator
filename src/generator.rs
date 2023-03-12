#![allow(dead_code)]
use std::{
    ffi::OsStr,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
    process::Command,
    str::{self, Utf8Error},
};

use comrak::{markdown_to_html, ComrakOptions};
use handlebars::{Handlebars, RenderError, TemplateFileError};
use log::{error, info};
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
    #[error("io operation failed")]
    IoError(#[from] io::Error),
    #[error("subprocess output contained invalid utf-8")]
    Utf8Error(#[from] Utf8Error),
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
            self.render_note(note)?;
        }

        Ok(())
    }

    pub fn generate_css(&self) -> Result<(), GeneratorError> {
        info!("generating css with tailwind ...");

        let mut command: Command = Command::new("tailwind");
        command.args(["-i", "./tailwind/input.css", "-o", "./output/output.css"]);

        let output = command.output()?;

        if !output.status.success() {
            error!(
                "command {command:?} failed with error:\n{}",
                str::from_utf8(&output.stderr)?
            );
        }

        info!("done");

        Ok(())
    }

    pub fn render_path(&self, path: impl AsRef<Path>) -> Result<(), GeneratorError> {
        let path = path.as_ref();

        let note = Note::open(path)?;
        info!("rendering file: {}", note.filename);
        self.render_note(&note)?;

        Ok(())
    }

    fn render_note(&self, note: &Note) -> Result<(), GeneratorError> {
        let output = self
            .output_dir
            .as_path()
            .join(&note.filename)
            .with_extension("html");

        let output = File::create(output)?;

        let data = json!({
            "note": {
                "contents": markdown_to_html(&note.contents, &ComrakOptions::default()),
            },
        });

        Ok(self.handlebars.render_to_write("note", &data, output)?)
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
