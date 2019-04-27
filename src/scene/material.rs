use ffi::{AiMaterial, AiMaterialProperty};

use super::material_property::{MaterialProperty, MaterialPropertyIter};

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
}
