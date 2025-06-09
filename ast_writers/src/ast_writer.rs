use program_structure::ast::AST;
use serde::{Deserialize, Deserializer};

use std::io::{BufWriter, Write};
use std::fs::File;

pub struct AstWriter {
    pub output_file: String,
    pub ast_writer: BufWriter<File>,
}

impl AstWriter {
    pub fn new(output_file: String) -> Result<AstWriter, ()> {
        let file = File::create(&output_file).map_err(|_err| {})?;
        let mut writer = BufWriter::new(file);
        Result::Ok(AstWriter { output_file, ast_writer: writer })
    }

    /// Serialize AST to JSON file using `serde_json`
    pub fn serialize_ast(&mut self, program: &AST) -> Result<(), ()> {
        // Write program to json using serde
        let json = serde_json::to_string_pretty(program).map_err(|_err| {})?;
        self.ast_writer.write_all(json.as_bytes()).map_err(|_err| {});
        self.ast_writer.flush().map_err(|_err| {});
        Result::Ok(())
    }
}
