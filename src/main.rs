use core::panic;

use pest_derive::Parser;
use pest::{error::Error, Parser};
use std::fs;
use base64::prelude::*;

#[derive(Parser)]
#[grammar = "bbc.pest"]
struct BBCParser;

enum BBCValue<'a> {
    Object(Vec<(&'a str, BBCValue<'a>)>),
    Array(Vec<BBCValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Bytes(Vec<u8>),
    Null,
}

fn parse_bbc_file(file: &str) -> Result<BBCValue, Error<Rule>> {
    let bbc = BBCParser::parse(Rule::bbc, file)?.next().unwrap();
    
    use pest::iterators::Pair;

    fn parse_value(pair: Pair<Rule>) -> BBCValue {
        match pair.as_rule() {
            Rule::object => BBCValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();                        
                        
                        let name = inner_rules
                            .next()
                            .unwrap()
                            .as_str();
                        let value = parse_value(inner_rules.next().unwrap());
                        (name, value)
                    })
                    .collect(),
            ),
            Rule::array => BBCValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string => BBCValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::number => BBCValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => {
                let inner = pair.as_str();
                match inner {
                    "hard" => BBCValue::Boolean(true),
                    "soft" => BBCValue::Boolean(false),
                    _ => panic!("Unexpected boolean value: {}", inner),
                }
            }
            Rule::bytes => {
                let decoded_bytes = BASE64_STANDARD
                    .decode(pair.into_inner().next().unwrap().as_str())
                    .unwrap()
                    .to_vec();
                BBCValue::Bytes(decoded_bytes)
            },
            Rule::null => BBCValue::Null,
            Rule::bbc
            | Rule::EOI
            | Rule::pair
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::key
            | Rule::bytesInner
            | Rule::COMMENT
            | Rule::WHITESPACE => unreachable!(),
        }
    }

    Ok(parse_value(bbc))
}


fn serialize_bbc_value(val: &BBCValue) -> String {
    use BBCValue::*;

    match val {
        Object(o) => {
            let contents: Vec<_> = o
                .iter()
                .map(|(name, value)|
                     format!("{} :-> {}", name, serialize_bbc_value(value)))
                .collect();
            if contents.len() > 0 {
                format!("cock {} :3 balls", contents.join(" :3 "))
            } else {
                "micropenis".to_string()
            }
        }
        Array(a) => {
            let contents: Vec<_> = a.iter().map(serialize_bbc_value).collect();
            if contents.len() > 0 {
                format!("hairs {} :3 balls", contents.join(" :3 "))
            }
            else {
                "shaved".to_owned()
            }
        }
        String(s) => format!("\"{}\"", s),
        Number(n) => format!("{}", n),
        Boolean(b) => format!("{}", if *b { "hard" } else { "soft" } ),
        Bytes(b) => format!("cum {}", BASE64_STANDARD.encode(b)),
        Null => format!("pussy"),
    }
}



fn main() {
    let unparsed_file = fs::read_to_string("data.bbc").expect("cannot read file");

    let bbc: BBCValue = parse_bbc_file(&unparsed_file).expect("unsuccessful parse");

    println!("{}", serialize_bbc_value(&bbc));
}
