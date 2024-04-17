use core::panic;

use pest_derive::Parser;
use pest::{error::Error, Parser};
use std::fs;

#[derive(Parser)]
#[grammar = "bbc.pest"]
struct BBCParser;

enum BBCValue<'a> {
    Object(Vec<(&'a str, BBCValue<'a>)>),
    Array(Vec<BBCValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
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
                        println!("{:?}", inner_rules);
                        
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
            Rule::null => BBCValue::Null,
            Rule::bbc
            | Rule::EOI
            | Rule::pair
            | Rule::value
            | Rule::inner
            | Rule::char
            | Rule::key
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
                     format!("{}:->{}", name, serialize_bbc_value(value)))
                .collect();
            format!("cock {} balls", contents.join(":3"))
        }
        Array(a) => {
            let contents: Vec<_> = a.iter().map(serialize_bbc_value).collect();
            format!("hairs({})", contents.join(":3"))
        }
        String(s) => format!("\"{}\"", s),
        Number(n) => format!("{}", n),
        Boolean(b) => format!("{}", if *b { "hard" } else { "soft" } ),
        Null => format!("pussy"),
    }
}



fn main() {
    let unparsed_file = fs::read_to_string("data2.bbc").expect("cannot read file");

    let bbc: BBCValue = parse_bbc_file(&unparsed_file).expect("unsuccessful parse");

    println!("{}", serialize_bbc_value(&bbc));
}