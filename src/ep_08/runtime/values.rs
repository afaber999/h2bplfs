use std::collections::HashMap;


#[derive(Debug, Clone)]
pub enum RtValue {
    NullVal,
    NumberVal(f64),
    StringVal(String),
    Boolean(bool),
    Object(ObjectRtVal),
}


#[derive(Debug, Clone)]
pub struct ObjectRtVal {
    pub properties : HashMap<String, RtValue>,
}
