
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
    stage_1: Stage1 
        config: StageXXXConfig
    stage_2: Stage2
    stage_3: Stage3
    stage_4: Stage4

    process(inventory) -> String
        stage_1.process(&inventory)
        stage_2.process(&inventory, &stage_1)
        stage_4.output

    


