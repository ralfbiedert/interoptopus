
xxx
    .register(foo())
    .register(bar())
    .register(type(XXX)) (enum / struct / primitive / union)
    .pattern(service!(MyService)) 



Id(u128)
TypeId(Id)
FunctionId(Id)


Field
    rust_name: String,
    type_id: TypeId,

Variant
    rust_name: String,
    type_id: TypeId,

TypeEnum
    variants: Vec<Variant>

TypeStruct
    fields: Vec<Field>

TypeKind
    TypeEnum
    TypeStruct
    TypePrimitive
    TypeUnion
    TypePattern
        Result
        Option

Type
    type_id: TypeId,
    rust_name: String,
    kind: TypeKind,

PatternKind
    Service,


Inventory
    types: HashMap<TypeId, Type>
    functions: HashMap<FunctionId, Function>
    const: HashMap<ConstId, Const>
    pattern: HashMap<PatternId, PatternKind>
    


SymbolInfo: <-- why not make this `SymbolInfo` as it can register what it wants and is universal
    register(&mut registry)
        


Inventory -> C# Bindings

Pipeline
    stages: Vec<Stage>
        Stage1, Stage2, Stage3, ...
    

    process(inventory) -> String
        for stage in stages
            stage.process(inventory, &dyn Stage)
        
        stage_1.process(&inventory)
        stage_2.process(&inventory, &stage_1)
        stage_4.output




crate_my_lib






Inventory
    types: HashMap<TypeId, Type>
    functions: HashMap<FunctionId, Function>
    const: HashMap<ConstId, Const>
    pattern: HashMap<PatternId, PatternKind>




my_rust_lib_ffi_import
    #[ffi_type]
    struct Something {}

    #[ffi_plugin(CSPluginImpl)]
    trait CSPlugin
        fn new() -> Self 
        fn bar(x: Something) -> ffi::String

    fn CsPlugin() -> PluginInfo
        PluginID 
        +list of methods

    fn get_instance(rt: Runtime) -> CSPlugin {
        // or
        // let rt = interoptopus_backend_csharp::Runtime::new();


        // let ??? = Instance<dyn CsPlugin>  rt.load_instance(CsPlugin());
        let x = rt.load_instance::<CsPluginXXX>();

        xxx.bar(x);
            // this needs specific type with methods 
    }


my_rust_lib
    foo()

    let rt = interoptopus_backend_csharp::Runtime::new();
    let csp = rt.load_plugin::<CSPlugin>(); // CAN'T, NEED SPECIFIC TYPE
    csp.foo();

    build.rs
        Interop::build("plugin.cs", inventory())
        // NO, THIS STILL NEEDS TO BE IMPLEMENTED IN C#, SO SOMEONE ELSE HAS TO COMPILE IT
        // compile("plugin.cs", "plugin.dll")
        // build.bat -> plugin.dll  


my_rust_lib_ffi_export
    #[ffi_function]
    foo()

my_rust_lib_ffi_export_build
    Interop::build("interop.cs", inventory())



