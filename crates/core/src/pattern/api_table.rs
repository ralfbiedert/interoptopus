// api_table!(Foo {
//     builtins_vec!(u8),
//     function!(function_f1),
//     function!(function_f1),
// });

// Steps
// - define struct Foo
// - translate markers to special fields:
//      - function -> foo: <foo as TypeXXX>::T,
//      - builtins_xxx(u8) -> builtins_xxx: BuiltinsXXX<u8>,
// - create Foo::default()
// - have user define `#[ffi] entry() -> Foo{}`

// On C# side:
// - Have Foo struct with basic methods that invoke fn pointer
// - overloads as regular method overloads of that struct
// - services & co: require struct to be passed as first parameter -> Service.New(foo, ...)
