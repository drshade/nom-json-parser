mod parser;
mod repr;
mod utils;

pub use nom::{
    error::{ErrorKind, ParseError, VerboseError},
    Err, IResult,
};
pub use parser::parse_json as parser;
pub use parser::parse_json_element as parse;
pub use repr::{Json, JsonValue};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
