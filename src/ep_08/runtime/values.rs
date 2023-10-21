use std::{collections::HashMap, fmt};


#[derive(Debug, Clone)]
pub enum RtValue {
    NullVal,
    NumberVal(f64),
    StringVal(String),
    Boolean(bool),
    Object(ObjectRtVal),
}

impl fmt::Display for RtValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            RtValue::NullVal => write!(f, "null"),
            RtValue::NumberVal(x) => write!(f, "Number{{{x}}}"),
            RtValue::StringVal(x) => write!(f, "String{{{x}}}"),
            RtValue::Boolean(x) => write!(f, "Boolean{{{x}}}"),
            RtValue::Object(x) => write!(f, "Object{{{x}}}")
        }
    }
}


#[derive(Debug, Clone)]
pub struct ObjectRtVal {
    pub properties : HashMap<String, RtValue>,
}

impl fmt::Display for ObjectRtVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        _= writeln!(f,"properties = {{");
        for prop in &self.properties {
            let k = prop.0;
            let v = prop.1;
            _ = writeln!(f, "    {{{k} => {v}}}," );
        }
        _= writeln!(f,"}}");
        Ok(())
    }
}
