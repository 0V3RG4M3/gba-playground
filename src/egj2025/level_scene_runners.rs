use crate::egj2025::context::Context;
use crate::egj2025::key_level::KeyLevel;
use crate::egj2025::level_scene::LevelScene;
use crate::egj2025::rabbit_level::RabbitLevel;
use crate::scene::SceneRunner;

pub const LEVEL_SCENE_RUNNERS: [fn() -> SceneRunner<Context>; 2] = [
    SceneRunner::<()>::new::<LevelScene<KeyLevel>>,
    SceneRunner::<()>::new::<LevelScene<RabbitLevel>>,
];
