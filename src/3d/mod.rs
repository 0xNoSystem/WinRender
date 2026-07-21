mod mesh;
mod object;
mod primitives;
mod transform;

pub use mesh::Mesh;
pub use object::{
    CullMode, Material, MaterialId, MaterialStore, MeshId, MeshStore, Object, ObjectId, ObjectSpec,
    ObjectStore,
};
pub use primitives::{
    Circle, Cube, Line2D, Rect, Shape, Sphere, Stroke2D, Torus, Triangle, Triangle3,
};
pub use transform::Transform3D;

pub(crate) use primitives::edge;
