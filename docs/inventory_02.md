
xxx
    .register(foo())
    .register(bar())
    .register(type(XXX)) (enum / struct / primitive / union)
    .pattern(service!(MyService)) 



Id(u128)
TypeId(Id)
FieldId(Id) // why would I want to look up fields independently?
            // they are all part of a type.
            // if I need to reference them elsewhere, I can just use (type_id, 2) or so.

VariantId(Id) // same

Field:
    id: FieldId,
    rust_name: String,
    type: TypeId,

TypeEnum
    variants: Vec<VariantId>

TypeStruct
    fields: Vec<FieldId>

TypeKind
    TypeEnum
    TypeStruct
    TypePrimitive
    TypeUnion

Type
    type_id: TypeId,
    rust_name: String,
    kind: TypeKind,


Inventory
    types: HashMap<TypeId, Type>
    functions: HashMap<FunctionId, Function>
