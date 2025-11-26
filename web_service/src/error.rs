use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::{fmt, io::Error as IOError};

#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    IOError(String),
    NotFound(String),
    OtherError(String),
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let msg_body = match self {
            MyError::DBError(msg) => format!("Database error occurred: {msg:?}"),
            MyError::IOError(msg) => format!("IO error occurred: {msg:?}"),
            MyError::NotFound(msg) => format!("Not found error occurred: {msg:?}"),
            MyError::OtherError(msg) => format!("Other error occurred: {msg:?}"),
        };
        (StatusCode::OK, msg_body).into_response()
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl From<SQLxError> for MyError {
    fn from(err: SQLxError) -> Self {
        MyError::DBError(err.to_string())
    }
}

impl From<IOError> for MyError {
    fn from(err: IOError) -> Self {
        MyError::IOError(err.to_string())
    }
}

impl From<&str> for MyError {
    fn from(err: &str) -> Self {
        MyError::OtherError(err.to_string())
    }
}
