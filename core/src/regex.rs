// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of substrate-desub.
//
// substrate-desub is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// substrate-desub is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-desub.  If not, see <http://www.gnu.org/licenses/>.

use super::{CommonTypes, RustTypeMarker};
use onig::{Regex, Region, SearchOptions};

#[derive(Debug, Clone, PartialEq, Eq)]
enum RegexSet {
    Array,
    Vec,
    Option,
    Result,
    Compact,
    Tuple,
}

impl RegexSet {
    /// Checks if string matches any of the patterns defined
    /// Returns none if it does not match
    fn get_type(s: &str) -> Option<RegexSet> {
        if rust_array_decl().is_match(s) {
            Some(RegexSet::Array)
        } else if rust_vec_decl().is_match(s) {
            Some(RegexSet::Vec)
        } else if rust_option_decl().is_match(s) {
            Some(RegexSet::Option)
        } else if rust_result_decl().is_match(s) {
            Some(RegexSet::Result)
        } else if rust_compact_decl().is_match(s) {
            Some(RegexSet::Compact)
        } else if rust_tuple_decl().is_match(s) {
            Some(RegexSet::Tuple)
        } else {
            None
        }
    }

    fn parse_type(&self, s: &str) -> Option<RustTypeMarker> {
        match self {
            RegexSet::Array => parse_regex_array(s),
            RegexSet::Vec => parse_regex_vec(s),
            RegexSet::Option => parse_regex_option(s),
            RegexSet::Result => parse_regex_result(s),
            RegexSet::Compact => parse_regex_compact(s),
            RegexSet::Tuple => parse_regex_tuple(s),
        }
    }
}

/// Match a rust array
pub fn rust_array_decl() -> Regex {
    // width of number and unsigned/signed are all in their own capture group
    // size of array is in the last capture group
    Regex::new(r"^\[ *?(?<type>[uif]{1})(?<bit8>8)?(?<bit16>16)?(?<bit32>32)?(?<bit64>64)?(?<bit128>128)?;\s*?(?<size>[\d]*) *?]")
        .expect("Regex expression invalid")
}

/// Match a rust vector
/// allowed to be nested within, or have other (ie Option<>) nested within
pub fn rust_vec_decl() -> Regex {
    Regex::new(r"^Vec<(?<type>[\w><,(): ]+)>")
        .expect("Regex expression should be infallible; qed")
}

/// Match a Rust Option
/// Allowed to be nested within another type, or have other (ie Vec<>) nested
pub fn rust_option_decl() -> Regex {
    Regex::new(r"^Option<(?<type>[\w><,(): ]+)>")
        .expect("Regex expression should be infallible; qed")
}

/// Match a rust result
pub fn rust_result_decl() -> Regex {
    Regex::new(r"^Result<(?<type>\(?[\w><,: ]*\)?), *(?<error>\(?[\w><, ]*\)?)>")
        .expect("Regex experession should be infallible; qed")
}

/// Match a parity-scale-codec Compact<T> type
pub fn rust_compact_decl() -> Regex {
    Regex::new(r"^Compact<(?<type>[\w><,(): ]+)>")
        .expect("Regex expression should be infallible; qed")
}

/// Match a Rust Generic Type Declaration
/// Excudes types Vec/Option/Compact from matches
pub fn rust_generic_decl() -> Regex {
    Regex::new(
        r"\b(?!(?:Vec|Option|Compact)\b)(?<outer_type>\w+)<(?<inner_type>[\w<>,:]+)>",
    )
    .expect("Regex expressions should be infallible; qed")
}

/// Transforms a prefixed generic type (EX: T::Moment)
/// into a non-prefixed type (T::Moment -> Moment)
pub fn remove_prefix<'a, S: Into<&'a str>>(s: S) -> Option<String> {
    let s: &str = s.into();

    let re = Regex::new(r"[\w]::([\w><]+)")
        .expect("Regex expressions should be infallible; qed");
    let caps = re.captures(s)?;
    caps.iter().nth(1)?.map(|s| s.to_string())
}

