use nom::number::complete::{be_f64, be_i16, be_u16, be_u32, be_u8};

mod marker;

use marker::Marker;

use std::convert::TryInto;

fn read_marker(input: &[u8]) -> Result<(&[u8], Marker), nom::Err<()>> {
    let (input, marker) = be_u8(input)?;
    Ok((input, marker.try_into()?))
}

#[derive(Debug)]
pub enum AMF<'a> {
    Number(f64),
    Boolean(bool),
    String(&'a str),
    ObjectStart(ObjectType<'a>),
    ObjectPropertyKey(&'a str),
    ObjectEnd,
    Null,
    Undefined,
    Unsupported,
    StrictArray(u32),
    Date(f64, i16),
}

#[derive(Debug)]
pub enum ObjectType<'a> {
    Normal,
    ECMAArray(u32),
    Typed(&'a str),
}

fn simple(input: &[u8]) -> Result<(&[u8], AMF), nom::Err<()>> {
    let (mut input, marker) = read_marker(input)?;
    let out = match marker {
        Marker::Number => {
            let (ninput, out) = be_f64(input)?;
            input = ninput;
            AMF::Number(out)
        }
        Marker::Boolean => {
            let (ninput, out) = be_u8(input)?;
            input = ninput;
            AMF::Boolean(out != 0)
        }
        Marker::String => {
            let (ninput, out) = nom::multi::length_data(be_u16)(input)?;
            input = ninput;
            AMF::String(std::str::from_utf8(out).ok().ok_or(nom::Err::Failure(()))?)
        }
        Marker::Object => AMF::ObjectStart(ObjectType::Normal),
        Marker::Null => AMF::Null,
        Marker::Undefined => AMF::Undefined,
        Marker::Reference => unimplemented!(),
        Marker::ECMAArray => {
            let (ninput, count) = be_u32(input)?;
            input = ninput;
            AMF::ObjectStart(ObjectType::ECMAArray(count))
        }
        Marker::ObjectEnd => AMF::ObjectEnd,
        Marker::StrictArray => {
            let (ninput, out) = be_u32(input)?;
            input = ninput;
            AMF::StrictArray(out)
        }
        Marker::Date => {
            let (ninput, date) = be_f64(input)?;
            let (ninput, tz) = be_i16(ninput)?;
            input = ninput;
            AMF::Date(date, tz)
        }
        Marker::XmlDocument | Marker::LongString => {
            let (ninput, out) = nom::multi::length_data(be_u32)(input)?;
            input = ninput;
            AMF::String(std::str::from_utf8(out).ok().ok_or(nom::Err::Failure(()))?)
        }
        Marker::Unsupported => AMF::Unsupported,
        Marker::MovieClip | Marker::RecordSet => return Err(nom::Err::Failure(())),
        Marker::TypedObject => {
            let (ninput, class_name) = nom::multi::length_data(be_u16)(input)?;
            input = ninput;
            AMF::ObjectStart(ObjectType::Typed(
                std::str::from_utf8(class_name)
                    .ok()
                    .ok_or(nom::Err::Failure(()))?,
            ))
        }
    };

    Ok((input, out))
}

enum AMFState {
    Initial,
    ObjectKey,
    ObjectValue,
    Array(u32),
}

pub struct AMFData<'a> {
    pub data: &'a [u8],
    state: Vec<AMFState>,
}

impl<'a> AMFData<'a> {
    pub fn new(data: &'a [u8]) -> AMFData<'a> {
        AMFData {
            data,
            state: vec![AMFState::Initial],
        }
    }
}

impl AMFData<'_> {
    pub fn reset(&mut self) {
        self.state = vec![AMFState::Initial];
    }
}

impl<'a> Iterator for AMFData<'a> {
    type Item = Result<AMF<'a>, nom::Err<()>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }
        let out = match self.state.pop()? {
            AMFState::Initial => {
                let pobject = simple(self.data);
                if let Err(x) = pobject {
                    self.state.clear();
                    return Some(Err(x));
                }
                let (input, object) = pobject.ok()?;
                if let AMF::ObjectEnd = object {
                    self.state.clear();
                    return Some(Err(nom::Err::Failure(())));
                };
                self.data = input;
                object
            }
            AMFState::ObjectKey => {
                let pkey = nom::multi::length_data(be_u16)(self.data);
                if let Err(x) = pkey {
                    self.state.clear();
                    return Some(Err(x));
                }
                let (input, key) = pkey.ok()?;
                let skey = std::str::from_utf8(key);
                if skey.is_err() {
                    self.state.clear();
                    return Some(Err(nom::Err::Failure(())));
                }
                let key = skey.ok()?;
                if key.is_empty() {
                    let pobject = simple(input);
                    if let Err(x) = pobject {
                        self.state.clear();
                        return Some(Err(x));
                    }
                    let (ninput, object) = pobject.ok()?;
                    if let AMF::ObjectEnd = object {
                        self.data = ninput;
                        return Some(Ok(AMF::ObjectEnd));
                    } else {
                        self.state.clear();
                        return Some(Err(nom::Err::Failure(())));
                    }
                }
                self.data = input;
                self.state.push(AMFState::ObjectValue);
                return Some(Ok(AMF::ObjectPropertyKey(key)));
            }
            AMFState::ObjectValue => {
                let pobject = simple(self.data);
                if let Err(x) = pobject {
                    self.state.clear();
                    return Some(Err(x));
                }
                let (input, object) = pobject.ok()?;
                if let AMF::ObjectEnd = object {
                    self.state.clear();
                    return Some(Err(nom::Err::Failure(())));
                };
                self.data = input;
                self.state.push(AMFState::ObjectKey);
                object
            }
            AMFState::Array(x) => {
                let pobject = simple(self.data);
                if let Err(x) = pobject {
                    self.state.clear();
                    return Some(Err(x));
                }
                let (input, object) = pobject.ok()?;
                if let AMF::ObjectEnd = object {
                    self.state.clear();
                    return Some(Err(nom::Err::Failure(())));
                };
                self.data = input;
                if x > 1 {
                    self.state.push(AMFState::Array(x - 1))
                }
                object
            }
        };
        match out {
            AMF::ObjectStart(_) => self.state.push(AMFState::ObjectKey),
            AMF::StrictArray(0) => {}
            AMF::StrictArray(x) => self.state.push(AMFState::Array(x)),
            _ => {}
        };
        Some(Ok(out))
    }
}

pub fn skip_amf0(input: &[u8]) -> Result<(&[u8], &[u8]), nom::Err<()>> {
    let mut amf = AMFData::new(input);
    if amf.by_ref().any(|i| i.is_err()) {
        Err(nom::Err::Failure(()))
    } else {
        Ok((amf.data, &input[0..input.len() - amf.data.len()]))
    }
}
