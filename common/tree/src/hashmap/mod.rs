use std::hash::Hash;
use std::collections::HashMap;

use crate::prelude::{Identifiable, Tree};

pub struct HashTree<Id, Data, Parent, Child, Item>
    where
        Item: Identifiable,
{
    pub id: Id,
    pub data: Data,
    pub parent: Option<Parent>,
    pub children: HashMap<Id, Child>,
    pub items: HashMap<Item::Id, Item>,
}

crate::impl_tree!(HashTree, HashMap);
crate::impl_map_tree!(HashTree);