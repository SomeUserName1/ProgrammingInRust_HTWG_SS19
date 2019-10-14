use std::str;
use std::io;
use std::marker::PhantomData;

use bytes::{BytesMut, BufMut};
use tokio::codec::{Encoder, Decoder};
use serde::{Serialize, de::DeserializeOwned};
use serde_json;

use super::messages::Messages;
use crate::blockchain::transaction::Transactional;

pub struct MessagesCodec<T> {
 next_index: usize,
 phantom: PhantomData<T>,   
} // json line

impl<T> MessagesCodec<T>{
    pub fn new() -> Self {
        MessagesCodec {
            next_index: 0,
            phantom: PhantomData
        }
    }
}

impl<T> Decoder for MessagesCodec<T> 
where T: DeserializeOwned + Transactional
{
    type Item = Messages<T>;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let newline_index = self.next_index + i;
            // remove the serialized frame from the buffer.
            let line = buf.split_to(newline_index + 1);

            let line = &line[..line.len() - 1];
            

            // Turn this data into a UTF string
            let s = str::from_utf8(&line)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            // Then turn it into json
            serde_json::from_str(&s)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))

        } else {
            Ok(None)
        }
    }

}

impl<T> Encoder for MessagesCodec<T> 
where T: Transactional + Serialize
{
    type Item = Messages<T>;
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> io::Result<()>
    {
        let json_msg = serde_json::to_string(&msg)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        buf.extend(json_msg.as_bytes());
        buf.put_u8(b'\n');

        Ok(())
    }

}
