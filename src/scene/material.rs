use std::ffi::CString;
use std::os::raw::{c_uint};
use ffi::{AiMaterial, AiMaterialProperty, AiPropertyTypeInfo, AiColor4D, aiGetMaterialColor, AiReturn};

use super::material_property::{MaterialProperty, MaterialPropertyIter};
use math::color3::Color3D;

define_type_and_iterator_indirect! {
    /// Material type (not yet implemented)
    struct Material(&AiMaterial)
    /// Material iterator type.
    struct MaterialIter
}

impl<'a> Material<'a> {
    /// Returns an iterator over all the material properties in the material.
    pub fn property_iter(&self) -> MaterialPropertyIter {
        MaterialPropertyIter::new(self.properties as *const *const AiMaterialProperty,
                                  self.num_properties as usize)
    }

    pub fn num_properties(&self) -> u32 {
        self.num_properties
    }

    pub fn property(&self, id: usize) -> Option<MaterialProperty> {
        if id < self.num_properties as usize {
            unsafe { Some(MaterialProperty::from_raw(*(self.properties.offset(id as isize)))) }
        } else {
            None
        }
    }

    pub fn get_color3(&self, key: &str) -> Option<Color3D> {
        let cstr = CString::new(key).unwrap();

        let mut raw_color = AiColor4D { a: 0.0, r: 0.0, g: 0.0, b: 0.0 };
        let result = unsafe {
            aiGetMaterialColor(
                self.0, 
                cstr.as_ptr(), 
                0, //AiPropertyTypeInfo::Float as c_uint, 
                0 as c_uint, 
                &mut raw_color)
        };

        match result {
            AiReturn::Success => {
                Some(Color3D::new(raw_color.r, raw_color.g, raw_color.b))
            },
            _ => None,
        }
    }
}
