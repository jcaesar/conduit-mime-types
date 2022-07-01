use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

static JSON: &str = include_str!("data/mime.json");

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(default)]
    extensions: Vec<String>,
}

fn main() {
    let json: BTreeMap<String, Record> = serde_json::from_str(JSON).unwrap();

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut used_exts = HashSet::new();
    let mut ext_by_type = vec![];
    let mut type_by_ext = vec![];
    let mut ext_to_type = vec![];
    let mut type_to_ext = vec![];

    for (mime_type, record) in json.iter() {
        let exts = &record.extensions;

        for ext in exts {
            if used_exts.insert(ext) {
                type_by_ext.push(format!(r#""{}" => Some("{}")"#, ext, mime_type));
                type_to_ext.push(format!(r#"("{}", "{}")"#, ext, mime_type));
            }
        }

        ext_by_type.push(format!(
            r#""{}" => Some(&[{}])"#,
            mime_type,
            exts.iter()
                .map(|ext| format!("\"{}\"", ext))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        ext_to_type.push(format!(
            r#"("{}", &[{}])"#,
            mime_type,
            exts.iter()
                .map(|ext| format!("\"{}\"", ext))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    writeln!(
        &mut file,
        r#"fn gen_get_extension(name: &str) -> Option<&[&str]> {{
            match name {{
                {},
                _ => None
            }}
        }}"#,
        ext_by_type.join(",\n            ")
    )
    .unwrap();

    writeln!(
        &mut file,
        r#"fn gen_get_mime_type(ext: &str) -> Option<&str> {{
            match ext {{
                {},
                _ => None
            }}
        }}"#,
        type_by_ext.join(",\n            ")
    )
    .unwrap();

    writeln!(
        &mut file,
        r#"pub static EXT_BY_MIME: [(&'static str, &'static [&'static str]);{}] = [{}];"#,
        ext_to_type.len(),
        ext_to_type.join(",\n    ")
    )
    .unwrap();

    writeln!(
        &mut file,
        r#"pub static MIME_BY_EXT: [(&'static str, &'static str);{}] = [{}];"#,
        type_to_ext.len(),
        type_to_ext.join(",\n    ")
    )
    .unwrap();
}
