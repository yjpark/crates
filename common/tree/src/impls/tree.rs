use crate::prelude::*;
use crate::*;

impl_default_for_tree!(HashTree);
#[cfg(feature = "indexmap")]
impl_default_for_tree!(IndexTree);
#[cfg(feature = "dashmap")]
impl_default_for_tree!(DashTree);

impl_debug_for_tree!(HashTree);
#[cfg(feature = "indexmap")]
impl_debug_for_tree!(IndexTree);
#[cfg(feature = "dashmap")]
impl_debug_for_tree!(DashTree);

impl_clone_for_tree!(HashTree);
#[cfg(feature = "indexmap")]
impl_clone_for_tree!(IndexTree);
#[cfg(feature = "dashmap")]
impl_clone_for_tree!(DashTree);
