mod reference_project {
    use interoptopus_c::{CLibrary, NamingStyle};

    #[test]
    fn interop() -> Result<(), Box<dyn std::error::Error>> {
        let inv = reference_project_c::inventory();
        let multibuf = CLibrary::builder(&inv)
            .loader_name("reference_project_c")
            .filename("reference_project.h")
            .prefix("rp_")
            .type_naming(NamingStyle::Snake)
            .enum_variant_naming(NamingStyle::Snake)
            .function_naming(NamingStyle::Snake)
            .function_parameter_naming(NamingStyle::Snake)
            .const_naming(NamingStyle::Snake)
            .build()
            .process()
            .map_err(|e| Box::new(std::io::Error::other(e.to_string())))?;
        multibuf.write_buffer_to(std::path::Path::new("tests/reference_project"), "reference_project.h")?;
        Ok(())
    }
}

mod snapshots {
    use interoptopus::inventory::RustInventory;
    use interoptopus_c::CLibrary;

    fn generate(name: &str, inv: &RustInventory) -> String {
        let multibuf = CLibrary::builder(inv).loader_name(name).build().process().unwrap();
        multibuf.buffer(&format!("{name}.h")).unwrap().clone()
    }

    #[test]
    fn full_reference_project() {
        let output = generate("reference_project_c", &reference_project_c::inventory());
        insta::assert_snapshot!(output);
    }

    mod basic_struct {
        use super::generate;
        use interoptopus::inventory::RustInventory;
        use interoptopus::{extra_type, ffi, function};

        #[ffi]
        #[derive(Copy, Clone)]
        pub struct Point2D {
            pub x: f32,
            pub y: f32,
        }

        #[ffi(export = unique)]
        pub fn add_points(a: Point2D, b: Point2D) -> Point2D {
            Point2D { x: a.x + b.x, y: a.y + b.y }
        }

        #[test]
        fn snapshot() {
            let inv = RustInventory::new().register(function!(add_points)).register(extra_type!(Point2D)).validate();
            insta::assert_snapshot!(generate("basic", &inv));
        }
    }

    mod simple_enum {
        use super::generate;
        use interoptopus::inventory::RustInventory;
        use interoptopus::{ffi, function};

        #[ffi]
        #[derive(Copy, Clone)]
        #[allow(dead_code)]
        pub enum Color {
            Red,
            Green,
            Blue,
        }

        #[ffi(export = unique)]
        pub fn use_color(_color: Color) {}

        #[test]
        fn snapshot() {
            let inv = RustInventory::new().register(function!(use_color)).validate();
            insta::assert_snapshot!(generate("enum_test", &inv));
        }
    }

    mod tagged_union {
        use super::generate;
        use interoptopus::inventory::RustInventory;
        use interoptopus::{ffi, function};

        #[ffi]
        #[derive(Copy, Clone)]
        pub struct Rect {
            pub w: f32,
            pub h: f32,
        }

        #[ffi]
        #[derive(Copy, Clone)]
        #[allow(dead_code)]
        pub enum Geom {
            Circle(f32),
            Box(Rect),
        }

        #[ffi(export = unique)]
        pub fn geom_area(g: Geom) -> f32 {
            match g {
                Geom::Circle(r) => std::f32::consts::PI * r * r,
                Geom::Box(r) => r.w * r.h,
            }
        }

        #[test]
        fn snapshot() {
            let inv = RustInventory::new().register(function!(geom_area)).validate();
            insta::assert_snapshot!(generate("shapes", &inv));
        }
    }

    mod callback_types {
        use super::generate;
        use interoptopus::inventory::RustInventory;
        use interoptopus::{callback, ffi, function};

        callback!(FloatCallback(value: f32) -> f32);

        #[ffi(export = unique)]
        pub fn invoke_float_cb(cb: FloatCallback) -> f32 {
            cb.call(42.0)
        }

        #[test]
        fn snapshot() {
            let inv = RustInventory::new().register(function!(invoke_float_cb)).validate();
            insta::assert_snapshot!(generate("callbacks", &inv));
        }
    }

    mod pattern_types {
        use super::generate;
        use interoptopus::inventory::RustInventory;
        use interoptopus::{ffi, function};

        #[ffi(export = unique)]
        pub fn sum_slice(data: ffi::Slice<u32>) -> u32 {
            data.as_slice().iter().sum()
        }

        #[ffi(export = unique)]
        pub fn unwrap_option(opt: ffi::Option<f32>) -> f32 {
            opt.into_option().unwrap_or(0.0)
        }

        #[test]
        fn snapshot() {
            let inv = RustInventory::new().register(function!(sum_slice)).register(function!(unwrap_option)).validate();
            insta::assert_snapshot!(generate("patterns", &inv));
        }
    }

    /// End-to-end test exercising prefix + non-default naming styles.
    mod custom_naming {
        use interoptopus::inventory::RustInventory;
        use interoptopus::{callback, ffi, function};
        use interoptopus_c::{CLibrary, NamingStyle};

        #[ffi]
        #[derive(Copy, Clone)]
        pub struct Vec2 {
            pub x: f32,
            pub y: f32,
        }

        #[ffi]
        #[derive(Copy, Clone)]
        #[allow(dead_code)]
        pub enum Shape {
            Circle(f32),
            Rectangle(Vec2),
        }

        callback!(ShapeCallback(shape: Shape) -> f32);

        #[ffi(export = unique)]
        pub fn shape_area(shape: Shape) -> f32 {
            match shape {
                Shape::Circle(r) => std::f32::consts::PI * r * r,
                Shape::Rectangle(v) => v.x * v.y,
            }
        }

        #[ffi(export = unique)]
        pub fn invoke_cb(cb: ShapeCallback) -> f32 {
            cb.call(Shape::Circle(1.0))
        }

        #[ffi(export = unique)]
        pub fn get_optional(opt: ffi::Option<f32>) -> f32 {
            opt.into_option().unwrap_or(0.0)
        }

        #[test]
        fn prefixed_upper_camel() {
            let inv = RustInventory::new()
                .register(function!(shape_area))
                .register(function!(invoke_cb))
                .register(function!(get_optional))
                .validate();
            let multibuf = CLibrary::builder(&inv)
                .loader_name("mylib")
                .prefix("mylib_")
                .type_naming(NamingStyle::UpperCamel)
                .enum_variant_naming(NamingStyle::ScreamingSnake)
                .function_naming(NamingStyle::Snake)
                .function_parameter_naming(NamingStyle::Snake)
                .const_naming(NamingStyle::ScreamingSnake)
                .build()
                .process()
                .unwrap();
            let output = multibuf.buffer("mylib.h").unwrap().clone();
            insta::assert_snapshot!(output);
        }
    }
}
