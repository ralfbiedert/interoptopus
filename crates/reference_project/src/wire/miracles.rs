use interoptopus::wire::{Wire, Wireable};
use interoptopus::{ffi, ffi_function, ffi_type};

#[ffi_type(wired)]
pub struct MyWiredType {
    name: String,
    values: Vec<u32>,
}

// input is a serialized representation, parse it to access MyWiredType.
// serialize resulting MyWiredType into a buffer and return it as WireOfMyWiredType on C# side
#[ffi_function]
fn perform_miracles(mut input: Wire<MyWiredType>) -> Wire<MyWiredType> {
    let w = input.unwire().expect("Something went wrong");
    w.wire()
}

#[ffi_function]
fn perform_half_miracles(mut input: Wire<MyWiredType>, other: ffi::String) -> ffi::String {
    let w = input.unwire().expect("Something went wrong");
    let result = format!("{} {}", w.name, other.as_str());
    result.into()
}

#[ffi_function]
fn perform_half_miracles_in_other_direction(input: ffi::String) -> Wire<'static, MyWiredType> {
    MyWiredType { name: input.as_str().to_owned(), values: vec![] }.wire()
}

// #[ffi_type(wired)]
// pub struct OtherType {
//     name: String,
//     foo: Foo1,
//     values: Vec<u32>,
//     mappngs: HashMap<String, HashMap<String, MyWiredType>>,
// }
//
// #[ffi_type(wired)]
// pub struct SomethingComplex {
//     name: String,
//     values: Vec<u32>,
//     mappngs: HashMap<String, HashMap<String, OtherType>>,
// }
//
// // #[ffi_type(wired)]
// // struct Foo1 {
// //     x: u32,
// //     y: i32,
// // }
//
// struct Foo1 {
//     x: u32,
//     y: i32,
// }
// impl ::interoptopus::lang::WireInfo for Foo1 {
//     fn name() -> &'static str {
//         "Foo1"
//     }
//     fn is_fixed_size_element() -> bool {
//         true && <u32 as ::interoptopus::lang::WireInfo>::is_fixed_size_element() && <i32 as ::interoptopus::lang::WireInfo>::is_fixed_size_element()
//     }
//     fn wire_info() -> ::interoptopus::lang::Type {
//         let docs = ::interoptopus::lang::Docs::from_line("");
//         let mut meta = ::interoptopus::lang::Meta::with_module_docs("".to_string(), docs);
//         let mut wire_fields: ::std::vec::Vec<interoptopus::lang::Field> = ::std::vec::Vec::new();
//         let mut generics: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
//         let name = format!("{}{}", "Foo1".to_string(), generics.join(""));
//         {
//             let docs = ::interoptopus::lang::Docs::from_line("");
//             let the_type = <u32 as ::interoptopus::lang::WireInfo>::wire_info();
//             let field = ::interoptopus::lang::Field::with_docs("x".to_string(), the_type, interoptopus::lang::Visibility::Private, docs);
//             wire_fields.push(field);
//         }
//         {
//             let docs = ::interoptopus::lang::Docs::from_line("");
//             let the_type = <i32 as ::interoptopus::lang::WireInfo>::wire_info();
//             let field = ::interoptopus::lang::Field::with_docs("y".to_string(), the_type, interoptopus::lang::Visibility::Private, docs);
//             wire_fields.push(field);
//         }
//         let repr = ::interoptopus::lang::Representation::new(::interoptopus::lang::Layout::C, None);
//         let retval = ::interoptopus::lang::Composite::with_meta_repr(name, wire_fields, meta, repr);
//         ::interoptopus::lang::Type::WirePayload(::interoptopus::lang::WirePayload::Composite(retval))
//     }
// }
// impl ::interoptopus::wire::Ser for Foo1 {
//     fn ser(&self, output: &mut impl ::std::io::Write) -> ::std::result::Result<(), ::interoptopus::wire::WireError> {
//         self.x.ser(output)?;
//         self.y.ser(output)?;
//         Ok(())
//     }
//     fn storage_size(&self) -> usize {
//         0 + self.x.storage_size() + self.y.storage_size()
//     }
// }
// impl ::interoptopus::wire::De for Foo1 {
//     fn de(input: &mut impl ::std::io::Read) -> ::std::result::Result<Self, ::interoptopus::wire::WireError>
//     where
//         Self: Sized,
//     {
//         let x = u32::de(input)?;
//         let y = i32::de(input)?;
//         Ok(Self { x, y })
//     }
// }
//
// // #[ffi_type]
// // struct Foo {
// //     x: u32,
// //     y: i32,
// // }
//
// #[repr(C)]
// struct Foo2 {
//     x: u32,
//     y: i32,
// }
//
// unsafe impl ::interoptopus::lang::TypeInfo for Foo2 {
//     fn type_info() -> ::interoptopus::lang::Type {
//         let docs = ::interoptopus::lang::Docs::from_line("");
//         let mut meta = ::interoptopus::lang::Meta::with_module_docs("".to_string(), docs);
//         let mut fields: ::std::vec::Vec<interoptopus::lang::Field> = ::std::vec::Vec::new();
//         let mut generics: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
//         let name = format!("{}{}", "Foo".to_string(), generics.join(""));
//         {
//             let docs = ::interoptopus::lang::Docs::from_line("");
//             let the_type = <u32 as ::interoptopus::lang::TypeInfo>::type_info();
//             let field = ::interoptopus::lang::Field::with_docs("x".to_string(), the_type, interoptopus::lang::Visibility::Private, docs);
//             fields.push(field);
//         }
//         {
//             let docs = ::interoptopus::lang::Docs::from_line("");
//             let the_type = <i32 as ::interoptopus::lang::TypeInfo>::type_info();
//             let field = ::interoptopus::lang::Field::with_docs("y".to_string(), the_type, interoptopus::lang::Visibility::Private, docs);
//             fields.push(field);
//         }
//         let repr = ::interoptopus::lang::Representation::new(::interoptopus::lang::Layout::C, None);
//         let rval = ::interoptopus::lang::Composite::with_meta_repr(name, fields, meta, repr);
//         ::interoptopus::lang::Type::Composite(rval)
//     }
// }
