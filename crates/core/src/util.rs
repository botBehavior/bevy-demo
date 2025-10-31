use bevy::prelude::*;

pub fn clamp_to_bounds(position: &mut Vec2, bounds: Vec2) {
    let half = bounds * 0.5;
    position.x = position.x.clamp(-half.x, half.x);
    position.y = position.y.clamp(-half.y, half.y);
}

pub fn screen_to_world(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    cursor: Vec2,
) -> Option<Vec2> {
    camera
        .viewport_to_world(camera_transform, cursor)
        .map(|ray| ray.origin.truncate())
}
