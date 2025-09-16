
xxx
    .register(foo())
    .register(bar())
    .register(type(XXX)) (enum / struct / primitive / union)
    .pattern(service!(MyService)) 





Id(u128)
TypeId(Id)
FieldId(Id)

Field:
    id: FieldId,
    rust_name: String,
    type: TypeId,

TypeEnum

TypeStruct

Type
    type_id: TypeId,
    rust_name: String,
    fields: Vec<FieldId>


Entry(enum)
    Type()



//

Inventory
    entries: Vec<Entry>

Inventory
    types: HashMap<TypeId, Type>

Inventory // this sucks looking up types, e.g., in fields. 
    structs: HashMap<StructId, Struct>
    enums: HashMap<EnumId, Enum>
    unions: HashMap<UnionId, Union>
    primitives: HashMap<PrimitiveId, Primitive>
