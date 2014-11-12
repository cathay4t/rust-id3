extern crate audiotag;

use frame::stream::FrameStream;
use frame::Frame;
use audiotag::{TagResult, TagError, StringDecodingError};
use util;

pub struct FrameV2;
impl FrameStream for FrameV2 {
    fn read(reader: &mut Reader, _: Option<FrameV2>) -> TagResult<Option<(u32, Frame)>> {
        let mut frame = Frame::with_version("", 2);

        frame.id = id_or_padding!(reader, 3);
        debug!("reading {}", frame.id); 

        let sizebytes = try!(reader.read_exact(3));
        let read_size = (sizebytes[0] as u32 << 16) | (sizebytes[1] as u32 << 8) | sizebytes[2] as u32;

        let data = try!(reader.read_exact(read_size as uint));
        try!(frame.parse_data(data.as_slice()));

        Ok(Some((6 + read_size, frame)))
    }

    fn write(writer: &mut Writer, frame: &Frame, _: Option<FrameV2>) -> TagResult<u32> {
        let content_bytes = frame.contents_to_bytes();
        let content_size = content_bytes.len() as u32;

        try!(writer.write(frame.id.slice_to(3).as_bytes()));
        try!(writer.write(util::u32_to_bytes(content_size).slice(1, 4)));
        try!(writer.write(content_bytes.as_slice()));

        Ok(6 + content_size)
    }
}