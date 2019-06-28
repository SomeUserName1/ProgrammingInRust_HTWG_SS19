use std::io;
use std::str;

use tokio::codec::{Encoder, Decoder};
use tokio_core::io::Codec;
use tokio_core::io::EasyBuf;
use serde_json;

use messages::Msg;

/// Encoding and decoding; include encryption
pub struct MsgCodec; // json line

impl Codec for MsgCodec {
    type In = Msg;
    type Out = Msg;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.drain_to(i);

            // Also remove the '\n'
            buf.drain_to(1);

            // Turn this data into a UTF string
            let s = str::from_utf8(line.as_slice())
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            // Then turn it into json
            serde_json::from_str(&s)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))

        } else {
            Ok(None)
        }
    }

    fn encode(&mut self, msg: Msg, buf: &mut Vec<u8>)
              -> io::Result<()>
    {
        let json_msg = serde_json::to_string(&msg)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        buf.extend(json_msg.as_bytes());
        buf.push(b'\n');
        Ok(())
    }
}
