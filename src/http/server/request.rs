use std::{
    io::{self, BufRead, BufReader},
    net::TcpStream,
};

use crate::utils::string::StringExt;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    InvalidRequestLine,
    MethodNotAllowed,
}

#[derive(Default, Debug)]
pub struct Request {
    pub path: String,
    pub headers: RequestHeaders,
}

#[derive(Default, Debug)]
pub struct RequestHeaders {
    pub if_none_match: Option<String>,
}

pub trait ReadRequest {
    fn read_request(&mut self) -> Result<Request, Error>;
}

impl ReadRequest for TcpStream {
    fn read_request(&mut self) -> Result<Request, Error> {
        let mut request = Request::default();
        let mut buf_reader = BufReader::new(self);
        let mut buffer = String::with_capacity(1024);

        parse_request_line(&mut buf_reader, &mut request, &mut buffer)?;
        parse_headers(&mut buf_reader, &mut request, &mut buffer)?;

        Ok(request)
    }
}

macro_rules! throw_if {
    ($expr:expr, $error:expr) => {
        if $expr {
            Err($error)?
        }
    };
}

fn parse_request_line(
    input: &mut BufReader<&mut TcpStream>,
    request: &mut Request,
    buffer: &mut String,
) -> Result<(), Error> {
    input.read_line(buffer).map_err(Error::Io)?;
    throw_if!(!buffer.starts_with("GET "), Error::MethodNotAllowed);
    request.path.reserve(buffer.len() - 4);
    for c in buffer.chars().skip(4) {
        if c == ' ' {
            break;
        }
        request.path.push(c);
    }
    if let Some(c) = request.path.chars().next() {
        throw_if!(c != '/', Error::InvalidRequestLine);
    }
    buffer.clear();
    Ok(())
}

fn parse_headers(
    input: &mut BufReader<&mut TcpStream>,
    request: &mut Request,
    buffer: &mut String,
) -> Result<(), Error> {
    loop {
        input.read_line(buffer).unwrap();
        buffer.truncate(buffer.len() - 2);
        if buffer.is_empty() {
            buffer.clear();
            break;
        }
        if buffer.starts_with_ci("if-none-match: ") {
            request.headers.if_none_match = Some(buffer.split_off(15));
        }
        buffer.clear();
    }
    Ok(())
}
