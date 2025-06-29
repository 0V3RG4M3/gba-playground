use crate::egj2025::context::Context;
use crate::egj2025::key_level::KeyLevel;
use crate::egj2025::level_scene::LevelScene;
use crate::scene::SceneRunner;

pub const LEVEL_SCENE_RUNNERS: [fn() -> SceneRunner<Context>; 1] =
    [SceneRunner::<()>::new::<LevelScene<KeyLevel>>];
