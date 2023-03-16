mod parse;
mod index_files;
mod trie;
mod lexer;

use std::path::{Path, PathBuf};
use std::process::exit;
use std::{collections::HashMap};
use std::env;

use index_files::{index, Document, Index};
use itertools::Itertools;
use lexer::Lexer;

fn main() -> std::io::Result<()> {
    let args : Vec<String> = env::args().collect_vec();
    if args.len() != 3 {
        println!(".. <path to repo> <search term>");
        exit(1)
    }


    let (doc_index, _triee) = index(Path::new(&args[1]), false, "json").unwrap();
    let search_result = search_term(args[2].clone(), &doc_index);
    println!("");
//  triee.print_ends();

    for (k, v) in search_result.iter().rev().take(15) {
        println!("{} -> {}", k.display(), v);
    }

    Ok(())
}

fn search_term(search_term: String, index: &Index) -> Vec<(PathBuf, f64)> {
    let search_charss = search_term.chars().collect::<Vec<_>>();
    let lexer = Lexer::new(&search_charss);

    let mut rankings: HashMap<PathBuf, f64> = HashMap::new();

    for term in lexer.into_iter().map(|s| String::from_iter(s).to_uppercase()) {

        let idf = ((index.len() as f64) / (index.iter()
            .filter(|(_, doc)| doc.contains_key(&term))
            .count() as f64 + 1.0)).ln() + 1.0;

        for (path, doc) in index {
            let tf = tf_in_document(&term, doc);
            let result = tf * idf;

            rankings.entry(path.to_path_buf())
                .and_modify(|rank| *rank += result)
                .or_insert(result);
        }
    }

    let mut res = rankings.into_iter().collect::<Vec<_>>();
    res.sort_by_key(|(_, v)| (*v * 100000.0) as i64);

    return res;
}


fn tf_in_document(term: &String, document: &Document) -> f64 {
    return document.get(term)
        .map(|f| ((*f as f64) + 1.0).ln()
            // / (document.len() as f64)
        )
        .unwrap_or(0.0)

}
