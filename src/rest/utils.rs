use actix_web::http::header::HeaderMap;
use crate::error::HeaderError;
use std::str::FromStr;

pub(crate) fn get_header_as<T: FromStr>(
    headers: &HeaderMap,
    header: &str
) -> Result<T, HeaderError> {
    headers.get(header)
        .ok_or_else(|| HeaderError::Missing(header.to_owned()))
        .and_then(|id| id.to_str().map_err(|_| HeaderError::Missing(header.to_owned())))
        .and_then(|id| id.parse::<T>().map_err(|_| HeaderError::Parse(header.to_owned())))
}
