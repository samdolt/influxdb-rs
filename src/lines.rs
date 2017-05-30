pub enum Value {
    Float(f64),
    Integer(i64),
    String(String),
    Boolean(bool),
}

impl Value {
    fn add_to_buf(self, buf: &mut String) {
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
               match b {
                   true => buf.push('t'),
                   false => buf.push('f'),
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

pub struct LinesBuilder {
    buf: String
}

pub struct LinesBuilderWithFields {
   buf: String
}

pub struct Lines {
    buf: String
}

fn escape_and_push(string: &str, buf: &mut String) {
    for char in string.chars() {
        match char {
            ',' => buf.push_str("\\,"),
            ' ' => buf.push_str("\\ "),
            '=' => buf.push_str("\\="),
            c => buf.push(c),
        }
    }
}


impl LinesBuilder {
    pub fn new(measurement: &str) -> LinesBuilder {

        let mut buf = String::with_capacity(100);


        let mut obj = LinesBuilder {
            buf: buf
        };

        obj.init(&measurement);
        obj
    }

    fn from_line(line: Lines, name: &str) -> LinesBuilder {
        LinesBuilder::from_buf(line.buf, name)
    }

    fn from_buf(buf: String, name: &str) -> LinesBuilder {
        let mut obj = LinesBuilder {
            buf: buf,
        };

        obj.buf.reserve(100);

        if obj.buf != "" {
            obj.buf.push('\n');
        }
        obj.init(&name);
        obj
    }

    fn init(&mut self, measurement: &str){
        for char in measurement.chars() {
            match char {
                ',' => self.buf.push_str("\\,"),
                ' ' => self.buf.push_str("\\ "),
                c => self.buf.push(c),
            };
        }
    }

    pub fn add_tag(mut self, key: &str, value: &str) -> LinesBuilder {
        self.buf.push(',');
        escape_and_push(key, &mut self.buf);
        self.buf.push('=');
        escape_and_push(value, &mut self.buf);
        self
    }

    pub fn add_field<T>(mut self, key: &str, value: T) -> LinesBuilderWithFields where T: Into<Value> {
        let value = value.into();

        // First field, we add a space
        self.buf.push(' ');
        escape_and_push(key, &mut self.buf);
        self.buf.push('=');
        value.add_to_buf(&mut self.buf);
        LinesBuilderWithFields::new(self)
    }
}

impl LinesBuilderWithFields {
    fn new(builder: LinesBuilder) -> LinesBuilderWithFields {
        LinesBuilderWithFields {
            buf: builder.buf,
        }
    }

    pub fn add_field<T>(mut self, key: &str, value: T) -> LinesBuilderWithFields where T: Into<Value> {
        let value = value.into();
        // Add more than one field
        self.buf.push(',');
        escape_and_push(key, &mut self.buf);
        self.buf.push('=');
        value.add_to_buf(&mut self.buf);
        self
    }

    pub fn build(mut self) -> Lines {
        Lines::from_line_builder_with_fields(self)
    }

    pub fn add_line(mut self, name: &str) -> LinesBuilder {
        LinesBuilder::from_buf(self.buf, name)
    }

}

impl<'a> Lines {
    pub fn new() -> Lines {
       Lines {
           buf: String::with_capacity(100),
       }
    }

    fn from_line_builder_with_fields(builder: LinesBuilderWithFields) -> Lines {
        Lines {
            buf: builder.buf
        }
    }

    pub fn add_line(mut self, name: &str) -> LinesBuilder {
        LinesBuilder::from_line(self, name)
    }

    pub fn as_str(&'a self) -> &'a str {
       &self.buf
    }

    pub unsafe fn from_str<T>(line: T) -> Lines where T: Into<String>  {
        let line = line.into();

        Lines {
            buf: line,
        }
    }
}


#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_basic_line() {
        let line = LinesBuilder::new("t")
                            .add_field("a", 32)
                            .build();

        assert_eq!(line.as_str(), "t a=32i");

    }

    #[test]
    fn test_with_tags_and_fields() {
        let line = LinesBuilder::new("t")
            .add_tag("a", "b")
            .add_tag("c", "d")
            .add_field("e", "f")
            .add_field("g", 1f64)
            .build();

        assert_eq!(line.as_str(), "t,a=b,c=d e=\"f\",g=1");


    }

    #[test]
    fn test_two_lines() {
        let line = LinesBuilder::new("t")
            .add_tag("a", "b")
            .add_tag("c", "d")
            .add_field("e", "f")
            .add_field("g", "h")
            .build().add_line("t")
            .add_tag("a", "b")
            .add_tag("c", "d")
            .add_field("e", "f")
            .add_field("g", "h")
            .build();

        assert_eq!(line.as_str(), "t,a=b,c=d e=\"f\",g=\"h\"\nt,a=b,c=d e=\"f\",g=\"h\"");


    }



}