use crate::prelude::Identifiable;

pub trait Tree : Identifiable {
    type Data;
    type Parent: AsRef<Self>;
    type Child: AsRef<Self>;
    type Item: Identifiable;
    type Children;
    type Items;

    fn data(&self) -> &Self::Data;
    fn parent(&self) -> Option<&Self::Parent>;
    fn children(&self) -> &Self::Children;
    fn items(&self) -> &Self::Items;

    fn has_parent(&self) -> bool {
        self.parent().is_some()
    }

    fn is_root(&self) -> bool {
        self.parent().is_none()
    }

    fn has_child(&self, id: &Self::Id) -> bool {
        self.with_child(id, &|x| {
            x.is_some()
        })
    }
    fn deep_has_child(&self, id: &Self::Id) -> bool {
        if self.has_child(id) {
            return true;
        }
        self.each_child(&|child| {
            if child.as_ref().deep_has_child(id) {
                return Some(true);
            }
            None
        }).is_some()
    }

    fn with_child<O, F: Fn(Option<&Self::Child>) -> O>(&self, id: &Self::Id, callback: &F) -> O;
    fn deep_with_child<O, F: Fn(Option<&Self::Child>) -> O>(&self, id: &Self::Id, callback: &F) -> O {
        if self.has_child(id) {
            return self.with_child(id, callback);
        }
        self.each_child(&|child| {
            if child.as_ref().deep_has_child(id) {
                return Some(child.as_ref().deep_with_child(id, callback));
            }
            None
        }).unwrap_or_else(|| callback(None) )
    }

    fn has_item(&self, id: &<Self::Item as Identifiable>::Id) -> bool {
        self.with_item(id, &|x| {
            x.is_some()
        })
    }
    fn deep_has_item(&self, id: &<Self::Item as Identifiable>::Id) -> bool {
        if self.has_item(id) {
            return true;
        }
        self.each_child(&|child| {
            if child.as_ref().deep_has_item(id) {
                return Some(true);
            }
            None
        }).is_some()
    }

    fn with_item<O, F: Fn(Option<&Self::Item>) -> O>(&self, id: &<Self::Item as Identifiable>::Id, callback: &F) -> O;
    fn deep_with_item<O, F: Fn(Option<&Self::Item>) -> O>(&self, id: &<Self::Item as Identifiable>::Id, callback: &F) -> O {
        if self.has_item(id) {
            return self.with_item(id, callback);
        }
        self.each_child(&|child| {
            if child.as_ref().deep_has_item(id) {
                return Some(child.as_ref().deep_with_item(id, callback));
            }
            None
        }).unwrap_or_else(|| callback(None) )
    }

    fn each_child<V, F: Fn(&Self::Child) -> Option<V>>(&self, callback: &F) -> Option<V>;
    fn deep_each_child<V, F: Fn(&Self::Child) -> Option<V>>(&self, callback: &F) -> Option<V> {
        self.each_child(&|child| {
            if let Some(v) = callback(child) {
                return Some(v);
            }
            if let Some(v) = child.as_ref().deep_each_child(callback) {
                return Some(v);
            }
            None
        })
    }

    fn each_item<V, F: Fn(&Self::Item) -> Option<V>>(&self, callback: &F) -> Option<V>;
    fn deep_each_item<V, F: Fn(&Self::Item) -> Option<V>>(&self, callback: &F) -> Option<V> {
        if let Some(v) = self.each_item(callback) {
            return Some(v);
        }
        self.each_child(&|child| {
            child.as_ref().deep_each_item(callback)
        })
    }

    //Parent: Clone
    //fn get_parent(&self) -> Option<Self::Parent>;
    //fn get_root(&self) -> Option<Self::Parent>;

    //Child: Clone
    //fn get_child(&self, id: &Self::Id) -> Option<Self::Child>
    //fn deep_get_child(&self, id: &Self::Id) -> Option<Self::Child>

    //Item: Clone
    //fn get_item(&self, id: &Item::Id) -> Option<Self::Child>
    //fn deep_get_item(&self, id: &Item::Id) -> Option<Self::Child>
}
