// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

//! Test utilities.


use byteorder::{ByteOrder, LittleEndian};

use capnp::Word;
use std::cmp;
use std::io::{self, Read, Write};

/// Writes segments as if they were a Capnproto message.
///
/// This is copied from capnproto-rust, and exists that our read/write format
/// does not differ from the 'canonical' capnp-rust.
pub fn write_message_segments<W>(write: &mut W, segments: &Vec<Vec<Word>>)
where
    W: Write,
{
    /// Writes a segment table to `write`.
    ///
    /// `segments` must contain at least one segment.
    fn write_segment_table<W>(write: &mut W, segments: &[&[Word]]) -> ::std::io::Result<()>
    where
        W: Write,
    {
        let mut buf: [u8; 8] = [0; 8];
        let segment_count = segments.len();

        // write the first Word, which contains segment_count and the 1st segment length
        <LittleEndian as ByteOrder>::write_u32(&mut buf[0..4], segment_count as u32 - 1);
        <LittleEndian as ByteOrder>::write_u32(&mut buf[4..8], segments[0].len() as u32);
        try!(write.write_all(&buf));

        if segment_count > 1 {
            for i in 1..((segment_count + 1) / 2) {
                // write two segment lengths at a time starting with the second
                // segment through the final full Word
                <LittleEndian as ByteOrder>::write_u32(&mut buf[0..4], segments[i * 2 - 1].len() as u32);
                <LittleEndian as ByteOrder>::write_u32(&mut buf[4..8], segments[i * 2].len() as u32);
                try!(write.write_all(&buf));
            }

            if segment_count % 2 == 0 {
                // write the final Word containing the last segment length and padding
                <LittleEndian as ByteOrder>::write_u32(&mut buf[0..4], segments[segment_count - 1].len() as u32);
                try!((&mut buf[4..8]).write_all(&[0, 0, 0, 0]));
                try!(write.write_all(&buf));
            }
        }
        Ok(())
    }

    /// Writes segments to `write`.
    fn write_segments<W>(write: &mut W, segments: &[&[Word]]) -> ::std::io::Result<()>
    where
        W: Write,
    {
        for segment in segments {
            try!(write.write_all(Word::words_to_bytes(segment)));
        }
        Ok(())
    }

    let borrowed_segments: &[&[Word]] = &segments
        .iter()
        .map(|segment| &segment[..])
        .collect::<Vec<_>>()[..];
    write_segment_table(write, borrowed_segments).unwrap();
    write_segments(write, borrowed_segments).unwrap();
}

/// Wraps a stream and injects artificial blocking.
pub struct BlockingStream<S> {
    /// The wrapped stream
    stream: S,

    /// Number of bytes to process read before blocking
    frequency: usize,

    /// Number of bytes read since last blocking
    read_idx: usize,

    /// Number of bytes written since last blocking
    write_idx: usize,
}

impl<S> BlockingStream<S> {
    pub fn new(stream: S, frequency: usize) -> BlockingStream<S> {
        BlockingStream {
            stream: stream,
            frequency: frequency,
            read_idx: 0,
            write_idx: 0,
        }
    }

    pub fn inner_mut(&mut self) -> &mut S {
        &mut self.stream
    }
}

impl<R> Read for BlockingStream<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.read_idx == 0 {
            self.read_idx = self.frequency;
            Err(io::Error::new(io::ErrorKind::WouldBlock, "BlockingStream"))
        } else {
            let len = cmp::min(self.read_idx, buf.len());
            let bytes_read = try!(self.stream.read(&mut buf[..len]));
            self.read_idx -= bytes_read;
            Ok(bytes_read)
        }
    }
}

impl<S> Write for BlockingStream<S>
where
    S: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.write_idx == 0 {
            self.write_idx = self.frequency;
            Err(io::Error::new(io::ErrorKind::WouldBlock, "BlockingStream"))
        } else {
            let len = cmp::min(self.write_idx, buf.len());
            let bytes_read = try!(self.stream.write(&buf[..len]));
            self.write_idx -= bytes_read;
            Ok(bytes_read)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}
