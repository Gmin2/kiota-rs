use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Put,
    Delete,
    Options,
    Head,
    Connect,
    Trace,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
            Self::Patch => write!(f, "PATCH"),
            Self::Put => write!(f, "PUT"),
            Self::Delete => write!(f, "DELETE"),
            Self::Options => write!(f, "OPTIONS"),
            Self::Head => write!(f, "HEAD"),
            Self::Connect => write!(f, "CONNECT"),
            Self::Trace => write!(f, "TRACE"),
        }
    }
}
