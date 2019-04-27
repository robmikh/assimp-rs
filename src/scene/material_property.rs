use ffi::{AiMaterialProperty, AiPropertyTypeInfo};

use super::structs::{PropertyTypeInfo};

define_type_and_iterator_indirect! {
    /// MaterialProperty type (not yet implemented)
    struct MaterialProperty(&AiMaterialProperty)
    /// MaterialProperty iterator type.
    struct MaterialPropertyIter
}

impl<'a> MaterialProperty<'a> {
    pub fn key(&self) -> &str {
        self.key.as_ref()
    }

    pub fn property_type(&self) -> PropertyTypeInfo {
        match self.property_type {
            AiPropertyTypeInfo::Float => PropertyTypeInfo::Float,
            AiPropertyTypeInfo::Double => PropertyTypeInfo::Double,
            AiPropertyTypeInfo::String => PropertyTypeInfo::String,
            AiPropertyTypeInfo::Integer => PropertyTypeInfo::Integer,
            AiPropertyTypeInfo::Buffer => PropertyTypeInfo::Buffer,
        }
    }
}