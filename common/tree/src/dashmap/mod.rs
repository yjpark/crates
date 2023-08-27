use std::hash::Hash;
pub use dashmap::DashMap;

use crate::prelude::{Identifiable, Tree};

pub struct DashTree<Id, Data, Parent, Child, Item>
    where
        Item: Identifiable,
{
    pub id: Id,
    pub data: Data,
    pub parent: Option<Parent>,
    pub children: DashMap<Id, Child>,
    pub items: DashMap<Item::Id, Item>,
}

crate::impl_tree!(DashTree, DashMap);

impl<Id, Data, Parent, Child, Item> DashTree<Id, Data, Parent, Child, Item>
    where
        Id: Eq + Hash + Clone,
        Parent: AsRef<Self>,
        Child: AsRef<Self>,
        Item: Identifiable,
{
    #[inline]
    fn _with_child<O, F: Fn(Option<&Child>) -> O>(&self, id: &Id, callback: &F) -> O {
        match self.children.get(id) {
            None => callback(None),
            Some(kv) => callback(Some(kv.value())),
        }
    }

    #[inline]
    fn _with_item<O, F: Fn(Option<&Item>) -> O>(&self, id: &Item::Id, callback: &F) -> O {
        match self.items.get(id) {
            None => callback(None),
            Some(kv) => callback(Some(kv.value())),
        }
    }

    #[inline]
    fn _each_child<V, F: Fn(&Child) -> Option<V>>(&self, callback: &F) -> Option<V> {
        for kv in self.children.iter() {
            if let Some(v) = callback(kv.value()) {
                return Some(v);
            }
        }
        None
    }

    #[inline]
    fn _each_item<V, F: Fn(&Item) -> Option<V>>(&self, callback: &F) -> Option<V> {
        for kv in self.items.iter() {
            if let Some(v) = callback(kv.value()) {
                return Some(v);
            }
        }
        None
    }
}

impl<Id, Data, Parent, Child, Item> DashTree<Id, Data, Parent, Child, Item>
    where
        Id: Eq + Hash + Clone,
        Parent: AsRef<Self>,
        Child: AsRef<Self> + Clone,
        Item: Identifiable,
{
    pub fn get_child(&self, id: &Id) -> Option<Child> {
        self.children.get(id).map(|x| x.value().clone() )
    }
}

impl<Id, Data, Parent, Child, Item> DashTree<Id, Data, Parent, Child, Item>
    where
        Id: Eq + Hash + Clone,
        Parent: AsRef<Self>,
        Child: AsRef<Self>,
        Item: Identifiable + Clone,
{
    pub fn get_item(&self, id: &Item::Id) -> Option<Item> {
        self.items.get(id).map(|x| x.value().clone() )
    }
}

impl<Id, Data, Parent, Child, Item> DashTree<Id, Data, Parent, Child, Item>
    where
        Id: Eq + Hash + Clone,
        Parent: AsRef<Self>,
        Child: AsRef<Self>,
        Item: Identifiable,
{
    pub fn add_child(&self, child: Child) {
        self.children.insert(child.as_ref().id().clone(), child);
    }

    pub fn add_item(&self, item: Item) {
        self.items.insert(item.id().clone(), item);
    }
}
