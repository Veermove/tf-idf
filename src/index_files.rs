use std::{
    path::{Path, PathBuf},
    fs::{DirEntry, File},
    io::{BufReader, Read},
    collections::HashMap
};

use crate::{Lexer, trie::Triee};

pub fn index(path: &Path, join_files: bool, file_extension: &str)
    -> Result<(Index, Triee), std::io::Error> {

    let mut triee = Triee::new();
    return index_folder_content(&mut triee, path, join_files, file_extension)
        .map(|r|
            (
                r.into_iter()
                .collect::<HashMap<PathBuf, Document>>(),
                triee
            )
        )
}

fn index_folder_content(triee: &mut Triee, path: &Path, join_files: bool, file_extension: &str)
    -> std::io::Result<Vec<(PathBuf, HashMap<String, usize>)>> {

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

    let files_ind =  index_files(triee, files, path.to_path_buf(), join_files, file_extension)?;
    let dirs_ind = index_folders(triee, dirs, join_files, file_extension)?;

    return Ok(files_ind.into_iter().chain(dirs_ind.into_iter()).collect());
}

pub type Index = HashMap<PathBuf, HashMap<String, usize>>;
pub type Document = HashMap<String, usize>;

fn index_folders(triee: &mut Triee, folder_entries: Vec<&DirEntry>, join_files: bool, file_extension: &str) -> std::io::Result<Vec<(PathBuf, HashMap<String, usize>)>> {
    return Ok(
        folder_entries.iter()
            .map(|f| index_folder_content(triee, &f.path(), join_files, file_extension))
            .filter(|r| r.is_ok())
            .flat_map(|r| r.unwrap().into_iter())
            .collect()
    )

}

fn index_files(triee: &mut Triee, file_entreis: Vec<&DirEntry>, path: PathBuf, join_files: bool, file_extension: &str) -> std::io::Result<Vec<(PathBuf, HashMap<String, usize>)>> {
    if join_files {
        let mut collector = HashMap::new();

        for entry in file_entreis {
            index_single_file(triee, entry, &mut collector, file_extension)?;

        }
        return Ok(vec![(path, collector)]);
    }

    let mut res = Vec::new();
    for entry in file_entreis {
        let mut collector = HashMap::new();
        index_single_file(triee, entry, &mut collector, file_extension)?;
        res.push((entry.path(), collector))
    }

    Ok(res)
}

fn index_single_file(triee: &mut Triee, entry: &DirEntry, collector: &mut Document, file_extension: &str) -> std::io::Result<()> {
    if entry.path().extension().map(|e| e != file_extension).unwrap_or(true) {
        return Ok(());
    }

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
        // dbg!(token);
        let value = String::from_iter(token);//.to_uppercase();
        let up_token = value.chars().collect::<Vec<_>>();

        triee.insert_word(&up_token, entry.path());

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
