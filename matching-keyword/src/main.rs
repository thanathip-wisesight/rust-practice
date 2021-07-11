use csv::{ByteRecord, ReaderBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::str;

mod matcher;

#[derive(Debug, Serialize, Deserialize)]
struct Patterns {
    include: Vec<String>,
    name: String,
    exclude: Vec<String>,
}

fn parse_json(path_json: &str) -> Result<Patterns> {
    let file = File::open(&path_json).expect("file should open read only");
    let json: Patterns = serde_json::from_reader(file).expect("file should be proper JSON");
    Ok(json)
}

fn parse_csv(path_csv: &str) {
    let f = BufReader::new(fs::File::open(&path_csv).unwrap());
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(f);
    let mut record = ByteRecord::new();

    while let Ok(true) = reader.read_byte_record(&mut record) {
        // if let Some(bytes) = record.get(0) {
        //     let s = str::from_utf8(bytes);
        //     println!("{:?}", s);
        // }
    }
}

fn filter_condition(list_match: Vec<String>) -> (Vec<String>, Vec<Vec<String>>) {
    let mut list_match_temp = list_match;
    let mut multiple_condition: Vec<Vec<String>> = Vec::new();
    let multiple_condition_temp: Vec<String> = list_match_temp
        .iter()
        .filter(|&element| element.contains("+"))
        .cloned()
        .collect();

    for condition in multiple_condition_temp {
        multiple_condition.push(split_condition(&condition))
    }

    // Remove condition contains x
    list_match_temp.retain(|x| !x.contains("+"));

    (list_match_temp, multiple_condition)
}

fn split_condition(line: &str) -> Vec<String> {
    line.split("+").map(str::to_string).collect()
}

#[allow(unused_variables)]
fn main() {
    let path_json = "./test.json";
    let message = "ไม่ส่งบ้านอ่อ🥺 เธอ hello test home hi good go house";
    let res = parse_json(&path_json).expect("err parse_json");
    let patterns = Patterns {
        include: res.include,
        exclude: res.exclude,
        name: res.name,
    };

    parse_csv("./message.test.csv");

    let (patterns_include_condition, patterns_include_multiple_condition) =
        filter_condition(patterns.include);
    let (patterns_exclude_condition, patterns_exclude_multiple_condition) =
        filter_condition(patterns.exclude);

    // init aho
    let ac_patterns_include_condition = matcher::generator_aho_match(patterns_include_condition);
    let ac_patterns_exclude_condition = matcher::generator_aho_match(patterns_exclude_condition);

    assert_eq!(
        true,
        matcher::is_match(ac_patterns_exclude_condition, &message)
    );
    assert_eq!(
        true,
        matcher::run_match_multiple_condition(patterns_exclude_multiple_condition, &message)
    );

    assert_eq!(
        true,
        matcher::is_match(ac_patterns_include_condition, &message)
    );

    assert_eq!(
        true,
        matcher::run_match_multiple_condition(patterns_include_multiple_condition, &message)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_condition() {
        let dummy = vec![
            "test".to_string(),
            "home".to_string(),
            "word+key".to_string(),
        ];
        let result = filter_condition(dummy);
        assert_eq!(
            (
                vec!["test".to_string(), "home".to_string()],
                vec![vec!["word".to_string(), "key".to_string()]]
            ),
            result
        );
    }
}