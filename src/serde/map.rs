use super::Error;
use crate::amf0::AMF;
use serde::de::{DeserializeSeed, MapAccess, SeqAccess};

impl<'de> MapAccess<'de> for crate::amf0::AMFData<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        match self.next().ok_or(Error::EOF)?? {
            AMF::ObjectPropertyKey(x) => seed
                .deserialize(serde::de::value::BorrowedStrDeserializer::new(x))
                .map(Some),
            AMF::ObjectEnd => Ok(None),
            _ => Err(super::Error::Message(
                "expected ObjectProptertyKey".to_string(),
            )),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }
}

pub struct TypedAMFObject<'de, 'a> {
    name: Option<&'de str>,
    inner: &'a mut crate::amf0::AMFData<'de>,
}

impl<'de, 'a> TypedAMFObject<'de, 'a> {
    pub fn new(name: &'de str, inner: &'a mut crate::amf0::AMFData<'de>) -> Self {
        TypedAMFObject {
            name: Some(name),
            inner,
        }
    }
}

impl<'a, 'de> MapAccess<'de> for TypedAMFObject<'de, 'a> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.name.is_some() {
            seed.deserialize(serde::de::value::BorrowedStrDeserializer::new(
                "AMF_TYPEDOBJECT_NAME",
            ))
            .map(Some)
        } else {
            self.inner.next_key_seed(seed)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(x) = self.name.take() {
            seed.deserialize(serde::de::value::BorrowedStrDeserializer::new(x))
        } else {
            self.inner.next_value_seed(seed)
        }
    }
}

pub struct SequencedAMFObject<'de, 'a> {
    len: u32,
    inner: &'a mut crate::amf0::AMFData<'de>,
}

impl<'de, 'a> SequencedAMFObject<'de, 'a> {
    pub fn new(len: u32, inner: &'a mut crate::amf0::AMFData<'de>) -> Self {
        SequencedAMFObject { len, inner }
    }
}

impl<'a, 'de> SeqAccess<'de> for SequencedAMFObject<'de, 'a> {
    type Error = Error;

    fn next_element_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if self.len > 0 {
            self.len -= 1;
            seed.deserialize(&mut *self.inner).map(Some)
        } else {
            Ok(None)
        }
    }
}
