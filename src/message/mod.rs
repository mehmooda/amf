use nom::number::complete::be_i32;
use nom::number::complete::be_u16;

#[derive(Debug)]
pub struct Message<'a> {
    pub target_uri: &'a str,
    pub response_uri: &'a str,
    pub message: &'a [u8],
}

pub fn message(input: &[u8]) -> Result<(&[u8], Message), nom::Err<()>> {
    let (input, target) = nom::multi::length_data(be_u16)(input)?;
    let (input, response) = nom::multi::length_data(be_u16)(input)?;
    let (input, header_len) = be_i32(input)?;
    let (input, out) = if header_len == -1 {
        crate::amf0::skip_amf0(input)?
    } else {
        nom::bytes::complete::take(header_len as usize)(input)?
    };
    Ok((
        input,
        Message {
            target_uri: std::str::from_utf8(target)
                .ok()
                .ok_or(nom::Err::Failure(()))?,
            response_uri: std::str::from_utf8(response)
                .ok()
                .ok_or(nom::Err::Failure(()))?,
            message: out,
        },
    ))
}
