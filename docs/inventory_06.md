
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

#[ffi_plugin]
trait CSPlugin
    fn foo();
    fn bar() -> ffi::String


let x = Runtime::new();
let csp = x.load_plugin::<CSPlugin>();
csp.foo();



Inventory
    types: HashMap<TypeId, Type>
    functions: HashMap<FunctionId, Function>
    const: HashMap<ConstId, Const>
    pattern: HashMap<PatternId, PatternKind>

    


my_rust_lib
    foo()

my_rust_lib_ffi
    #[ffi_function]
    foo()

my_rust_lib_ffi_build
    Interop::build("interop.cs", inventory())



