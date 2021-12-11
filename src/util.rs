/*
    Advent of Code 2021
    Caleb Stanford
    Utilities
    (copied from AoC 2020)
*/

use std::convert::TryInto;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

/*
    Parsing
*/

// Convert a file to a vector of its lines
pub fn file_to_vec_parsed<T>(filepath: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap().parse().unwrap()).collect()
}

// Simple string version
pub fn file_to_vec(filepath: &str) -> Vec<String> {
    file_to_vec_parsed(filepath)
}

// Version that is terminated with an empty line ("")
pub fn file_to_vec_el(filepath: &str) -> Vec<String> {
    let mut v = file_to_vec(filepath);
    v.push("".to_owned());
    v
}

// Split by whitespace, and variants
pub fn split(s: &str) -> Vec<String> {
    s.split_whitespace().map(|x| x.to_string()).collect()
}
pub fn split_parsed<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    s.split_whitespace()
        .map(|x| {
            x.parse().unwrap_or_else(|err| {
                panic!("Failed to parse: {} ({:?})", x, err)
            })
        })
        .collect()
}
pub fn split_by_parsed<T>(s: &str, ch: char) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    s.split(ch)
        .map(|x| {
            x.parse().unwrap_or_else(|err| {
                panic!("Failed to parse: {} ({:?})", x, err)
            })
        })
        .collect()
}
pub fn split_array<const N: usize>(s: &str) -> [String; N] {
    split(s).try_into().unwrap_or_else(|err| {
        panic!("String is not {} ws-separated parts: {} ({:?})", N, s, err)
    })
}
pub fn split_array_parsed<T, const N: usize>(s: &str) -> [T; N]
where
    T: Debug + FromStr,
    <T as FromStr>::Err: Debug,
{
    split_parsed(s).try_into().unwrap_or_else(|err| {
        panic!("String is not {} ws-separated parts: {} ({:?})", N, s, err)
    })
}
pub fn split_pair(s: &str) -> (String, String) {
    let pair: [String; 2] = split_array(s);
    (pair[0].clone(), pair[1].clone())
}
pub fn split_pair_parsed<T1, T2>(s: &str) -> (T1, T2)
where
    T1: FromStr,
    T2: FromStr,
    <T1 as FromStr>::Err: Debug,
    <T2 as FromStr>::Err: Debug,
{
    let pair: [String; 2] = split_array(s);
    let t1 = pair[0].parse().unwrap_or_else(|err| {
        panic!("Failed to parse: {} ({:?})", pair[0], err)
    });
    let t2 = pair[1].parse().unwrap_or_else(|err| {
        panic!("Failed to parse: {} ({:?})", pair[0], err)
    });
    (t1, t2)
}

// Bool-char conversion
pub fn bool_to_char(b: bool) -> char {
    if b {
        '1'
    } else {
        '0'
    }
}
pub fn char_to_bool(ch: char) -> bool {
    match ch {
        '1' => true,
        '0' => false,
        _ => panic!("cannot convert to bool: {}", ch),
    }
}
