use std::collections::HashMap;
use handle_errors::Error;

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

pub fn prepare_pagination(start: usize, end: usize, length: usize) -> (usize, usize) {
    if length == 1 {
        (0, 1)
    } else if start > end {
        if end < length {
            (0, end)
        } else {
            (0, length)
        }
    } else if end > length {
        (start, length)
    } else {
        (start, end)
    }
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }

    Err(Error::MissingParameters)
}
