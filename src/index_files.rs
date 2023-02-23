use std::{
    path::{Path, PathBuf},
    fs::{DirEntry, File},
    io::{BufReader, Read},
    collections::HashMap
};

use crate::Lexer;

pub fn index(path: &Path, join_files: bool) -> Result<Index, std::io::Error> {
    return index_folder_content(path, join_files)
        .map(|r| r
            .into_iter()
            .collect::<HashMap<PathBuf, Document>>()
        )
}

pub fn index_folder_content(path: &Path, join_files: bool) -> std::io::Result<Vec<(PathBuf, HashMap<String, usize>)>> {

    let entries = path.read_dir()
        .expect("Failed to open provided path")
        .filter_map(|entry| entry.ok())
        .collect::<Vec<_>>();

    let files = entries.iter()
        .filter_map(|en| en.metadata()
            .ok()
            .and_then(|s|
                if s.is_file()
                    { Some(en) }
                else
                    { None }
            )
        ).collect::<Vec<_>>();

    let dirs = entries.iter()
        .filter_map(|en| en.metadata()
            .ok()
            .and_then(|s|
                if s.is_dir()
                    { Some(en) }
                else
                    { None }
            )
        ).collect::<Vec<_>>();

    let files_ind =  index_files(files, path.to_path_buf(), join_files)?;
    let dirs_ind = index_folders(dirs, join_files)?;

    return Ok(files_ind.into_iter().chain(dirs_ind.into_iter()).collect());
}

pub type Index = HashMap<PathBuf, HashMap<String, usize>>;
pub type Document = HashMap<String, usize>;

fn index_folders(folder_entries: Vec<&DirEntry>, join_files: bool) -> std::io::Result<Vec<(PathBuf, HashMap<String, usize>)>> {
    return Ok(
        folder_entries.iter()
            .map(|f| index_folder_content(&f.path(), join_files))
            .filter(|r| r.is_ok())
            .flat_map(|r| r.unwrap().into_iter())
            .collect()
    )

}

fn index_files(file_entreis: Vec<&DirEntry>, path: PathBuf, join_files: bool) -> std::io::Result<Vec<(PathBuf, HashMap<String, usize>)>> {
    if join_files {
        let mut collector = HashMap::new();

        for entry in file_entreis {
            index_single_file(entry, &mut collector)?;

        }
        return Ok(vec![(path, collector)]);
    }

    let mut res = Vec::new();
    for entry in file_entreis {
        let mut collector = HashMap::new();
        index_single_file(entry, &mut collector)?;
        res.push((entry.path(), collector))
    }

    Ok(res)
}

fn index_single_file(entry: &DirEntry, collector: &mut Document) -> std::io::Result<()> {
    // if !(entry.path().extension().map(|e| e == "json").unwrap_or(false)) {
    //     return Ok(());
    // }

    println!("Indexing {}", entry.path().display());
    let mut buffer = String::new();
    {
        let mut reader = BufReader::new(
            File::open(entry.path())?
        );
        reader.read_to_string(&mut buffer)?;
    }

    let binding = buffer.chars().collect::<Vec<_>>();
    let lex = Lexer::new(&binding);


    for token in lex {
        let value = String::from_iter(token).to_uppercase();
        collector.insert(
            value.clone(),
            if collector.contains_key(&value)
                { *collector.get(&value).unwrap() + 1 }
            else
                { 1 }
        );
    }

    Ok(())
}

// fn create_index_on_file(value: JsonValue, path: String) -> std::io::Result<FileIndex> {
//     fn index_inner(collector: &mut HashMap<String, usize>, val: JsonValue) {
//         match val {
//             JsonValue::StringValue(s_val) => {
//                 for sub_s_val in s_val.split_whitespace().map(str::to_lowercase) {
//                     collector.insert(
//                         sub_s_val.clone(), if collector.contains_key(&sub_s_val)
//                             { *collector.get(&sub_s_val).unwrap() + 1 }
//                         else
//                             { 1 }
//                     );
//                 }
//             }
//             JsonValue::ArrayValue(ar_val) => {
//                 for val in ar_val {
//                     index_inner(collector, val);
//                 }
//             }
//             JsonValue::ObjectValue(o_val) => {
//                 for (_, val) in o_val {
//                     index_inner(collector, val);
//                 }
//             },
//             JsonValue::IntegerValue(i_val) => {
//                 let s_val = i_val.to_string();
//                 collector.insert(
//                     s_val.clone(), if collector.contains_key(&s_val) { *collector.get(&s_val).unwrap() + 1 } else { 1 }
//                 );
//             },
//             JsonValue::DecimalValue(d_val) => {
//                 let s_val = d_val.to_string();
//                 collector.insert(
//                     s_val.clone(), if collector.contains_key(&s_val) { *collector.get(&s_val).unwrap() + 1 } else { 1 }
//                 );
//             },
//             JsonValue::BooleanValue(_) =>  { },
//             JsonValue::Null => { },

//         }
//     }
//     let mut index = HashMap::new();
//     index_inner(&mut index, value);
//     return Ok(FileIndex { rankings: index, path });
// }


// #[derive(Debug)]
// pub struct FileIndex {
//     pub rankings: Index,
//     pub path: String
// }

// impl Display for FileIndex {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}\n  {:#?}", self.path, self.rankings)
//     }
// }
