use std::fs::File;
use std::path::PathBuf;
use std::io::Write;

use askama::Template;
use derive_builder::Builder;

#[derive(Debug, Builder)]
pub struct TemplateWriter {
    pub path: PathBuf,
}

impl TemplateWriter {
    pub fn in_folder(&self, folder: &str) -> Self {
        let mut path = self.path.clone();
        path.push(folder);
        Self {
            path,
        }
    }

    pub fn ensure_path(&self) {
        if !self.path.exists() {
            tracing::info!(path = self.path.to_string_lossy().to_string(), "creating directory ...");
            std::fs::create_dir_all(self.path.clone()).unwrap();
        }
    }

    pub fn write_text(&self, filename: &str, content: String) {
        self.ensure_path();
        let mut path = self.path.clone();
        let folder = path.to_string_lossy().to_string();
        path.push(filename);
        tracing::info!(folder, filename, "writing file ...");
        let mut file = File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    pub fn write<T: Template>(&self, filename: &str, template: &T) {
        let content = template.render().unwrap();
        self.write_text(filename, content);
    }
}