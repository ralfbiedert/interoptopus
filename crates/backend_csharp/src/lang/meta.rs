use interoptopus::new_id;

new_id!(NamespaceId);

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}
