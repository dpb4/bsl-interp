#[derive(Debug, Clone)]
pub struct ParsingContext {
    data: String,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone)]
pub struct ParseSuccess<T> {
    context: ParsingContext,
    result: T,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    context: ParsingContext,
    message: &'static str,
}

#[derive(Debug, Clone)]
pub enum ParseResult<T> {
    Success(ParseSuccess<T>),
    Error(ParseError),
}
