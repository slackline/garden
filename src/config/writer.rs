extern crate yaml_rust;

use std::io::Write;

use self::yaml_rust::YamlEmitter;
use self::yaml_rust::Yaml;

/// Write a Yaml object to a file

pub fn write_yaml<P>(doc: &Yaml, path: P) -> bool
where P: std::convert::AsRef<std::path::Path> + std::fmt::Debug {
    // Emit the YAML configuration into a string
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&doc).ok(); // dump the YAML object to a String
    }
    out_str += "\n";

    let file_result = std::fs::File::create(&path);
    if file_result.is_err() {
        errmsg!("{:?}: unable to create configuration: {}",
                path, file_result.as_ref().err().unwrap());
        return false;
    }

    let mut file = file_result.unwrap();
    let write_result = file.write_all(&out_str.into_bytes());
    if write_result.is_err() {
        errmsg!("unable to write configuration: {:?}", path);
        return false;
    }

    if let Err(err) = file.sync_all() {
        errmsg!("unable to sync files: {}", err);
        return false;
    }

    true
}
