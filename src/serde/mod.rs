mod error;
mod map;
use error::{Error, Result};

pub mod enc;

use crate::amf0::{AMFData, ObjectType, AMF};

use serde::de::Deserializer;
use serde::de::Visitor;
impl<'de> Deserializer<'de> for &mut AMFData<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.next().ok_or(Error::EOF)?? {
            AMF::Number(x) => visitor.visit_f64(x),
            AMF::Boolean(x) => visitor.visit_bool(x),
            AMF::String(x) => visitor.visit_borrowed_str(x),
            AMF::ObjectStart(ObjectType::Typed(x)) => {
                visitor.visit_map(map::TypedAMFObject::new(x, self))
            }
            AMF::ObjectStart(_) => visitor.visit_map(self),
            AMF::ObjectPropertyKey(_) => Err(Error::ParseError),
            AMF::ObjectEnd => Err(Error::ParseError),
            AMF::Null => visitor.visit_unit(),
            AMF::Undefined => visitor.visit_unit(),
            AMF::Unsupported => visitor.visit_unit(),
            AMF::StrictArray(x) => visitor.visit_seq(map::SequencedAMFObject::new(x, self)),
            AMF::Date(x, y) => {
                let mut tmp = [0u8; 10];
                tmp[0..8].copy_from_slice(&x.to_ne_bytes());
                tmp[8..10].copy_from_slice(&y.to_ne_bytes());
                visitor.visit_bytes(&tmp)
            }
        }
    }

    serde::forward_to_deserialize_any! {bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string
            bytes byte_buf option unit_struct seq tuple newtype_struct tuple_struct map struct enum
        identifier ignored_any unit
    }
}
