use std::{
    collections::HashMap,
    env::args,
    fs::File,
    io::{stdout, BufReader, Read, Write},
};

use lcov::{Reader, Record};

fn main() {
    let args = args().into_iter().collect::<Vec<String>>();

    let mut lcov = File::open(&args[1]).unwrap();
    let mut lcov_content = Vec::new();
    lcov.read_to_end(&mut lcov_content).unwrap();

    let lcov_str = String::from_utf8_lossy(&lcov_content);
    let reader = Reader::new(BufReader::new(lcov_str.as_bytes()));

    let records = reader.collect::<Result<Vec<_>, _>>().unwrap();

    let mut current = String::new();
    let mut map = HashMap::new();
    for record in records {
        match record {
            Record::SourceFile { path } => {
                current = path.to_str().unwrap().to_owned();
                if !map.contains_key(&current) {
                    map.insert(current.clone(), Vec::<u32>::new());
                }
            }
            Record::LineData { line, .. } => {
                map.get_mut(&current).unwrap().push(line);
            }
            _ => {}
        }
    }

    stdout()
        .write_all(serde_json::to_string_pretty(&map).unwrap().as_bytes())
        .unwrap();
}
