use ffi::AiMaterialProperty;

define_type_and_iterator_indirect! {
    /// MaterialProperty type (not yet implemented)
    struct MaterialProperty(&AiMaterialProperty)
    /// MaterialProperty iterator type.
    struct MaterialPropertyIter
}