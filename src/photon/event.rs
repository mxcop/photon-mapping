use glam::{Vec2, Vec3};

/// Created when a photon interacts with a material.
#[derive(Clone, Copy)]
pub struct PhotonEvent {
    /// Location where the event occured.
    pub p: Vec2,
    /// Indicent direction of the photon.
    pub i: Vec2,
    /// Color of the photon.
    pub c: Vec3
    // Flux deposit. (monochromatic for now)
    // pub f: f32
}
