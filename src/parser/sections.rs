use crate::parser::types::parse_function_type;
use crate::parser::values::{parse_name, parse_u32, parse_vector};
use crate::{Custom, FunctionType, ModuleSection};
use nom::bytes::complete::{tag, take};
use nom::combinator::{all_consuming, map, map_parser, rest};
use nom::sequence::tuple;
use nom::{IResult, Parser};

/// Parses a WebAssembly custom section.
///
/// See <https://webassembly.github.io/spec/core/binary/modules.html#binary-customsec>
pub fn parse_custom_section(input: &[u8]) -> IResult<&[u8], Custom> {
    parse_section(ModuleSection::Custom, parse_custom_content)(input)
}

/// Parses the custom content (name and bytes) of a custom section.
fn parse_custom_content(input: &[u8]) -> IResult<&[u8], Custom> {
    map(tuple((parse_name, rest)), |(name, contents)| {
        Custom::new(name, Vec::from(contents))
    })(input)
}

/// Parses a WebAssembly type section.
///
/// See <https://webassembly.github.io/spec/core/binary/modules.html#binary-typesec>
pub fn parse_type_section(input: &[u8]) -> IResult<&[u8], Vec<FunctionType>> {
    parse_section(ModuleSection::Type, parse_vector(parse_function_type))(input)
}

/// Parses a section with the given identifier.
///
/// See <https://webassembly.github.io/spec/core/binary/modules.html#sections>
fn parse_section<'input, O, P>(
    section: ModuleSection,
    parser: P,
) -> impl FnMut(&'input [u8]) -> IResult<&'input [u8], O>
where
    P: Parser<&'input [u8], O, nom::error::Error<&'input [u8]>>,
{
    map_parser(parse_section_raw(section), all_consuming(parser))
}

/// Parses the raw bytes of a section with the given identifier.
/// Validates the section identified and length.
///
/// See <https://webassembly.github.io/spec/core/binary/modules.html#sections>
fn parse_section_raw(section: ModuleSection) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> {
    move |input| {
        let (input, _) = tag(&[section as u8])(input)?;
        let (input, length) = parse_u32(input)?;

        take(length)(input)
    }
}
