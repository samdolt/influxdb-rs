/// Represent a value supported by InfluxDb.
pub enum Value {
    Float(f64),
    Integer(i64),
    String(String),
    Boolean(bool),
}

impl Value {
    pub fn add_to_buf(self, buf: &mut String) {
        match self {
            Value::Float(f) => buf.push_str(&format!("{}", f)),
            Value::Integer(i) => buf.push_str(&format!("{}i", i)),
            Value::String(s) => {
                buf.push('"');
                for c in s.chars() {
                    match c {
                        '"' => buf.push_str("\\\""),
                        c   => buf.push(c),
                    };
                };
                buf.push('"');
            },
            Value::Boolean(b) => {
                if b {
                    buf.push('t');
                } else {
                    buf.push('f');
                }
            }
        }
    }
}

impl From<f64> for Value {
    fn from(number: f64) ->  Value {
        Value::Float(number)
    }
}

impl From<i64> for Value {
    fn from(number: i64) -> Value {
        Value::Integer(number)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Value {
        Value::String(s)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Value {
        Value::String(s.to_string())
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Value {
        Value::Boolean(b)
    }
}