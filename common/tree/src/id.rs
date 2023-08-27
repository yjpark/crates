use std::hash::Hash;

pub trait Identifiable {
    type Id: Eq + Hash + Clone;
    fn id(&self) -> Self::Id;
}
