//! Defines a numerical value parselet.

use std::collections::HashMap;

use super::{
    Error,
    Expression,
    Parser,
    PREFIXES,
    PrefixParselet,
    Token,
    TokenClass,
    Tokenstream,
    UNITS,
};

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
            Err (_) => {
                Error::CouldNotParseNumber (&numeric).warn();
                return (String::new(), 0.0);
            },
        }
    };

    (alpha, n)
}

/// Parses a string into a unit.
fn parse_unit(mut input: &str) -> (f64, f64, f64, f64, f64, f64, f64) {
    let prefixes = HashMap::from(PREFIXES);
    let units = HashMap::from(UNITS);

    // Strips the prefix, if it exists
    let (al, _) = split_string(&input);
    let mut multiplier = if let Some(_) = units.get(&*al) {
        1.0
    } else {
        match prefixes.get(&input.chars().nth(0).unwrap()) {
            Some (p) => {
                let mut inputchars = input.chars();
                inputchars.next();
                input = inputchars.as_str();
                *p
            },
            None => 1.0,
        }
    };

    let (alpha, exp) = split_string(&input);

    let (
        mult,
        mut kg,
        mut m,
        mut s,
        mut a,
        mut k,
        mut mol,
    ) = match units.get(&*alpha) {
        Some (u) => *u,
        None => {
            Error::CouldNotParseNumber (&input).warn();
            return (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        },
    };

    // Correct the multiplier for conversions (e.g. kg <-> g)
    multiplier *= mult;
    multiplier = multiplier.powf(exp);

    kg *= exp;
    m *= exp;
    s *= exp;
    a *= exp;
    k *= exp;
    mol *= exp;

    (multiplier, kg, m, s, a, k, mol)
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
        let mut amp = 0.0;
        let mut kelvin = 0.0;
        let mut mole = 0.0;

        while let Some(t) = tokenstream.peek() {
            if t.check(TokenClass::Identifier) && check_unit(&t.value) {
                tokenstream.next();
                
                let (mult, kg, m, s, a, k, mol) = parse_unit(&t.value);

                multiplier *= mult;
                kilogram += kg;
                meter += m;
                second += s;
                amp += a;
                kelvin += k;
                mole += mol;
            } else {
                break;
            }
        }

        let value = match str::parse::<f64>(&token.value) {
            Ok (f) => f,
            _ => {
                Error::CouldNotParseNumber (&token.value).warn();
                0.0
            },
        };

        Float {
            value: multiplier * sign * value,
            kg: kilogram,
            m: meter,
            s: second,
            a: amp,
            k: kelvin,
            mol: mole,
        }
    }
}