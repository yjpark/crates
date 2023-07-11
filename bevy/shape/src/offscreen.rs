use bevy::prelude::*;

pub fn offset_2() -> Vec2 {
    Vec2::new(-999999.0, -999999.0)
}
pub fn offset() -> Vec3 {
    Vec3::new(-999999.0, -999999.0, -999999.0)
}
pub fn transform() -> Transform {
    Transform {
        translation: Vec3::new(-999999.0, -999999.0, -999999.0),
        ..Default::default()
    }
}
