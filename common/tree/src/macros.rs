#[macro_export]
macro_rules! impl_tree {
    ($type: ident, $map: ident) => {
        impl<Id, Data, Parent, Child, Item> Identifiable for $type<Id, Data, Parent, Child, Item>
            where
                Id: Eq + Hash + Clone,
                Parent: AsRef<Self>,
                Child: AsRef<Self>,
                Item: Identifiable,
        {
            type Id = Id;

            fn id(&self) -> &Id {
                &self.id
            }
        }

        impl<Id, Data, Parent, Child, Item> Tree for $type<Id, Data, Parent, Child, Item>
            where
                Id: Eq + Hash + Clone,
                Parent: AsRef<Self>,
                Child: AsRef<Self>,
                Item: Identifiable,
        {
            type Data = Data;
            type Parent = Parent;
            type Child = Child;
            type Item = Item;
            type Children = $map<Id, Child>;
            type Items = $map<Item::Id, Item>;

            fn data(&self) -> &Self::Data {
                &self.data
            }

            fn parent(&self) -> Option<&Self::Parent> {
                self.parent.as_ref()
            }

            fn children(&self) -> &Self::Children {
                &self.children
            }

            fn items(&self) -> &Self::Items {
                &self.items
            }

            fn with_child<O, F: Fn(Option<&Self::Child>) -> O>(&self, id: &Self::Id, callback: &F) -> O {
                self._with_child(id, callback)
            }

            fn with_item<O, F: Fn(Option<&Self::Item>) -> O>(&self, id: &<Self::Item as Identifiable>::Id, callback: &F) -> O {
                self._with_item(id, callback)
            }

            fn each_child<V, F: Fn(&Self::Child) -> Option<V>>(&self, callback: &F) -> Option<V> {
                self._each_child(callback)
            }

            fn each_item<V, F: Fn(&Self::Item) -> Option<V>>(&self, callback: &F) -> Option<V> {
                self._each_item(callback)
            }
        }

        impl<Id, Data, Parent, Child, Item> $type<Id, Data, Parent, Child, Item>
            where
                Id: Eq + Hash + Clone,
                Parent: AsRef<Self>,
                Child: AsRef<Self> + Clone,
                Item: Identifiable,
        {
            pub fn get_node(&self, id: &Id) -> Option<Child> {
                if let Some(child) = self.get_child(id) {
                    return Some(child);
                }
                self.each_child(&|child| {
                    if let Some(child) = child.as_ref().get_node(id) {
                        return Some(child.clone());
                    }
                    None
                })
            }
        }

        impl<Id, Data, Parent, Child, Item> $type<Id, Data, Parent, Child, Item>
            where
                Id: Eq + Hash + Clone,
                Parent: AsRef<Self>,
                Child: AsRef<Self>,
                Item: Identifiable + Clone,
        {
            pub fn get_node_item(&self, id: &Item::Id) -> Option<Item> {
                if let Some(item) = self.get_item(id) {
                    return Some(item);
                }
                self.each_child(&|child| {
                    if let Some(item) = child.as_ref().get_node_item(id) {
                        return Some(item.clone());
                    }
                    None
                })
            }
        }
    }
}

#[macro_export]
macro_rules! impl_map_tree {
    ($type: ident) => {
        impl<Id, Data, Parent, Child, Item> $type<Id, Data, Parent, Child, Item>
            where
                Id: Eq + Hash + Clone,
                Parent: AsRef<Self>,
                Child: AsRef<Self>,
                Item: Identifiable,
        {
            #[inline]
            fn _with_child<O, F: Fn(Option<&Child>) -> O>(&self, id: &Id, callback: &F) -> O {
                callback(self.children.get(id))
            }

            #[inline]
            fn _with_item<O, F: Fn(Option<&Item>) -> O>(&self, id: &Item::Id, callback: &F) -> O {
                callback(self.items.get(id))
            }

            #[inline]
            fn _each_child<V, F: Fn(&Child) -> Option<V>>(&self, callback: &F) -> Option<V> {
                for (_, child) in self.children.iter() {
                    if let Some(v) = callback(child) {
                        return Some(v);
                    }
                }
                None
            }

            #[inline]
            fn _each_item<V, F: Fn(&Item) -> Option<V>>(&self, callback: &F) -> Option<V> {
                for (_, item) in self.items.iter() {
                    if let Some(v) = callback(item) {
                        return Some(v);
                    }
                }
                None
            }
        }

        impl<Id, Data, Parent, Child, Item> $type<Id, Data, Parent, Child, Item>
            where
                Id: Eq + Hash + Clone,
                Parent: AsRef<Self>,
                Child: AsRef<Self> + Clone,
                Item: Identifiable,
        {
            pub fn get_child(&self, id: &Id) -> Option<Child> {
                self.children.get(id).map(|x| x.clone())
            }
        }

        impl<Id, Data, Parent, Child, Item> $type<Id, Data, Parent, Child, Item>
            where
                Id: Eq + Hash + Clone,
                Parent: AsRef<Self>,
                Child: AsRef<Self>,
                Item: Identifiable + Clone,
        {
            pub fn get_item(&self, id: &Item::Id) -> Option<Item> {
                self.items.get(id).map(|x| x.clone())
            }
        }
    }
}

#[macro_export]
macro_rules! impl_default_for_tree {
    ($type: ident) => {
        impl<Id, Data, Parent, Child, Item> Default for $type<Id, Data, Parent, Child, Item>
            where
                Id: Eq + std::hash::Hash + Default,
                Data: Default,
                Parent: Default,
                Child: Default,
                Item: Identifiable + Default,
        {
            fn default() -> Self {
                Self {
                    id: Default::default(),
                    data: Default::default(),
                    parent: Default::default(),
                    children: Default::default(),
                    items: Default::default(),
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_debug_for_tree {
    ($type: ident) => {
        impl<Id, Data, Parent, Child, Item> std::fmt::Debug for $type<Id, Data, Parent, Child, Item>
            where
                Id: Eq + std::hash::Hash + std::fmt::Debug,
                Data: std::fmt::Debug,
                Parent: std::fmt::Debug,
                Child: std::fmt::Debug,
                Item: Identifiable + std::fmt::Debug,
                Item::Id: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut pmap = f.debug_map();
                pmap.entry(&"id", &self.id);
                pmap.entry(&"data", &self.data);
                pmap.entry(&"parent", &self.parent);
                pmap.entry(&"children", &self.children);
                pmap.entry(&"items", &self.items);
                pmap.finish()
            }
        }
    }
}

#[macro_export]
macro_rules! impl_clone_for_tree {
    ($type: ident) => {
        impl<Id, Data, Parent, Child, Item> Clone for $type<Id, Data, Parent, Child, Item>
            where
                Id: Eq + std::hash::Hash + Clone,
                Data: Clone,
                Parent: Clone,
                Child: Clone,
                Item: Identifiable + Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    id: self.id.clone(),
                    data: self.data.clone(),
                    parent: self.parent.clone(),
                    children: self.children.clone(),
                    items: self.items.clone(),
                }
            }
        }
    }
}