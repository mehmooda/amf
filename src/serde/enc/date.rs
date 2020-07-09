pub fn with_serialize<T, S>(v: &T, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
    T: GetDate,
{
    let mut out = [0u8; 11];
    out[0] = 0x0b;
    out[1..9].copy_from_slice(&v.date().to_be_bytes()[..]);
    out[9..11].copy_from_slice(&v.tz().to_be_bytes()[..]);
    s.serialize_newtype_struct("AMF\x00", serde_bytes::Bytes::new(&out[..]))
}

#[derive(Copy, Clone)]
pub struct Date {
    pub date: f64,
    pub tz: u16,
}

impl std::fmt::Debug for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use chrono::TimeZone;
        if self.tz != 0 {
            f.write_fmt(format_args!("TZ IS NOT ZERO = {}", self.tz))?
        }
        std::fmt::Debug::fmt(&chrono::Utc.timestamp_millis(self.date as i64), f)
    }
}

impl serde::Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        with_serialize(self, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(Date { date: 0.0, tz: 0 })
    }
}

impl<'de> serde::de::Visitor<'de> for Date {
    type Value = Self;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a DATE TYPE")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.len() != 10 {
            return Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Bytes(v),
                &self,
            ));
        }
        Ok(Date {
            date: f64::from_ne_bytes(v[0..8].try_into().unwrap()),
            tz: u16::from_ne_bytes(v[8..10].try_into().unwrap()),
        })
    }
}

impl GetDate for Date {
    fn date(&self) -> f64 {
        self.date
    }
    fn tz(&self) -> u16 {
        self.tz
    }
}

pub trait GetDate {
    fn date(&self) -> f64;
    fn tz(&self) -> u16;
}

pub(crate) struct RawDateSerializer;

type Unimplemented = serde::ser::Impossible<[u8; 11], serde::de::value::Error>;
use std::convert::TryInto;

impl serde::Serializer for &mut RawDateSerializer {
    type Ok = [u8; 11];
    type Error = serde::de::value::Error;
    type SerializeSeq = Unimplemented;
    type SerializeTuple = Unimplemented;
    type SerializeTupleStruct = Unimplemented;
    type SerializeTupleVariant = Unimplemented;
    type SerializeMap = Unimplemented;
    type SerializeStruct = Unimplemented;
    type SerializeStructVariant = Unimplemented;
    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        //        if self.state == 1 {
        //          self.state = 2;
        //        self.tz = v;
        //      Ok(())
        //} else {
        panic!("Unexpected Type handed to RawDateSerializer")
        //}
    }
    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        // if self.state == 0 {
        //   self.state = 1;
        //  self.date = v;
        //  Ok(())
        //} else {
        panic!("Unexpected Type handed to RawDateSerializer")
        // }
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        if v.len() == 11 {
            return Ok(v.try_into().unwrap());
        }
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        panic!("Unexpected Type handed to RawDateSerializer")
    }
}
