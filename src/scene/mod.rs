//! The `scene` module contains definitions of imported scene data.

pub use self::animation::*;
pub use self::camera::*;
pub use self::face::*;
pub use self::light::*;
pub use self::material::*;
pub use self::material_property::*;
pub use self::mesh::*;
pub use self::node::*;
pub use self::scene::Scene;
pub use self::texture::*;
pub use self::structs::*;

mod animation;
mod camera;
mod face;
mod light;
mod material;
mod material_property;
mod mesh;
mod node;
mod scene;
mod texture;
mod structs;
