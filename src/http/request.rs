use super::method::{Method, MethodError};
use super::query_string::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug, Result as FmtResult, Formatter};
use std::str;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf>  {
    pub fn path(&self) -> &'buf str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> Request<'buf> {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String>{
        unimplemented!()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(value: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(value).or(Err(ParseError::InvalidEncoding))?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidEncoding)?;
       
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;
        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        return Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(req: &str) -> Option<(&str, &str)> {
    for (i, c) in req.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&req[..i], &req[i + 1..]));
        } 
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod => "InvalidMethod",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}


impl Error for ParseError {

}