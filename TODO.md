

- unit test all in Python
- unit test all in C#
- service constructs in custom wrapper with re-entrance checks?
- benchmarks and document reference project call overhead for each backend method



- todo-todo: Create issues for TODOs
  
- replace all String, Vec, ... with 'static version
    - is that even possible? 
    - IIRC I ran into issues a) collecting variable length info before, and b) cyclic types (e.g., CType::Struct containing CType::Struct)
    - use &'static (&'a?) CType and <const N: usize> (or &'static [CType]) for nested fields instead?


