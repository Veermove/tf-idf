#![allow(dead_code)]

use std::{iter::Peekable, str::Chars, collections::HashMap, error::Error};

#[derive(Debug, PartialEq, Clone)]
enum JsonTokens {
    OpenOb,
    CloseOb,
    StringVal(String),
    NumVal(i64),
    DecVal(f64),
    BoolVal(bool),
    Null,
    OpArr,
    CloseArr,
    Separator,
    Ddot,
}

#[derive(Debug)]
pub enum JsonValue {
    StringValue(String),
    IntegerValue(i64),
    DecimalValue(f64),
    BooleanValue(bool),
    ObjectValue(HashMap<String, JsonValue>),
    Null,
    ArrayValue(Vec<JsonValue>)
}

pub fn parse_json(json_string: String) -> Option<JsonValue> {
    let tokens = tokenize_json(json_string).ok()?;
    return parse_object(&tokens, 0).map(|s| s.0);
}

fn parse_pair(tokens_given: &Vec<JsonTokens>, index: usize) -> Option<(String, JsonValue, usize)> {
    if let JsonTokens::StringVal(n) = &tokens_given[index] {
        if let JsonTokens::Ddot = tokens_given[index + 1] {
            let (val, n_index) = parse_value(tokens_given, index + 2)?;
            return Some((n.to_owned(), val, n_index));
        }
    }

    return None;
}

fn parse_value(tokens_given: &Vec<JsonTokens>, index: usize)  -> Option<(JsonValue, usize)> {
    return parse_string(tokens_given, index)
        .or_else(|| parse_decimal(tokens_given, index))
        .or_else(|| parse_null(tokens_given, index))
        .or_else(|| parse_interger(tokens_given, index))
        .or_else(|| parse_boolean(tokens_given, index))
        .or_else(|| parse_array(tokens_given, index))
        .or_else(|| parse_object(tokens_given, index));
}

fn parse_object(tokens_given: &Vec<JsonTokens>, index: usize) -> Option<(JsonValue, usize)> {
    if let JsonTokens::OpenOb = tokens_given[index] {
        let mut n_index = index + 1;
        let mut ress = HashMap::new();
        if tokens_given[n_index] != JsonTokens::CloseOb {
            loop {
                let (name, val, ret_index) = parse_pair(tokens_given, n_index)?;
                n_index = ret_index;
                ress.insert(name, val);
                if tokens_given[n_index] == JsonTokens::CloseOb {
                    break;
                } else if tokens_given[n_index] == JsonTokens::Separator {
                    n_index += 1;
                    continue;
                } else {
                    return None;
                }
            }
        }
        return Some((JsonValue::ObjectValue(ress), n_index + 1));
    }
    return None;
}

fn parse_array(tokens_given: &Vec<JsonTokens>, index: usize)  -> Option<(JsonValue, usize)> {
    if let JsonTokens::OpArr = tokens_given[index] {
        let mut n_index = index + 1;
        let mut ress = Vec::new();
        if tokens_given[n_index] != JsonTokens::CloseArr {
            loop {
                let (ret, ret_index) = parse_value(tokens_given, n_index)?;
                n_index = ret_index;
                ress.push(ret);
                if tokens_given[n_index] == JsonTokens::CloseArr {
                    break;
                } else if tokens_given[n_index] == JsonTokens::Separator {
                    n_index += 1;
                    continue;
                } else {
                    return None;
                }
            }
        }
        return Some((JsonValue::ArrayValue(ress), n_index + 1));
    }
    return None;
}

fn parse_string(tokens_given: &Vec<JsonTokens>, index: usize) -> Option<(JsonValue, usize)> {
    if let JsonTokens::StringVal(v) = &tokens_given[index] {
        return Some((JsonValue::StringValue(v.to_owned()), index + 1));
    }

    return None;
}

fn parse_decimal(tokens_given: &Vec<JsonTokens>, index: usize) -> Option<(JsonValue, usize)> {
    if let JsonTokens::DecVal(v) = tokens_given[index] {
        return Some((JsonValue::DecimalValue(v), index + 1));
    }
    return None;
}

fn parse_interger(tokens_given: &Vec<JsonTokens>, index: usize) -> Option<(JsonValue, usize)> {
    if let JsonTokens::NumVal(v) = tokens_given[index] {
        return Some((JsonValue::IntegerValue(v), index + 1));
    }
    return None;
}