/// Only captures text within the tuples,
/// need to use 'Matches' (ie `find_iter`) iterator to get all matches
/// max tuple size is 64
///
/// # Note
/// this does not contain named capture groups
/// captures may be indexed like a tuple, via Captures<'a>::at(pos: usize)
/// except starting at 1, since 0 is always the entire match
pub fn rust_tuple_decl() -> Regex {
    Regex::new(
        [
            r#"^\(([\w><:\n]+)"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*,? *([\w><:\n]+)*"#,
            r#",? *([\w><:\n]+)*\)$"#,
        ]
        .join("")
        .as_str(),
    )
    .expect("Regex Expressions should be infallible; qed")
}

/// Parse a known match to the array regular expression
///
/// # Panics
///
/// TODO: Use errors instead of returning option
pub fn parse_regex_array(s: &str) -> Option<RustTypeMarker> {
    let re = rust_array_decl();
    if !re.is_match(s) {
        return None;
    }

    let mut region = Region::new();

    let (mut t, mut size, mut ty) = (None, None, None);
    if let Some(_position) = re.search_with_options(
        s,
        0,
        s.len(),
        SearchOptions::SEARCH_OPTION_NONE,
        Some(&mut region),
    ) {
        re.foreach_name(|name, groups| {
            for group in groups {
                // capture groups that don't represent type
                // ie if type is 'u8', 'u32' capture group
                // will be None
                if let Some(pos) = region.pos(*group as usize) {
                    match name {
                        "type" => {
                            // first thing matched
                            t = Some(&s[pos.0..pos.1])
                        }
                        "size" => {
                            // last thing matched
                            size = Some(&s[pos.0..pos.1])
                        }
                        "bit8" => ty = Some(8),
                        "bit16" => ty = Some(16),
                        "bit32" => ty = Some(32),
                        "bit64" => ty = Some(64),
                        "bit128" => ty = Some(128),
                        _ => panic!("Unhandled capture group"),
                    }
                } else {
                    continue;
                }
            }
            true
        });
    };
    let t = t?;
    let size = size?;
    let ty = ty?;

    let ty = match ty {
        8 => match t {
            "u" => RustTypeMarker::U8,
            "i" => RustTypeMarker::I8,
            "f" => panic!("type does not exist 'f8'"),
            _ => panic!("impossible match encountered"),
        },
        16 => match t {
            "u" => RustTypeMarker::U16,
            "i" => RustTypeMarker::I16,
            "f" => panic!("type does not exist 'f16'"),
            _ => panic!("impossible match encountered"),
        },
        32 => match t {
            "u" => RustTypeMarker::U32,
            "i" => RustTypeMarker::I32,
            "f" => RustTypeMarker::F32,
            _ => panic!("impossible match encountered"),
        },
        64 => match t {
            "u" => RustTypeMarker::U64,
            "i" => RustTypeMarker::I64,
            "f" => RustTypeMarker::F64,
            _ => panic!("impossible match encountered"),
        },
        128 => match t {
            "u" => RustTypeMarker::U128,
            "i" => RustTypeMarker::I128,
            "f" => panic!("type does not exist: 'f128'"),
            _ => panic!("impossible match encountered"),
        },
        _ => panic!("Couldn't determine bit-width of types in array"),
    };
    let ty = Box::new(ty);
    let size = size.parse::<usize>().expect("Should always be number");
    Some(RustTypeMarker::Array { size, ty })
}

fn parse_regex_vec(s: &str) -> Option<RustTypeMarker> {
    let re = rust_vec_decl();

    if !re.is_match(s) {
        return None;
    }

    let ty = re.captures(s)?.at(1)?;

    let ty = parse(ty).expect("Should always be some type; qed");
    Some(RustTypeMarker::Std(CommonTypes::Vec(Box::new(ty))))
}

fn parse_regex_option(s: &str) -> Option<RustTypeMarker> {
    let re = rust_option_decl();
    if !re.is_match(s) {
        return None;
    }
    let ty = re.captures(s)?.at(1)?;
    let ty = parse(ty).expect("Should always be some type; qed");
    Some(RustTypeMarker::Std(CommonTypes::Option(Box::new(ty))))
}

