use std::hash::Hash;
use std::marker::PhantomData;

use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

use crate::prelude::*;
use crate::impl_serde_for_tree;

#[derive(serde::Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum TreeField { Id, Data, Parent, Children, Items }
const TREE_FIELDS: &'static [&'static str] = &["id", "data", "parent", "children", "items"];

impl_serde_for_tree!(HashTree, HashTreeVisitor);

#[cfg(feature = "indexmap")]
impl_serde_for_tree!(IndexTree, IndexTreeVisitor);

#[cfg(feature = "dashmap")]
impl_serde_for_tree!(DashTree, DashTreeVisitor);
