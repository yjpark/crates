//https://serde.rs/impl-serialize.html#serializing-a-struct
#[macro_export]
macro_rules! impl_serde_serialize_for_tree {
    ($type: ident) => {
        impl<Id, Data, Parent, Child, Item> Serialize for $type<Id, Data, Parent, Child, Item>
            where
                Id: Eq + Hash + Clone + Serialize,
                Data: Serialize,
                Parent: Serialize,
                Child: Serialize,
                Item: Identifiable + Serialize,
                Item::Id: Serialize,
        {
            fn serialize<T>(&self, serializer: T) -> Result<T::Ok, T::Error>
            where
                T: Serializer,
            {
                let mut state = serializer.serialize_struct(stringify!($type), 5)?;
                state.serialize_field("id", &self.id)?;
                state.serialize_field("data", &self.data)?;
                state.serialize_field("parent", &self.parent)?;
                state.serialize_field("children", &self.children)?;
                state.serialize_field("items", &self.items)?;
                state.end()
            }
        }
    }
}

//https://serde.rs/deserialize-struct.html
//Move visitor out of fn implementation to support generic types
#[macro_export]
macro_rules! impl_serde_deserialize_for_tree {
    ($type: ident, $visitor: ident) => {
        impl<'de, Id, Data, Parent, Child, Item> Deserialize<'de> for $type<Id, Data, Parent, Child, Item>
        where
            Id: Eq + Hash + Clone + Deserialize<'de>,
            Data: Deserialize<'de>,
            Parent: Deserialize<'de>,
            Child: Deserialize<'de>,
            Item: Identifiable + Deserialize<'de>,
            Item::Id: Deserialize<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_struct(stringify!($type), TREE_FIELDS, $visitor::<Id, Data, Parent, Child, Item>::new())
            }
        }

        struct $visitor<Id, Data, Parent, Child, Item>{
            id: PhantomData<Id>,
            data: PhantomData<Data>,
            parent: PhantomData<Parent>,
            child: PhantomData<Child>,
            item: PhantomData<Item>,
        }

        impl<Id, Data, Parent, Child, Item> $visitor<Id, Data, Parent, Child, Item> {
            fn new() -> Self {
                Self {
                    id: PhantomData,
                    data: PhantomData,
                    parent: PhantomData,
                    child: PhantomData,
                    item: PhantomData,
                }
            }
        }

        impl<'de, Id, Data, Parent, Child, Item> Visitor<'de> for $visitor<Id, Data, Parent, Child, Item>
        where
            Id: Eq + Hash + Clone + Deserialize<'de>,
            Data: Deserialize<'de>,
            Parent: Deserialize<'de>,
            Child: Deserialize<'de>,
            Item: Identifiable + Deserialize<'de>,
            Item::Id: Deserialize<'de>,
        {
            type Value = $type<Id, Data, Parent, Child, Item>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(concat!("struct ", stringify!($type)))
            }

            fn visit_map<V>(self, mut map: V) -> Result<$type<Id, Data, Parent, Child, Item>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id = None;
                let mut data = None;
                let mut parent = None;
                let mut children = None;
                let mut items = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        TreeField::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        TreeField::Data => {
                            if data.is_some() {
                                return Err(de::Error::duplicate_field("data"));
                            }
                            data = Some(map.next_value()?);
                        }
                        TreeField::Parent => {
                            if parent.is_some() {
                                return Err(de::Error::duplicate_field("parent"));
                            }
                            parent = Some(map.next_value()?);
                        }
                        TreeField::Children => {
                            if children.is_some() {
                                return Err(de::Error::duplicate_field("children"));
                            }
                            children = Some(map.next_value()?);
                        }
                        TreeField::Items => {
                            if items.is_some() {
                                return Err(de::Error::duplicate_field("items"));
                            }
                            items = Some(map.next_value()?);
                        }
                    }
                }
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                let data = data.ok_or_else(|| de::Error::missing_field("data"))?;
                let parent = parent.ok_or_else(|| de::Error::missing_field("parent"))?;
                let children = children.ok_or_else(|| de::Error::missing_field("children"))?;
                let items = items.ok_or_else(|| de::Error::missing_field("items"))?;
                Ok($type{id, data, parent, children, items})
            }
        }
    }
}

#[macro_export]
macro_rules! impl_serde_for_tree {
    ($type: ident, $visitor: ident) => {
        crate::impl_serde_serialize_for_tree!($type);
        crate::impl_serde_deserialize_for_tree!($type, $visitor);
    }
}
