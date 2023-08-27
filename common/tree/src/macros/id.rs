#[macro_export]
macro_rules! impl_identifiable_as_ref {
    ($type: ident) => {
        impl<Id> Identifiable for $type<Id>
            where
                Id: Identifiable
        {
            type Id = Id::Id;

            fn id(&self) -> Id::Id {
                self.as_ref().id()
            }
        }
    }
}