use indexmap::IndexMap;

/// Represents a dynamically-typed value from an API response
/// where the schema is unknown or uses `additionalProperties: true`.
#[derive(Debug, Clone, PartialEq)]
pub enum UntypedNode {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<UntypedNode>),
    Object(IndexMap<String, UntypedNode>),
}

impl UntypedNode {
    pub fn is_null(&self) -> bool {
        matches!(self, UntypedNode::Null)
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            UntypedNode::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            UntypedNode::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            UntypedNode::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            UntypedNode::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<UntypedNode>> {
        match self {
            UntypedNode::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&IndexMap<String, UntypedNode>> {
        match self {
            UntypedNode::Object(o) => Some(o),
            _ => None,
        }
    }
}
