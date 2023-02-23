mod parse;
mod index_files;

use std::path::Path;

use parse::parse_json;
use index_files::index_files;

fn main() {
    // let mut a = "{\"glossary\":{\"title\":\"example glossary\",\"GlossDiv\":{\"title\":\"S\",\"GlossList\":{\"GlossEntry\":{\"ID\":\"SGML\",\"SortAs\":\"SGML\",\"GlossTerm\":\"Standard Generalized Markup Language\",\"Acronym\":\"SGML\",\"Abbrev\":\"ISO 8879:1986\",\"GlossDef\":{\"para\":\"A meta-markup language, used to create markup languages such as DocBook.\",\"GlossSeeAlso\":[\"GML\",\"XML\"]},\"GlossSee\":\"markup\"}}}}}".to_owned();
    // let d = "{\"test\": \"21\", \"test2\": 213, \"test3\": 231.421}".to_owned();
    // let c = "{\"a\":[[], {}]}";
    // // let res = parse_json(a);
    // let res = parse_json(a.to_owned());
    // dbg!(res);
    let v = index_files(Path::new("./messages/inbox")).unwrap();
    println!("{:#?}", v.len());
    println!("{:#?}", v[22])
}

