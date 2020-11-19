extern crate serde;

use evtx::EvtxParser;
use quick_xml::de::DeError;
use rusty_blue::detections::configs;
use rusty_blue::detections::detection;
use std::{fs, path::PathBuf, process};

fn main() -> Result<(), DeError> {
    if let Some(filepath) = configs::singleton().args.value_of("filepath") {
        parse_file(&filepath.to_string());
    }

    Ok(())
}

fn parse_file(filepath: &str) {
    let fp = PathBuf::from(filepath);
    let parser = match EvtxParser::from_path(fp) {
        Ok(pointer) => pointer,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };

    let mut detection = detection::Detection::new();
    &detection.start(parser);
}
