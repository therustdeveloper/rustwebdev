use crate::errors::Error;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: i32,
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        return Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<i32>()
                    .map_err(Error::Parse)?,
            ),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<i32>()
                .map_err(Error::Parse)?,
        });
    }

    Err(Error::MissingParameters)
}