fn parse_regex_result(s: &str) -> Option<RustTypeMarker> {
    let re = rust_result_decl();
    if !re.is_match(s) {
        return None;
    }

    let ty = parse(re.captures(s)?.at(1)?).expect("Should always be some type; qed");
    let err = parse(re.captures(s)?.at(2)?).expect("Should always be some type; qed");
    Some(RustTypeMarker::Std(CommonTypes::Result(
        Box::new(ty),
        Box::new(err),
    )))
}

fn parse_regex_compact(s: &str) -> Option<RustTypeMarker> {
    let re = rust_compact_decl();
    if !re.is_match(s) {
        return None;
    }
    let ty = re.captures(s)?.at(1)?;

    let ty = parse(ty).expect("Should always be some type; qed");
    Some(RustTypeMarker::Std(CommonTypes::Compact(Box::new(ty))))
}

fn parse_regex_tuple(s: &str) -> Option<RustTypeMarker> {
    let re = rust_tuple_decl();
    if !re.is_match(s) {
        return None;
    }

    // skip the first element (entire match)
    let ty = re
        .captures(s)?
        .iter()
        .skip(1)
        .filter_map(|c| {
            if let Some(c) = c {
                Some(parse(c).expect("Must be a type; qed"))
            } else {
                None
            }
        })
        .collect::<Vec<RustTypeMarker>>();

    Some(RustTypeMarker::Tuple(ty))
}

