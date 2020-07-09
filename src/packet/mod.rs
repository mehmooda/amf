mod version;

struct Packet {
    pub version: version::Version,
    headers: u16,
}

use nom::number::complete::be_u16;

fn amf_packet_part1(input: &[u8]) -> Result<(&[u8], Packet), nom::Err<()>> {
    let (input, version) = be_u16(input)?;
    let (input, headers) = be_u16(input)?;
    use std::convert::TryInto;
    Ok((
        input,
        Packet {
            version: version.try_into()?,
            headers,
        },
    ))
}

fn amf_packet_part2(input: &[u8]) -> Result<(&[u8], u16), nom::Err<()>> {
    be_u16(input)
}

pub struct AMFPacket<'a> {
    input: &'a [u8],
    state: State,
}

enum State {
    Header(u16),
    Message(u16),
}

impl<'a> AMFPacket<'a> {
    pub fn new(input: &'a [u8]) -> AMFPacket<'a> {
        //TODO ERRORS
        let (input, p) = amf_packet_part1(input).unwrap();
        AMFPacket {
            input,
            state: State::Header(p.headers),
        }
    }
}

#[derive(Debug)]
pub enum PacketContents<'a> {
    Header(crate::header::Header<'a>),
    Message(crate::message::Message<'a>),
}

impl<'a> PacketContents<'a> {
    pub fn message(self) -> Option<crate::message::Message<'a>> {
        if let PacketContents::Message(x) = self {
            Some(x)
        } else {
            dbg!(self);
            None
        }
    }
}

impl<'a> Iterator for AMFPacket<'a> {
    type Item = Result<PacketContents<'a>, nom::Err<()>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            State::Header(0) => {
                let (input, messages) = {
                    let x = amf_packet_part2(self.input);
                    match x {
                        Ok((i, m)) => (i, m),
                        Err(e) => return Some(Err(e)),
                    }
                };
                self.input = input;
                self.state = State::Message(messages);
                self.next()
            }
            State::Header(x) => {
                let (input, header) = {
                    let x = crate::header::header(self.input);
                    match x {
                        Ok((i, h)) => (i, h),
                        Err(e) => return Some(Err(e)),
                    }
                };
                self.input = input;
                self.state = State::Header(x - 1);
                Some(Ok(PacketContents::Header(header)))
            }
            State::Message(0) => {
                if self.input.is_empty() {
                    None
                } else {
                    Some(Err(nom::Err::Error(())))
                }
            }
            State::Message(x) => {
                let (input, message) = {
                    let x = crate::message::message(self.input);
                    match x {
                        Ok((i, m)) => (i, m),
                        Err(e) => return Some(Err(e)),
                    }
                };
                self.input = input;
                self.state = State::Message(x - 1);
                Some(Ok(PacketContents::Message(message)))
            }
        }
    }
}
