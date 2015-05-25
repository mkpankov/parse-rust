#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

use std::collections::BTreeMap;
use std::io::BufRead;

fn main() {
    let re = regex!(concat!(
        r#""GET (?:https?://.+?/)?(.+?\.mp3)"#,
        r#" HTTP/1\.[01]" \d{3} (\d+)"#));
    let stdin = std::io::stdin();
    let locked_stdin = stdin.lock();
    let mut stat: BTreeMap<String, (isize, isize)> = BTreeMap::new();

    for maybe_line in locked_stdin.lines() {
        let line = maybe_line.unwrap();
        let captures: Vec<_> = re.captures_iter(&line).collect();
        if captures.len() == 1 {
            let fname = captures[0].at(1).unwrap();
            let size: isize =
                std::str::FromStr::from_str(
                    captures[0].at(2).unwrap())
                .unwrap();
            if ! stat.contains_key(fname) {
                stat.insert(fname.to_owned(), (size, size));
            } else {
                let pair = stat.get(fname).unwrap().clone();
                let (mut sum, mut max) = pair;
                sum += size;
                max = std::cmp::max(max, size);
                stat.insert(fname.to_owned(), (sum, max));
            }
        }
    };

    for it in stat {
        let (key, value) = it;
        let (sum, mut max) = value;
        if max == 0 {
            max = 1;
        }
        let downloads = sum as f64 / max as f64;
        println!("Key: {} downloads: {} (max size: {})",
                 key, downloads, max);
    }
}
