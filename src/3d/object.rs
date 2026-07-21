use super::{Mesh, Shape, Transform3D};

#[derive(Debug)]
pub struct Object {
    id: ObjectId,
    pub mesh_id: MeshId,
    pub material_id: MaterialId,
    pub transform: Transform3D,
    pub visible: bool,
}

impl Object {
    pub fn id(&self) -> ObjectId {
        self.id
    }

    pub(crate) fn new(
        id: ObjectId,
        ObjectSpec {
            mesh_id,
            material_id,
            transform,
            visible,
        }: ObjectSpec,
    ) -> Self {
        Self {
            id,
            mesh_id,
            material_id,
            transform,
            visible,
        }
    }
}

pub struct ObjectSpec {
    pub mesh_id: MeshId,
    pub material_id: MaterialId,
    pub transform: Transform3D,
    pub visible: bool,
}

impl ObjectSpec {
    pub fn new(mesh_id: MeshId, material_id: MaterialId) -> Self {
        Self {
            mesh_id,
            material_id,
            transform: Transform3D::default(),
            visible: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObjectId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MeshId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MaterialId(pub u32);

#[derive(Default)]
pub struct MeshStore {
    meshes: Vec<Option<Mesh>>,
}

impl MeshStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, mesh: Mesh) -> MeshId {
        let id = MeshId(self.free_id());

        if id.0 == self.meshes.len() {
            self.meshes.push(Some(mesh));
        } else {
            self.meshes[id.0] = Some(mesh);
        }

        id
    }

    pub fn insert_shape(&mut self, shape: Shape) -> MeshId {
        self.insert(shape.into())
    }

    pub fn get(&self, id: MeshId) -> Option<&Mesh> {
        self.meshes.get(id.0).and_then(Option::as_ref)
    }

    pub fn get_mut(&mut self, id: MeshId) -> Option<&mut Mesh> {
        self.meshes.get_mut(id.0).and_then(Option::as_mut)
    }

    fn free_id(&self) -> usize {
        self.meshes
            .iter()
            .position(Option::is_none)
            .unwrap_or(self.meshes.len())
    }
}

#[derive(Default)]
pub struct ObjectStore {
    pub objects: Vec<Option<Object>>,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_object(&mut self, spec: ObjectSpec) -> ObjectId {
        let id = ObjectId(self.free_id());
        let object = Object::new(id, spec);

        if id.0 == self.objects.len() {
            self.objects.push(Some(object));
        } else {
            self.objects[id.0] = Some(object);
        }

        id
    }

    pub fn get(&self, id: ObjectId) -> Option<&Object> {
        self.objects.get(id.0).and_then(Option::as_ref)
    }

    pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut Object> {
        self.objects.get_mut(id.0).and_then(Option::as_mut)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Object> {
        self.objects.iter().filter_map(Option::as_ref)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Object> {
        self.objects.iter_mut().filter_map(Option::as_mut)
    }

    fn free_id(&self) -> usize {
        self.objects
            .iter()
            .position(Option::is_none)
            .unwrap_or(self.objects.len())
    }
}