/// recursively parses a regex set
/// returning a RustTypeMarker with all matched types
pub fn parse(s: &str) -> Option<RustTypeMarker> {
    match s {
        // match primitive types first
        "u8" => Some(RustTypeMarker::U8),
        "u16" => Some(RustTypeMarker::U16),
        "u32" => Some(RustTypeMarker::U32),
        "u64" => Some(RustTypeMarker::U64),
        "u128" => Some(RustTypeMarker::U128),
        "usize" => Some(RustTypeMarker::USize),

        "i8" => Some(RustTypeMarker::I8),
        "i16" => Some(RustTypeMarker::I16),
        "i32" => Some(RustTypeMarker::I32),
        "i64" => Some(RustTypeMarker::I64),
        "i128" => Some(RustTypeMarker::I128),
        "isize" => Some(RustTypeMarker::ISize),

        "f32" => Some(RustTypeMarker::F32),
        "f64" => Some(RustTypeMarker::F64),

        "bool" => Some(RustTypeMarker::Bool),
        "Null" => Some(RustTypeMarker::Null),

        _ => {
            // check if nested type
            if let Some(m) = RegexSet::get_type(s) {
                m.parse_type(s)
            } else {
                // if not a primitive, then a type pointer
                Some(RustTypeMarker::TypePointer(s.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_match_array_decl() {
        let re = rust_array_decl();
        assert!(re.is_match("[u8; 16]"));
        assert!(re.is_match("[u16; 16]"));
        assert!(re.is_match("[u32; 16]"));
        assert!(re.is_match("[u64; 16]"));
        assert!(re.is_match("[u128; 16]"));
        assert!(re.is_match("[i8; 16]"));
        assert!(re.is_match("[i16; 16]"));
        assert!(re.is_match("[i32; 16]"));
        assert!(re.is_match("[i64; 16]"));
        assert!(re.is_match("[i128; 16]"));
        assert!(re.is_match("[u16; 128]"));
        assert!(re.is_match("[u32; 64]"));
        assert!(re.is_match("[u64; 99999]"));
        assert!(re.is_match("[u128; 23]"));
    }

    #[test]
    fn should_seperate_args_in_capture_groups() {
        let re = rust_array_decl();

        let caps = re.captures("[u8; 16]").unwrap();
        assert_eq!(
            vec![
                Some("[u8; 16]"),
                Some("u"),
                Some("8"),
                None,
                None,
                None,
                None,
                Some("16")
            ],
            caps.iter().collect::<Vec<Option<&str>>>()
        );

        let caps = re.captures("[i8; 16]").unwrap();
        assert_eq!(
            vec![
                Some("[i8; 16]"),
                Some("i"),
                Some("8"),
                None,
                None,
                None,
                None,
                Some("16")
            ],
            caps.iter().collect::<Vec<Option<&str>>>()
        );

        let caps = re.captures("[u16; 16]").unwrap();
        assert_eq!(
            vec![
                Some("[u16; 16]"),
                Some("u"),
                None,
                Some("16"),
                None,
                None,
                None,
                Some("16")
            ],
            caps.iter().collect::<Vec<Option<&str>>>()
        );

        let caps = re.captures("[i16; 16]").unwrap();
        assert_eq!(
            vec![
                Some("[i16; 16]"),
                Some("i"),
                None,
                Some("16"),
                None,
                None,
                None,
                Some("16")
            ],
            caps.iter().collect::<Vec<Option<&str>>>()
        );

        let caps = re.captures("[u32; 16]").unwrap();
        assert_eq!(
            vec![
                Some("[u32; 16]"),
                Some("u"),
                None,
                None,
                Some("32"),
                None,
                None,
                Some("16")
            ],
            caps.iter().collect::<Vec<Option<&str>>>()
        );

        let caps = re.captures("[i32; 16]").unwrap();
        assert_eq!(
            vec![
                Some("[i32; 16]"),
                Some("i"),
                None,
                None,
                Some("32"),
                None,
                None,
                Some("16")
            ],
            caps.iter().collect::<Vec<Option<&str>>>()
        );

        let caps = re.captures("[u64; 16]").unwrap();
        assert_eq!(
            vec![
                Some("[u64; 16]"),
                Some("u"),
                None,
                None,
                None,
                Some("64"),
                None,
                Some("16")
            ],
            caps.iter().collect::<Vec<Option<&str>>>()
        );

        let caps = re.captures("[i64; 16]").unwrap();
        assert_eq!(
            vec![
                Some("[i64; 16]"),
                Some("i"),
                None,
                None,
                None,
                Some("64"),
                None,
                Some("16")
            ],
            caps.iter().collect::<Vec<Option<&str>>>()
        );

        let caps = re.captures("[u128; 16]").unwrap();
        assert_eq!(
            vec![
                Some("[u128; 16]"),
                Some("u"),
                None,
                None,
                None,
                None,
                Some("128"),
                Some("16")
            ],
            caps.iter().collect::<Vec<Option<&str>>>()
        );

        let caps = re.captures("[i128; 16]").unwrap();
        assert_eq!(
            vec![
                Some("[i128; 16]"),
                Some("i"),
                None,
                None,
                None,
                None,
                Some("128"),
                Some("16")
            ],
            caps.iter().collect::<Vec<Option<&str>>>()
        );

        let caps = re.captures("[f32; 16]").unwrap();
        assert_eq!(
            vec![
                Some("[f32; 16]"),
                Some("f"),
                None,
                None,
                Some("32"),
                None,
                None,
                Some("16")
            ],
            caps.iter().collect::<Vec<Option<&str>>>()
        );

        let caps = re.captures("[f64; 16]").unwrap();
        assert_eq!(
            vec![
                Some("[f64; 16]"),
                Some("f"),
                None,
                None,
                None,
                Some("64"),
                None,
                Some("16")
            ],
            caps.iter().collect::<Vec<Option<&str>>>()
        );

        let caps = re.captures("[i128; 9999]").unwrap();
        assert_eq!(
            vec![
                Some("[i128; 9999]"),
                Some("i"),
                None,
                None,
                None,
                None,
                Some("128"),
                Some("9999")
            ],
            caps.iter().collect::<Vec<Option<&str>>>()
        );

        let caps = re.captures("[u128; 9999]").unwrap();
        assert_eq!(
            vec![
                Some("[u128; 9999]"),
                Some("u"),
                None,
                None,
                None,
                None,
                Some("128"),
                Some("9999")
            ],
            caps.iter().collect::<Vec<Option<&str>>>()
        );
    }

    #[test]
    #[should_panic]
    fn should_not_parse_on_nonexistant_type() {
        parse_regex_array("[f16; 32]");
    }

    #[test]
    fn should_match_regex_array() {
        assert_eq!(
            parse_regex_array("[u8; 32]").unwrap(),
            RustTypeMarker::Array {
                size: 32,
                ty: Box::new(RustTypeMarker::U8)
            }
        );
        assert_eq!(
            parse_regex_array("[u16; 32]").unwrap(),
            RustTypeMarker::Array {
                size: 32,
                ty: Box::new(RustTypeMarker::U16)
            }
        );
        assert_eq!(
            parse_regex_array("[u32; 32]").unwrap(),
            RustTypeMarker::Array {
                size: 32,
                ty: Box::new(RustTypeMarker::U32)
            }
        );
        assert_eq!(
            parse_regex_array("[u64; 32]").unwrap(),
            RustTypeMarker::Array {
                size: 32,
                ty: Box::new(RustTypeMarker::U64)
            }
        );
        assert_eq!(
            parse_regex_array("[u128; 32]").unwrap(),
            RustTypeMarker::Array {
                size: 32,
                ty: Box::new(RustTypeMarker::U128)
            }
        );
        assert_eq!(
            parse_regex_array("[i8; 32]").unwrap(),
            RustTypeMarker::Array {
                size: 32,
                ty: Box::new(RustTypeMarker::I8)
            }
        );
        assert_eq!(
            parse_regex_array("[i16; 32]").unwrap(),
            RustTypeMarker::Array {
                size: 32,
                ty: Box::new(RustTypeMarker::I16)
            }
        );
        assert_eq!(
            parse_regex_array("[i32; 32]").unwrap(),
            RustTypeMarker::Array {
                size: 32,
                ty: Box::new(RustTypeMarker::I32)
            }
        );
        assert_eq!(
            parse_regex_array("[i64; 32]").unwrap(),
            RustTypeMarker::Array {
                size: 32,
                ty: Box::new(RustTypeMarker::I64)
            }
        );
        assert_eq!(
            parse_regex_array("[i128; 32]").unwrap(),
            RustTypeMarker::Array {
                size: 32,
                ty: Box::new(RustTypeMarker::I128)
            }
        );
        assert_eq!(
            parse_regex_array("[f32; 32]").unwrap(),
            RustTypeMarker::Array {
                size: 32,
                ty: Box::new(RustTypeMarker::F32)
            }
        );
        assert_eq!(
            parse_regex_array("[f64; 32]").unwrap(),
            RustTypeMarker::Array {
                size: 32,
                ty: Box::new(RustTypeMarker::F64)
            }
        );
        assert_eq!(
            parse_regex_array("[i128; 999999]").unwrap(),
            RustTypeMarker::Array {
                size: 999_999,
                ty: Box::new(RustTypeMarker::I128)
            }
        );
    }

    #[test]
    fn should_match_vecs() {
        let re = rust_vec_decl();
        assert!(re.is_match("Vec<RuntimeVersionApi>"));
        assert!(re.is_match("Vec<BlockNumber>"));
        assert!(re.is_match("Vec<SomeStruct>"));
    }

    #[test]
    fn should_get_type_of_vec() {
        let re = rust_vec_decl();
        let caps = re.captures("Vec<RuntimeVersionApi>").unwrap();
        // first capture group is always entire expression
        assert!(caps.at(1) == Some("RuntimeVersionApi"));
    }

    #[test]
    fn should_match_compact() {
        let re = rust_compact_decl();
        assert!(re.is_match("Compact<RuntimeVersionApi>"));
        assert!(re.is_match("Compact<BlockNumber>"));
        assert!(re.is_match("Compact<SomeStruct>"));
    }

    #[test]
    fn should_get_type_of_compact() {
        let re = rust_compact_decl();
        let caps = re.captures("Compact<RuntimeVersionApi>").unwrap();
        assert!(caps.at(1) == Some("RuntimeVersionApi"));
    }

    #[test]
    fn should_match_options() {
        let re = rust_option_decl();
        assert!(re.is_match("Option<RuntimeVersionApi>"));
        assert!(re.is_match("Option<BlockNumber>"));
        assert!(re.is_match("Option<SomeStruct>"));
        assert!(re.is_match("Option<Vec<SomeStruct>>"));
    }

    #[test]
    fn should_get_type_of_option() {
        let re = rust_option_decl();
        let caps = re.captures("Option<RuntimeVersionApi>").unwrap();
        // first capture group is always entire expression
        assert!(caps.at(1) == Some("RuntimeVersionApi"));

        let re = rust_option_decl();
        let caps = re.captures("Option<Vec<RuntimeVersionApi>>").unwrap();
        assert!(caps.at(1) == Some("Vec<RuntimeVersionApi>"));
    }

    #[test]
    fn should_match_results() {
        let re = rust_result_decl();
        assert!(re.is_match("Result<RuntimeVersionApi, DispatchError>"));
        assert!(re.is_match("Result<(), FooError>"));
        assert!(re.is_match("Result<Foo, (WeirdError, PogError)>"));
        assert!(re.is_match("Result<Vec<WeirdType>, FooError>"));
        assert!(
            re.is_match("Result<(Vec<WeirdType>, Weird), (FooError, WeirdErrorFormat)>")
        );
    }

    #[test]
    fn should_get_type_of_result() {
        let re = rust_result_decl();
        let caps = re.captures("Result<Foo, Bar>").unwrap();

        assert!(caps.at(1) == Some("Foo"));
        assert!(caps.at(2) == Some("Bar"));

        let caps = re.captures("Result<(Foo, Zoo), (Bar, Car)>").unwrap();
        dbg!(&caps.at(1));
        assert!(caps.at(1) == Some("(Foo, Zoo)"));
        assert!(caps.at(2) == Some("(Bar, Car)"));
    }

    #[test]
    fn should_match_arbitrary_types() {
        let re = rust_generic_decl();
        assert!(re.is_match("GenericOuterType<GenericInnerType>"));
        assert!(re.is_match("GenericOutT<GenericOutInT<InnerT>>"));
        assert!(!re.is_match("Vec<Foo>"));
        assert!(!re.is_match("Option<Foo>"));
        assert!(!re.is_match("Compact<Foo>"));
    }

    #[test]
    fn should_get_arbitrary_type() {
        let re = rust_generic_decl();
        let caps = re.captures("GenericOuterType<GenericInnerType>").unwrap();
        assert_eq!(
            vec![
                Some("GenericOuterType<GenericInnerType>"),
                Some("GenericOuterType"),
                Some("GenericInnerType")
            ],
            caps.iter().collect::<Vec<Option<&str>>>()
        );
    }

    #[test]
    fn should_match_tuples() {
        let re = rust_tuple_decl();
        assert!(re.is_match("(StorageKey, Option<StorageData>)"));
        assert!(re.is_match("(ApiKey, u32)"));
        assert!(re.is_match("(u32,ApiKey,AnotherType)"));
        // assert the upper match limit
        assert!(re.is_match(["(StorageKey, Option<StorageData>, Foo, Bar, Aoo, Raw, Car, Dar, Eoo, Foo, Goo, Foo, Foo, Foo, Foo, Foo,",
                             "Hoo, Ioo, Joo, Koo, Loo, Moo, Noo, Ooo, Poo, Qoo, Roo, Soo, Too, Uoo, Voo, Xoo,",
                             "Hoo, Ioo, Joo, Koo, Loo, Moo, Noo, Ooo, Poo, Qoo, Roo, Soo, Too, Uoo, Voo, Xoo,",
                             "Hoo, Ioo, Joo, Koo, Loo, Moo, Noo, Ooo, Poo, Qoo, Roo, Soo, Too, Uoo, Voo, Xoo)"
                             ].join("").as_str()));
        assert!(re.is_match(
            ["(StorageKey, Option<StorageData>, Foo,
        Bar, Aoo)"]
            .join("")
            .as_str()
        ));
    }

    #[test]
    fn should_get_types_in_tuple() {
        let re = rust_tuple_decl();
        let match_str = "(StorageKey, Option<StorageData>)";
        let caps = re.captures(match_str).unwrap();
        assert_eq!(
            vec![
                Some("(StorageKey, Option<StorageData>)"),
                Some("StorageKey"),
                Some("Option<StorageData>"),
            ],
            caps.iter()
                .filter(|c| c.is_some())
                .collect::<Vec<Option<&str>>>()
        );
    }

    #[test]
    fn should_correctly_indicate_type() {
        assert_eq!(RegexSet::get_type("[   u8;   32 ]"), Some(RegexSet::Array));
        assert_eq!(RegexSet::get_type("Vec<Foo>"), Some(RegexSet::Vec));
        assert_eq!(RegexSet::get_type("Option<Foo>"), Some(RegexSet::Option));
        assert_eq!(RegexSet::get_type("Compact<Foo>"), Some(RegexSet::Compact));
        assert_eq!(
            RegexSet::get_type("(StorageKey, Foo<Bar>)"),
            Some(RegexSet::Tuple)
        );

        assert_eq!(RegexSet::get_type("Vec<Option<Foo>>"), Some(RegexSet::Vec));
        assert_eq!(
            RegexSet::get_type("Option<Vec<Hello>>"),
            Some(RegexSet::Option)
        );
    }

    #[test]
    fn should_parse_type() {
        assert_eq!(parse("u8").unwrap(), RustTypeMarker::U8);
        assert_eq!(parse("u16").unwrap(), RustTypeMarker::U16);
        assert_eq!(parse("u32").unwrap(), RustTypeMarker::U32);
        assert_eq!(parse("u64").unwrap(), RustTypeMarker::U64);
        assert_eq!(parse("u128").unwrap(), RustTypeMarker::U128);
        assert_eq!(parse("usize").unwrap(), RustTypeMarker::USize);

        assert_eq!(parse("i8").unwrap(), RustTypeMarker::I8);
        assert_eq!(parse("i16").unwrap(), RustTypeMarker::I16);
        assert_eq!(parse("i32").unwrap(), RustTypeMarker::I32);
        assert_eq!(parse("i64").unwrap(), RustTypeMarker::I64);
        assert_eq!(parse("i128").unwrap(), RustTypeMarker::I128);
        assert_eq!(parse("isize").unwrap(), RustTypeMarker::ISize);

        assert_eq!(parse("f32").unwrap(), RustTypeMarker::F32);
        assert_eq!(parse("f64").unwrap(), RustTypeMarker::F64);

        assert_eq!(parse("bool").unwrap(), RustTypeMarker::Bool);
        assert_eq!(parse("Null").unwrap(), RustTypeMarker::Null);

        assert_eq!(
            parse("Option<Foo>").unwrap(),
            RustTypeMarker::Std(CommonTypes::Option(Box::new(
                RustTypeMarker::TypePointer("Foo".to_string())
            )))
        );
        assert_eq!(
            parse("Compact<Vec<Option<Foo>>>").unwrap(),
            RustTypeMarker::Std(CommonTypes::Compact(Box::new(RustTypeMarker::Std(
                CommonTypes::Vec(Box::new(RustTypeMarker::Std(CommonTypes::Option(
                    Box::new(RustTypeMarker::TypePointer("Foo".to_string()))
                ))))
            ))))
        );

        assert_eq!(
            parse("Compact<Vec<(Foo, Bar, u8)>>").unwrap(),
            RustTypeMarker::Std(CommonTypes::Compact(Box::new(RustTypeMarker::Std(
                CommonTypes::Vec(Box::new(RustTypeMarker::Tuple(vec![
                    RustTypeMarker::TypePointer("Foo".to_string()),
                    RustTypeMarker::TypePointer("Bar".to_string()),
                    RustTypeMarker::U8,
                ])))
            ))))
        );

        assert_eq!(
            parse("Option<Vec<(Foo, Bar, u8)>>").unwrap(),
            RustTypeMarker::Std(CommonTypes::Option(Box::new(RustTypeMarker::Std(
                CommonTypes::Vec(Box::new(RustTypeMarker::Tuple(vec![
                    RustTypeMarker::TypePointer("Foo".to_string()),
                    RustTypeMarker::TypePointer("Bar".to_string()),
                    RustTypeMarker::U8,
                ])))
            ))))
        );

        assert_eq!(
            parse("Vec<Vec<(Foo, Bar, T::SystemMarker)>>").unwrap(),
            RustTypeMarker::Std(CommonTypes::Vec(Box::new(RustTypeMarker::Std(
                CommonTypes::Vec(Box::new(RustTypeMarker::Tuple(vec![
                    RustTypeMarker::TypePointer("Foo".to_string()),
                    RustTypeMarker::TypePointer("Bar".to_string()),
                    RustTypeMarker::TypePointer("T::SystemMarker".to_string()),
                ])))
            ))))
        );
    }

    #[test]
    fn should_remove_prefix() {
        assert_eq!(remove_prefix("T::Moment").unwrap(), "Moment");
        assert_eq!(
            remove_prefix("T::Generic<Runtime>").unwrap(),
            "Generic<Runtime>"
        );
    }
}
