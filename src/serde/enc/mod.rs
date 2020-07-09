pub mod date;
mod unimplemented;

use unimplemented::Unimplemented;

use serde::ser;
use std::io::Write;

pub struct Encoder<W: Write> {
    w: W,
}

impl<W: Write> Encoder<W> {
    pub fn new(w: W) -> Self {
        Encoder { w }
    }
}

pub struct SeqEncoder<'a, W: Write> {
    w: &'a mut W,
    count: usize,
    v: Vec<u8>,
}

use ser::Serializer;

impl<'a, W: Write> Serializer for &'a mut Encoder<W> {
    type Ok = ();
    type Error = serde::de::value::Error;

    type SerializeSeq = SeqEncoder<'a, W>;
    type SerializeTuple = SeqEncoder<'a, W>;
    type SerializeTupleStruct = Unimplemented;
    type SerializeTupleVariant = Unimplemented;
    type SerializeMap = Unimplemented;
    type SerializeStruct = Self;
    type SerializeStructVariant = Unimplemented;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.w
            .write_all(&[1, v as u8])
            .map_err(serde::ser::Error::custom)
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.w.write_all(&[0]).map_err(serde::ser::Error::custom)?;
        self.w
            .write_all(&v.to_be_bytes())
            .map_err(serde::ser::Error::custom)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        if v.len() >= 1 << 16 {
            todo!("AMF LONGSTRING")
        }
        self.w
            .write_all(&[2, ((v.len() >> 8) % 256) as u8, (v.len() % 256) as u8])
            .map_err(serde::ser::Error::custom)?;
        self.w
            .write_all(v.as_bytes())
            .map_err(serde::ser::Error::custom)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: serde::Serialize + ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        if name == "AMF\x00" {
            let out = value.serialize(&mut date::RawDateSerializer {})?;
            self.w
                .write_all(&out[..])
                .map_err(serde::ser::Error::custom)
        } else {
            unimplemented!()
        }
    }

    fn serialize_newtype_variant<T: serde::Serialize + ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let v = match len {
            Some(x) => {
                self.w
                    .write_all(&[
                        10,
                        ((x >> 24) % 256) as u8,
                        ((x >> 16) % 256) as u8,
                        ((x >> 8) % 256) as u8,
                        (x % 256) as u8,
                    ])
                    .unwrap();
                vec![]
            }
            None => vec![10, 0, 0, 0, 0],
        };
        Ok(SeqEncoder {
            w: &mut self.w,
            count: 0,
            v,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.w.write_all(&[3]).map_err(serde::ser::Error::custom)?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}

impl<'a, W: Write> ser::SerializeSeq for SeqEncoder<'a, W> {
    type Ok = ();
    type Error = serde::de::value::Error;
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.count += 1;
        if self.v.is_empty() {
            value.serialize(&mut Encoder { w: &mut self.w })
        } else {
            value.serialize(&mut Encoder { w: &mut self.v })
        }
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        if !self.v.is_empty() {
            self.v[1] = ((self.count >> 24) % 256) as u8;
            self.v[2] = ((self.count >> 16) % 256) as u8;
            self.v[3] = ((self.count >> 8) % 256) as u8;
            self.v[4] = (self.count % 256) as u8;
            self.w.write_all(&self.v).map_err(serde::ser::Error::custom)
        } else {
            Ok(())
        }
    }
}

impl<'a, W: Write> ser::SerializeTuple for SeqEncoder<'a, W> {
    type Ok = ();
    type Error = serde::de::value::Error;
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<W> ser::SerializeStruct for &mut Encoder<W>
where
    W: Write,
{
    type Ok = ();
    type Error = serde::de::value::Error;
    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        if key.len() >= 1 << 16 {
            panic!("unsupported")
        }
        self.w
            .write_all(&[((key.len() >> 8) % 256) as u8, (key.len() % 256) as u8])
            .map_err(serde::ser::Error::custom)?;
        self.w
            .write_all(key.as_bytes())
            .map_err(serde::ser::Error::custom)?;
        value.serialize(&mut Encoder { w: &mut self.w })
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.w
            .write_all(&[0, 0, 9])
            .map_err(serde::ser::Error::custom)
    }
}

#[cfg(test)]
mod test {
    use serde::Serialize;
    #[test]
    fn serialize_bool() {
        let mut out = vec![];

        (true, "A")
            .serialize(&mut super::Encoder { w: &mut out })
            .unwrap();

        println!("{:?}", out)
    }
}
