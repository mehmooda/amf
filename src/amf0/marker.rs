pub enum Marker {
    Number,
    Boolean,
    String,
    Object,
    MovieClip,
    Null,
    Undefined,
    Reference,
    ECMAArray,
    ObjectEnd,
    StrictArray,
    Date,
    LongString,
    Unsupported,
    RecordSet,
    XmlDocument,
    TypedObject,
    //  AMF3
}

impl From<MarkerError> for nom::Err<()> {
    fn from(_: MarkerError) -> nom::Err<()> {
        nom::Err::Failure(())
    }
}

pub struct MarkerError;

impl std::convert::TryFrom<u8> for Marker {
    type Error = MarkerError;

    fn try_from(v: u8) -> Result<Marker, MarkerError> {
        match v {
            0 => Ok(Marker::Number),
            1 => Ok(Marker::Boolean),
            2 => Ok(Marker::String),
            3 => Ok(Marker::Object),
            4 => Ok(Marker::MovieClip),
            5 => Ok(Marker::Null),
            6 => Ok(Marker::Undefined),
            7 => Ok(Marker::Reference),
            8 => Ok(Marker::ECMAArray),
            9 => Ok(Marker::ObjectEnd),
            10 => Ok(Marker::StrictArray),
            11 => Ok(Marker::Date),
            12 => Ok(Marker::LongString),
            13 => Ok(Marker::Unsupported),
            14 => Ok(Marker::RecordSet),
            15 => Ok(Marker::XmlDocument),
            16 => Ok(Marker::TypedObject),
            //          17 => Ok(Marker::AMF3)
            _ => Err(MarkerError),
        }
    }
}
