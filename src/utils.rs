// source of bracket handling code:
// https://codereview.stackexchange.com/questions/253279/matching-brackets-in-rust
use periodic_table_on_an_enum::Element;
use std::collections::HashMap;
enum Bracket {
    Open(char),
    Close(char),
}

impl Bracket {
    fn from_char(c: char) -> Option<Bracket> {
        match c {
            '[' | '(' => Some(Bracket::Open(c)),
            ']' => Some(Bracket::Close('[')),
            ')' => Some(Bracket::Close('(')),
            _ => None,
        }
    }
}

pub fn check_brackets(string: &str) -> bool {
    let mut brackets: Vec<char> = vec![];
    for c in string.chars() {
        match Bracket::from_char(c) {
            Some(Bracket::Open(char_bracket)) => {
                brackets.push(char_bracket);
            }
            Some(Bracket::Close(char_close_bracket)) => {
                if brackets.pop() != Some(char_close_bracket) {
                    return false;
                }
            }
            _ => {}
        }
    }
    brackets.is_empty()
}
pub fn gc(s: &str, i: usize) -> char {
    // stands for get_char
    s.chars().nth(i).unwrap_or_default()
}
pub fn parse_num(s: &str, i: usize) -> u32 {
    match gc(s, i).is_numeric() {
        true => {
            let mut j = i;
            loop {
                if !gc(s, j + 1).is_numeric() || j + 1 >= s.len() {
                    break;
                }
                j = j + 1;
            }
            s[i..j + 1].trim().parse().unwrap_or(1)
        }
        false => 1,
    }
}
// fn debug_print_map_content(){
//     print!("cur:{},", gc(s, i));
//     for (key, val) in &tmp_map {
//         print!("{:?}:{},", key, val);
//     }
//     println!();
// }

pub static ABOUT_MSG: &str = "Lightmol v0.1 by aisuneko\npowered by Rust and fltk-rs";
pub fn init_schooldata() -> HashMap<Element, f32> {
    HashMap::from([
        (Element::Carbon, 12.0),
        (Element::Hydrogen, 1.0),
        (Element::Oxygen, 16.0),
        (Element::Nitrogen, 14.0),
        (Element::Phosphorus, 31.0),
        (Element::Sulfur, 32.0),
        (Element::Potassium, 39.0),
        (Element::Iodine, 127.0),
        (Element::Barium, 137.0),
        (Element::Gold, 197.0),
        (Element::Calcium, 40.0),
        (Element::Chlorine, 35.5),
        (Element::Sodium, 23.0),
        (Element::Magnesium, 24.0),
        (Element::Aluminum, 27.0),
        (Element::Silicon, 28.0),
        (Element::Manganese, 55.0),
        (Element::Iron, 56.0),
        (Element::Copper, 64.0),
        (Element::Zinc, 65.0),
        (Element::Silver, 108.0),
        (Element::Mercury, 201.0),
    ])
}
