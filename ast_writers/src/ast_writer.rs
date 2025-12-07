use program_structure::ast::AST;
use serde::{Deserialize, Deserializer};

use std::io::{BufWriter, Write};
use std::fs::File;
use std::path::PathBuf;

pub struct AstWriter {
    pub output_file: String,
    pub ast_writer: BufWriter<File>,
}

impl AstWriter {
    pub fn new(output_path: &PathBuf, input_file: &str) -> Result<AstWriter, ()> {
        // Input file might be wrapped by " ", so we need to remove them
        let mut input_file = input_file.to_string();
        if input_file.starts_with('"') && input_file.ends_with('"') {
            input_file = input_file[1..input_file.len() - 1].to_string();
        }
        // Create output file path
        let input_file_path = PathBuf::from(input_file);
        let input_file_name = input_file_path.file_stem().ok_or(())?.to_str().ok_or(())?;
        let output_file_path = output_path.join(format!("{}_ast.json", input_file_name));
        if output_file_path.exists() {
            println!(
                "Warning: Output file {} already exists and will be overwritten.",
                output_file_path.to_string_lossy()
            );
        }
        let output_file = File::create(&output_file_path).map_err(|_err| {})?;
        let writer = BufWriter::new(output_file);
        Result::Ok(AstWriter {
            output_file: output_file_path.to_string_lossy().into_owned(),
            ast_writer: writer,
        })
    }

    /// Serialize circuit AST to JSON file using `serde_json`
    pub fn serialize_ast(&mut self, program: &AST) -> Result<(), ()> {
        // Write circuit AST to json using serde
        let json = serde_json::to_string_pretty(program).map_err(|_err| {})?;
        self.ast_writer.write_all(json.as_bytes()).map_err(|_err| {}).ok();
        self.ast_writer.flush().map_err(|_err| {}).ok();
        Result::Ok(())
    }
}
