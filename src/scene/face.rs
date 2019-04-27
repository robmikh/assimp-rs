use std::mem;
use std::ops::Index;
use std::os::raw::c_uint;

use ffi::AiFace;

define_type_and_iterator! {
    /// Face type (not yet implemented)
    struct Face(&AiFace)
    /// Face iterator type.
    struct FaceIter
}

impl<'a> Index<isize> for Face<'a> {
    type Output = c_uint;
    fn index(&self, index: isize) -> &c_uint {
        unsafe {
            assert!(index < self.num_indices as isize);
            mem::transmute(self.indices.offset(index))
        }
    }
}

impl<'a> Face<'a> {
    pub fn num_indices(&self) -> u32 {
        self.num_indices
    }

    pub fn index_iter(&self) -> std::slice::Iter<u32> {
        let slice = unsafe {
            std::slice::from_raw_parts(
                self.indices, 
                self.num_indices as usize)
        };
        slice.iter()
    }

    pub fn get_index(&self, id: u32) -> Option<u32> {
        if id < self.num_indices {
            unsafe { Some(*self.indices.offset(id as isize)) }
        } else {
            None
        }
    }
}
