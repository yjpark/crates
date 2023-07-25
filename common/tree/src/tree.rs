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

    fn with_child<O, F: Fn(Option<&Self::Child>) -> O>(&self, id: &Self::Id, callback: &F) -> O;
    fn with_item<O, F: Fn(Option<&Self::Item>) -> O>(&self, id: &<Self::Item as Identifiable>::Id, callback: &F) -> O;

    fn each_child<V, F: Fn(&Self::Child) -> Option<V>>(&self, callback: &F) -> Option<V>;
    fn each_item<V, F: Fn(&Self::Item) -> Option<V>>(&self, callback: &F) -> Option<V>;

    fn each_node<V, F: Fn(&Self::Child) -> Option<V>>(&self, callback: &F) -> Option<V> {
        self.each_child(&|child| {
            if let Some(v) = callback(child) {
                return Some(v);
            }
            if let Some(v) = child.as_ref().each_node(callback) {
                return Some(v);
            }
            None
        })
    }

    fn each_node_item<V, F: Fn(&Self::Item) -> Option<V>>(&self, callback: &F) -> Option<V> {
        if let Some(v) = self.each_item(callback) {
            return Some(v);
        }
        self.each_child(&|child| {
            child.as_ref().each_node_item(callback)
        })
    }
}