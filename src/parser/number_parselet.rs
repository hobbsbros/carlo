//! Defines a numerical value parselet.

use std::collections::HashMap;

use super::{
    Error,
    Expression,
    Parser,
    PrefixParselet,
    Token,
    TokenClass,
    Tokenstream,
};

const UNITS: [(&str, (f64, f64, f64, f64)); 5] = [
    ("g", (0.001, 1.0, 0.0, 0.0)),
    ("m", (1.0, 0.0, 1.0, 0.0)),
    ("s", (1.0, 0.0, 0.0, 1.0)),
    ("N", (1.0, 1.0, 1.0, -2.0)),
    ("Pa", (1.0, 1.0, -1.0, -2.0)),
];

const PREFIXES: [(char, f64); 7] = [
    ('n', 1E-9),
    ('u', 1E-6),
    ('m', 1E-3),
    ('c', 1E-2),
    ('k', 1E+3),
    ('M', 1E+6),
    ('G', 1E+9),
];

/// Checks if a string is in the form of a unit.
fn check_unit(input: &str) -> bool {
    for (unit, _value) in UNITS {
        if input.starts_with(unit) {
            return true;
        }
    }

    for (prefix, _value) in PREFIXES {
        if input.starts_with(prefix) {
            return true;
        }
    }

    return false;
}

/// Gets the alpha and numeric parts of a unit.
fn split_string(input: &str) -> (String, f64) {
    let mut alpha = String::new();
    let mut numeric = String::new();
    let mut sign = 1.0;

    for c in input.chars() {
        if ('A'..='Z').contains(&c) || ('a'..='z').contains(&c) {
            alpha.push(c);
        } else if ('0'..='9').contains(&c) {
            numeric.push(c);
        } else if c == '_' {
            sign = -1.0;
        }
    }

    let n = if numeric.len() == 0 {
        1.0
    } else {
        match str::parse::<f64>(&numeric) {
            Ok (f) => sign * f,
            Err (_) => Error::CouldNotParseNumber (&numeric).throw(),
        }
    };

    (alpha, n)
}

/// Parses a string into a unit.
fn parse_unit(mut input: &str) -> (f64, f64, f64, f64) {
    let prefixes = HashMap::from(PREFIXES);
    let units = HashMap::from(UNITS);

    // Strips the prefix, if it exists
    let (a, _) = split_string(&input);
    let mut multiplier = if a.len() > 1 {
        match prefixes.get(&input.chars().nth(0).unwrap()) {
            Some (p) => {
                let mut inputchars = input.chars();
                inputchars.next();
                input = inputchars.as_str();
                *p
            },
            None => 1.0,
        }
    } else {
        1.0
    };

    let (alpha, exp) = split_string(&input);

    let (mult, mut kg, mut m, mut s) = match units.get(&*alpha) {
        Some (u) => (u.0, u.1, u.2, u.3),
        None => Error::CouldNotParseNumber (&input).throw(),
    };

    // Correct the multiplier for conversions (e.g. kg <-> g)
    multiplier *= mult;
    multiplier = multiplier.powf(exp);

    kg *= exp;
    m *= exp;
    s *= exp;

    (multiplier, kg, m, s)
}

pub struct NumberParselet {}

impl PrefixParselet for NumberParselet {
    fn parse(&self, tokenstream: &mut Tokenstream, _parser: &Parser, mut token: Token, _nesting: usize) -> Expression {
        use Expression::*;

        // Parse negative sign
        let sign = if token.check(TokenClass::Minus) {
            token = tokenstream.next_unwrap();
            -1.0
        } else {
            1.0
        };

        let mut multiplier = 1.0;
        let mut kilogram = 0.0;
        let mut meter = 0.0;
        let mut second = 0.0;

        while let Some(t) = tokenstream.peek() {
            if t.check(TokenClass::Identifier) && check_unit(&t.value) {
                tokenstream.next();
                
                let (mult, kg, m, s) = parse_unit(&t.value);

                multiplier *= mult;
                kilogram += kg;
                meter += m;
                second += s;
            } else {
                break;
            }
        }

        match str::parse::<f64>(&token.value) {
            Ok (f) => Float {
                value: multiplier * sign * f,
                kg: kilogram,
                m: meter,
                s: second,
            },
            _ => Error::CouldNotParseNumber (&token.value).throw(),
        }
    }
}