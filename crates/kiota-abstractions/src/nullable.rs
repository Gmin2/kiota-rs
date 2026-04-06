/// Represents a value that can be explicitly null, distinct from absent.
///
/// Used for the required+nullable case in OpenAPI:
/// - `T` = required, not nullable
/// - `Option<T>` = not required, not nullable
/// - `Nullable<T>` = required, nullable
/// - `Option<Nullable<T>>` = not required, nullable
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Nullable<T> {
    Value(T),
    Null,
}

impl<T> Nullable<T> {
    pub fn is_null(&self) -> bool {
        matches!(self, Nullable::Null)
    }

    pub fn value(&self) -> Option<&T> {
        match self {
            Nullable::Value(v) => Some(v),
            Nullable::Null => None,
        }
    }

    pub fn into_value(self) -> Option<T> {
        match self {
            Nullable::Value(v) => Some(v),
            Nullable::Null => None,
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Nullable<U> {
        match self {
            Nullable::Value(v) => Nullable::Value(f(v)),
            Nullable::Null => Nullable::Null,
        }
    }
}

impl<T: Default> Default for Nullable<T> {
    fn default() -> Self {
        Nullable::Null
    }
}
