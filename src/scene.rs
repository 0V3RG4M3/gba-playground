pub trait Scene {
    type C;

    fn new(context: &mut Self::C) -> Self;
    fn run(&mut self, context: &mut Self::C) -> SceneRunner<Self::C>;
}

pub struct SceneRunner<C> {
    run: fn(context: &mut C) -> SceneRunner<C>,
}

impl<C> SceneRunner<C> {
    pub fn new<S: Scene>() -> SceneRunner<S::C> {
        fn run<S: Scene>(context: &mut S::C) -> SceneRunner<S::C> {
            S::new(context).run(context)
        }

        SceneRunner { run: run::<S> }
    }

    pub fn run(self, context: &mut C) -> SceneRunner<C> {
        (self.run)(context)
    }
}

#[cfg(test)]
mod tests {
    use crate::scene::{Scene, SceneRunner};

    #[test]
    fn test_run() {
        struct FirstScene {}

        impl Scene for FirstScene {
            type C = u32;

            fn new(_: &mut u32) -> FirstScene {
                FirstScene {}
            }

            fn run(&mut self, context: &mut u32) -> SceneRunner<u32> {
                *context += 1;
                if *context % 3 == 0 {
                    SceneRunner::<u32>::new::<FirstScene>()
                } else {
                    SceneRunner::<u32>::new::<SecondScene>()
                }
            }
        }

        struct SecondScene {}

        impl Scene for SecondScene {
            type C = u32;

            fn new(_: &mut u32) -> SecondScene {
                SecondScene {}
            }

            fn run(&mut self, context: &mut u32) -> SceneRunner<u32> {
                *context += 2;
                if *context % 5 == 0 {
                    SceneRunner::<u32>::new::<SecondScene>()
                } else {
                    SceneRunner::<u32>::new::<FirstScene>()
                }
            }
        }

        let mut scene_runner = SceneRunner::<u32>::new::<FirstScene>();
        let mut context = 0;
        for expected_context in [1, 3, 4, 6, 7, 9, 10, 12, 13, 15, 17] {
            scene_runner = scene_runner.run(&mut context);
            assert_eq!(context, expected_context);
        }
    }
}
