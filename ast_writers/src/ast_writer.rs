use program_structure::ast::AST;
use program_structure::program_archive::ProgramArchive;
use serde::{Deserialize, Deserializer};

use std::io::{BufWriter, Write};
use std::fs::File;

pub struct AstWriter {
    pub output_file: String,
    pub ast_writer: BufWriter<File>,
}

impl AstWriter {
    pub fn new(output_file: &str) -> Result<AstWriter, ()> {
        let file = File::create(output_file).map_err(|_err| {})?;
        let writer = BufWriter::new(file);
        Result::Ok(AstWriter { output_file: output_file.to_owned(), ast_writer: writer })
    }

    /// Serialize program AST to JSON file using `serde_json`
    pub fn serialize_program_ast(&mut self, program: &AST) -> Result<(), ()> {
        // Write program to json using serde
        let json = serde_json::to_string_pretty(program).map_err(|_err| {})?;
        self.ast_writer.write_all(json.as_bytes()).map_err(|_err| {}).ok();
        self.ast_writer.flush().map_err(|_err| {}).ok();
        Result::Ok(())
    }

    /// Serialize program archive to JSON file using `serde_json`
    pub fn serialize_program_archive(&mut self, program: &ProgramArchive) -> Result<(), ()> {
        // Write program to json using serde
        let json = serde_json::to_string_pretty(program).map_err(|_err| {})?;
        self.ast_writer.write_all(json.as_bytes()).map_err(|_err| {}).ok();
        self.ast_writer.flush().map_err(|_err| {}).ok();
        Result::Ok(())
    }
}
