use codespan_reporting::files::{Files, SimpleFiles};
use serde::Serialize;
use std::ops::Range;

pub type FileSource = String;
pub type FilePath = String;
pub type FileID = usize;
pub type FileLocation = Range<usize>;
type FileStorage = SimpleFiles<FilePath, FileSource>;

#[derive(Clone)]
pub struct FileLibrary {
    files: FileStorage,
    file_id_max: isize,
}

impl Default for FileLibrary {
    fn default() -> Self {
        FileLibrary { files: FileStorage::new(), file_id_max: -1 }
    }
}

impl FileLibrary {
    pub fn new() -> FileLibrary {
        FileLibrary::default()
    }
    pub fn add_file(&mut self, file_name: FilePath, file_source: FileSource) -> FileID {
        let file_id = self.get_mut_files().add(file_name, file_source);
        self.file_id_max = file_id as isize;
        file_id
    }
    pub fn get_line(&self, start: usize, file_id: FileID) -> Option<usize> {
        match self.files.line_index(file_id, start) {
            Some(lines) => Some(lines + 1),
            None => None,
        }
    }
    pub fn to_storage(&self) -> &FileStorage {
        &self.get_files()
    }
    fn get_files(&self) -> &FileStorage {
        &self.files
    }
    fn get_mut_files(&mut self) -> &mut FileStorage {
        &mut self.files
    }
}

impl Serialize for FileLibrary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let files = self.get_files();
        let mut file_id_mapping: Vec<(usize, String)> = Vec::new();
        for i in 0..self.file_id_max {
            if let Some(file) = files.get(i as usize) {
                let file_name = file.name();
                // use serde_json to normalize the file name, if it is is a JSON String format.
                let file_name: String =
                    serde_json::from_str(file_name).unwrap_or_else(|_| file_name.to_string());
                file_id_mapping.push((i as usize, file_name));
            }
        }
        serializer.collect_map(file_id_mapping.iter().map(|(id, name)| (id, name)))
    }
}

pub fn generate_file_location(start: usize, end: usize) -> FileLocation {
    start..end
}
