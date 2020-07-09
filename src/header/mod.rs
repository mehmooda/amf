use nom::number::complete::{be_i16, be_u16, be_u8};

#[derive(Debug)]
pub struct Header<'a> {
    pub name: &'a str,
    pub must_understand: bool,
    pub data: &'a [u8],
}

pub fn header(input: &[u8]) -> Result<(&[u8], Header), nom::Err<()>> {
    let (input, name) = nom::multi::length_data(be_u16)(input)?;
    let (input, must_understand) = be_u8(input)?;
    let (input, header_len) = be_i16(input)?;
    let (input, out) = if header_len == -1 {
        crate::amf0::skip_amf0(input)?
    } else {
        nom::bytes::complete::take(header_len as usize)(input)?
    };
    dbg!("UNHANDLED HEADER");
    dbg!(crate::amf0::AMFData::new(input)
        .map(Result::unwrap)
        .collect::<Vec<crate::amf0::AMF>>());
    Ok((
        input,
        Header {
            name: std::str::from_utf8(name)
                .ok()
                .ok_or(nom::Err::Failure(()))?,
            must_understand: must_understand > 0,
            data: out,
        },
    ))
}
