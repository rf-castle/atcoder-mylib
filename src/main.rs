use std::fs::{read_to_string, write, File, OpenOptions};
use std::io::{Read, Write};
use toml_edit::{value, Document, TomlError};

fn main() {
    // Auto run Test
    // Write to Somewhere
    replace_lib("../compete.toml", "./src/lib.rs").unwrap();
}

fn run_test() {}

fn replace_lib(toml_path: &str, lib_path: &str) -> Result<(), String> {
    let toml_content =
        read_to_string(toml_path).map_err(|why| format!("Reading toml file failed\n{}", why))?;
    let mut doc = parse_toml(toml_content.as_str())
        .map_err(|why| format!("Parsing toml file failed\n{}", why))?;
    let src_content =
        read_to_string(lib_path).map_err(|why| format!("Reading Library file failed\n{}", why))?;
    toml_doc_edit(&mut doc, src_content.as_str());
    write(toml_path, doc.to_string())
        .map_err(|why| format!("Writing toml file failed\n{}", why))?;
    Ok(())
}

fn parse_toml(src_toml: &str) -> Result<Document, TomlError> {
    src_toml.parse::<Document>()
}

fn toml_doc_edit(toml_doc: &mut Document, lib_content: &str) {
    toml_doc["template"]["src"] = value(lib_content);
}
