use std::hash::Hash;
use indexmap::IndexMap;

use crate::prelude::{Identifiable, Tree};

pub struct IndexTree<Id, Data, Parent, Child, Item>
    where
        Item: Identifiable,
{
    pub id: Id,
    pub data: Data,
    pub parent: Option<Parent>,
    pub children: IndexMap<Id, Child>,
    pub items: IndexMap<Item::Id, Item>,
}

crate::impl_tree!(IndexTree, IndexMap);
crate::impl_map_tree!(IndexTree);