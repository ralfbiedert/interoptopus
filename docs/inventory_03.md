
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
    


TypeInfo: <-- why not make this `SymbolInfo` as it can register what it wants and is universal
    old: info() -> TypeInfo
    new: register(&mut registry) <- allows to register multiple types (e.g., children) at once and use typeid
        - each register call first would try to register all known used elements
        - registering something that exists already doesn't re-register it, just returns Id
        


