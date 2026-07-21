use crate::math::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform3D {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Transform3D {
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        let mut point = Vec3::new(
            point.x * self.scale.x,
            point.y * self.scale.y,
            point.z * self.scale.z,
        );

        let (sin_x, cos_x) = self.rotation.x.sin_cos();
        point = Vec3::new(
            point.x,
            point.y * cos_x - point.z * sin_x,
            point.y * sin_x + point.z * cos_x,
        );

        let (sin_y, cos_y) = self.rotation.y.sin_cos();
        point = Vec3::new(
            point.x * cos_y + point.z * sin_y,
            point.y,
            -point.x * sin_y + point.z * cos_y,
        );

        let (sin_z, cos_z) = self.rotation.z.sin_cos();
        point = Vec3::new(
            point.x * cos_z - point.y * sin_z,
            point.x * sin_z + point.y * cos_z,
            point.z,
        );

        point + self.position
    }
}

impl Default for Transform3D {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Vec3::ZERO,
            scale: Vec3::ONE,
        }
    }
}
