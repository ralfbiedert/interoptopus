/// Generates helper structs for our [`Pipeline`], given a bunch of stages.
#[macro_export]
macro_rules! stage_setup {
    ($data:ident, $config:ident, $ctor:ident, [$($t:tt,)*]) => {
        #[derive(Default)]
        pub struct StageData {
            $(
                pub $t: $t::Data,
            )*
        }

        #[derive(Default)]
        pub struct StageConfig {
            $(
                pub $t: $t::Config,
            )*
        }

        fn $ctor(config: StageConfig) -> Vec<Box<dyn Stage>> {
            vec![
                $(
                    $t::Stage::new(config.$t),
                )*
            ]
        }
    };
}

/// Defines a simple stage without config.
#[macro_export]
macro_rules! stage_without_config {
    () => {
        #[derive(Default)]
        pub struct Config {}

        pub struct Stage {
            config: Config,
        }

        impl Stage {
            pub fn new(config: Config) -> Box<dyn super::Stage> {
                Box::new(Self { config })
            }
        }

        impl super::Stage for Stage {
            fn process(&mut self, data: &mut StageData, inventory: &Inventory) {
                Self::process(self, data, inventory)
            }
        }
    };
}
