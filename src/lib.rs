pub mod chord;
pub mod note;
pub mod playable;
pub mod score;
pub const DEFAULT_BASE: f32 = 523.26;
pub static STRUMMING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
