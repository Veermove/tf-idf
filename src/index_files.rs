use std::{str, path::{Path}, fs::{DirEntry, File}, io::{BufReader, Read}, collections::HashMap, fmt::Display, error::Error};

use crate::parse::{parse_json, JsonValue};

pub fn index_files(path: &Path) -> Result<Vec<FileIndex>, std::io::Error> {

    return Ok(
        path.read_dir()
            .expect("Failed to open provided path")
            .filter_map(|entry| entry.ok())
            .map(index_single)
            .filter_map(|v| v.ok())
            .flat_map(|v| v.into_iter())
            .collect::<Vec<_>>()
    )
}

type Index = HashMap<String, usize>;

#[derive(Debug)]
pub struct FileIndex {
    pub rankings: Index,
    pub path: String
}

impl Display for FileIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n  {:#?}", self.path, self.rankings)
    }
}

fn index_single(entry: DirEntry) -> std::io::Result<Vec<FileIndex>> {

    if entry.metadata()?.is_dir() {
        return index_files(&entry.path());
    }


    if !(entry.path().extension().map(|e| e == "json").unwrap_or(false)) {
        return Ok(vec![]);
    }

    let mut buffer = String::new();

    {
        let mut reader = BufReader::new(
            File::open(entry.path())?
        );
        reader.read_to_string(&mut buffer)?;
        // buffer = unescape(buffer.as_bytes()).expect("Faield reading as utf8");
    }

    if let Some(res) = parse_json(buffer) {
        return create_index_on_file(
            res,
            entry.path().to_str().unwrap_or("!").to_lowercase()
        ).map(|r| vec![r]);
    }
    Ok(vec![])
}

fn create_index_on_file(value: JsonValue, path: String) -> std::io::Result<FileIndex> {
    fn index_inner(collector: &mut HashMap<String, usize>, val: JsonValue) {
        match val {
            JsonValue::StringValue(s_val) => {
                for sub_s_val in s_val.split_whitespace().map(str::to_lowercase) {
                    collector.insert(
                        sub_s_val.clone(), if collector.contains_key(&sub_s_val)
                            { *collector.get(&sub_s_val).unwrap() + 1 }
                        else
                            { 1 }
                    );
                }
            }
            JsonValue::ArrayValue(ar_val) => {
                for val in ar_val {
                    index_inner(collector, val);
                }
            }
            JsonValue::ObjectValue(o_val) => {
                for (_, val) in o_val {
                    index_inner(collector, val);
                }
            },
            JsonValue::IntegerValue(i_val) => {
                let s_val = i_val.to_string();
                collector.insert(
                    s_val.clone(), if collector.contains_key(&s_val) { *collector.get(&s_val).unwrap() + 1 } else { 1 }
                );
            },
            JsonValue::DecimalValue(d_val) => {
                let s_val = d_val.to_string();
                collector.insert(
                    s_val.clone(), if collector.contains_key(&s_val) { *collector.get(&s_val).unwrap() + 1 } else { 1 }
                );
            },
            JsonValue::BooleanValue(_) =>  { },
            JsonValue::Null => { },

        }
    }
    let mut index = HashMap::new();
    index_inner(&mut index, value);
    return Ok(FileIndex { rankings: index, path });
}
