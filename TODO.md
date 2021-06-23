


- todo-todo: Create issues for TODOs

- benchmark and document reference project call overhead for each backend method

- replace all String, Vec, ... with 'static version
    - is that even possible? 
    - IIRC I ran into issues a) collecting variable length info before, and b) cyclic types (e.g., CType::Struct containing CType::Struct)
    - use &'static (&'a?) CType and <const N: usize> (or &'static [CType]) for nested fields instead?


