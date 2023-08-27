use std::rc::Rc;
use std::sync::Arc;

use crate::prelude::Identifiable;
use crate::impl_identifiable_as_ref;

impl<Id> Identifiable for dyn AsRef<Id>
    where
        Id: Identifiable
{
    type Id = Id::Id;

    fn id(&self) -> Id::Id {
        self.as_ref().id().clone()
    }
}

impl_identifiable_as_ref!(Rc);
impl_identifiable_as_ref!(Arc);

impl<Id> Identifiable for Option<Id>
    where
        Id: Identifiable,
        Id::Id: Default,
{
    type Id = Id::Id;

    fn id(&self) -> Id::Id {
        self.as_ref()
            .map(|x| x.id() )
            .unwrap_or_else(|| Id::Id::default() )
    }
}