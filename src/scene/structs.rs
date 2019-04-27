#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PropertyTypeInfo {
    Float,
    Double,
    String,
    Integer,
    Buffer,
}