// algorithm inspired by
// https://leetcode.com/problems/number-of-atoms/discuss/214835/0ms-parser-in-Rust.

use crate::utils::*;
use periodic_table_on_an_enum::Element;
use std::collections::HashMap;

pub fn parse(s: &str, is_school: bool) -> f32 {
    let mut vec: Vec<HashMap<Element, u32>> = Vec::new();
    vec.push(HashMap::new());
    let mut i = 0;
    let mut tmp_map: HashMap<Element, u32> = HashMap::new();
    if !check_brackets(s) || s.is_empty() {
        return 0.0;
    }
    let left_brackets = ['(', '['];
    let right_brackets = [')', ']'];
    loop {
        if i >= s.len() {
            break;
        }
        let cur = gc(s, i);
        if left_brackets.contains(&cur) {
            vec.push(tmp_map);
            tmp_map = HashMap::new();
        } else if right_brackets.contains(&cur) {
            for (_, val) in tmp_map.iter_mut() {
                *val *= parse_num(s, i + 1);
            }
            tmp_map = match vec.pop() {
                Some(mut top) => {
                    for (key, val) in tmp_map.iter() {
                        *top.entry(*key).or_insert(0) += *val;
                    }
                    top
                }
                None => tmp_map,
            };
        } else if cur == '-' && &s[s.len() - 3..] == "H2O" {
            let num = parse_num(s, i + 1);
            *tmp_map
                .entry(Element::from_symbol("H").unwrap())
                .or_insert(0) += num * 2;
            *tmp_map
                .entry(Element::from_symbol("O").unwrap())
                .or_insert(0) += num;
            break;
        } else {
            if cur.is_alphabetic() && cur.is_uppercase() {
                // is symbol
                let next = gc(s, i + 1);
                let element;
                if next.is_alphabetic() && next.is_lowercase() {
                    element = Element::from_symbol(&s[i..i + 2]);
                    i += 1;
                } else {
                    element = Element::from_symbol(&s[i..i + 1])
                }
                if element.is_some() {
                    let entry = tmp_map.entry(element.unwrap()).or_insert(0);
                    *entry += parse_num(s, i + 1);
                }
            }
        }
        i = i + 1;
    }
    let mut sum = 0.0;
    let elements: Vec<_> = tmp_map.iter().collect();
    if is_school {
        let school_data = init_schooldata();
        for (key, val) in elements {
            sum += (*val as f32)
                * match school_data.get(key) {
                    Some(&num) => num,
                    None => 0.0,
                }
        }
    } else {
        for (key, val) in elements {
            // println!("{:?}:{},", key, val);
            sum += key.get_atomic_mass() * (*val as f32);
        }
    }
    (sum * 100.0).round() / 100.0
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(142.05, parse("Na2SO4", false));
        assert_eq!(44.01, parse("CO2", false));
        assert_eq!(98.08, parse("H2SO4", false));
        assert_eq!(149.24, parse("C10H15N", false));
        assert_eq!(34.09, parse("H2S", false)); //?
        assert_eq!(35.05, parse("NH4OH", false)); //?
        assert_eq!(88.11, parse("CH3COOC2H5", false));
        assert_eq!(279.19, parse("[C(NH2)3]Al(SO4)2", false));
        assert_eq!(3123.32, parse("CuSO4-5H2O", false));
        assert_eq!(0.0, parse("flka321", false));
    }
}