fn parse_null(tokens_given: &Vec<JsonTokens>, index: usize) -> Option<(JsonValue, usize)> {
    if let JsonTokens::Null = tokens_given[index] {
        return Some((JsonValue::Null, index + 1));
    }
    return None;
}

fn parse_boolean(tokens_given: &Vec<JsonTokens>, index: usize) -> Option<(JsonValue, usize)> {
    if let JsonTokens::BoolVal(s) = tokens_given[index] {
        return Some((JsonValue::BooleanValue(s), index + 1));
    }
    return None;
}

fn tokenize_json(json_string: String) -> Result<Vec<JsonTokens>, String> {
    let mut j_itr = json_string.chars().peekable();
    let mut res = Vec::new();

    loop {
        res.push(
            match j_itr.next() {
                Some('{') => JsonTokens::OpenOb,
                Some('}') => JsonTokens::CloseOb,
                Some('[') => JsonTokens::OpArr,
                Some(']') => JsonTokens::CloseArr,
                Some(':') => JsonTokens::Ddot,
                Some(',') => JsonTokens::Separator,
                Some('"') => parse_str(&mut j_itr)?,
                Some(c ) if c == 't' || c == 'f' || c == 'n' => parse_bool(c, &mut j_itr)?,
                Some(d) if d.is_ascii_digit() || d == '-' => parse_number(d, &mut j_itr)?,
                Some(' ')
                | Some('\n')
                | Some('\t') => continue,
                None => break,
                _c=> continue,
                    // unreachable!("{}", format!("Failed on char: {:?}", _c)),
            }
        )
    }

    return Ok(res);
}

fn parse_number(start: char, supplier: &mut Peekable<Chars>) -> Result<JsonTokens, String> {
    let mut res = Vec::new();
    loop {
        let c = supplier.peek();
        if let Some(ch) = c {
            if !ch.is_ascii_digit() && *ch != '.' {
                break;
            }
            res.push(supplier.next().unwrap())
        } else {
            break;
        }
    }

    let string_num = vec![start].iter().chain(res.iter()).collect::<String>();

    if let Ok(num) = string_num.parse::<i64>() {
        return Ok(JsonTokens::NumVal(num));
    } else if let Ok(num) = string_num.parse::<f64>() {
        return Ok(JsonTokens::DecVal(num));
    }

    return Err("Failed to parse json number".to_owned());

}

fn parse_str(supplier: &mut Peekable<Chars>) -> Result<JsonTokens, String> {
    let mut res = Vec::new();
    let mut prev = '"';
    for c in supplier {
        if c == '"' && prev != '\\' {
            prev = c;
            break;
        }
        res.push(c as u8);
        prev = c;
    }

    if prev == '"' {
        return Ok(JsonTokens::StringVal(unescape(res.as_slice()).unwrap()));
    }

    return Err("Unfinished json string literal".to_owned());
}

fn unescape(s: &[u8]) -> Result<String, Box<dyn Error>> {
    let mut output = Vec::new();
    let mut i = 0;

    while i < s.len() {
        match s[i] {
            b'\\' => {
                i += 1;
                match s[i] {
                    b'u' => {
                        let num = u8::from_str_radix(std::str::from_utf8(&s[i+1..][..4])?, 16)?;
                        output.push(num);
                        i += 4;
                    }
                    byte => output.push(byte),
                }
            },
            byte => output.push(byte),
        }
        i += 1;
    }
    Ok(String::from_utf8(output)?)
}


fn parse_bool(started: char, supplier: &mut Peekable<Chars>) -> Result<JsonTokens, String> {
    let result = vec![supplier.next(), supplier.next(), supplier.next()]
        .iter()
        .filter_map(|c| *c)
        .collect::<String>();

    if started == 't' && result == "rue" {// t rue
        return Ok(JsonTokens::BoolVal(true));
    } else if started == 'n' && result == "ull" {// t rue
        return Ok(JsonTokens::Null);
    } else if started == 'f' && result == "als" { //f als e
        return if let Some('e') = supplier.next() {
            Ok(JsonTokens::BoolVal(false))
        } else {
            Err("Unfinished json bool literal".to_owned())
        }
    }

    return Err("Unfinished json bool literal".to_owned());
}
