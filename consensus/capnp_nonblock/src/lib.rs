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

//! `capnp_nonblock` provides a helper struct, `MessageStream`, for reading and
//! writing [Cap'n Proto](https://capnproto.org/) messages to non-blocking
//! streams.

#![feature(alloc, allocator_api)]
extern crate alloc;
extern crate byteorder;
extern crate capnp;

#[cfg(test)]
extern crate quickcheck;

mod buf;

#[cfg(test)]
mod test_utils;


use buf::{MutBuf, Buf};

use byteorder::{ByteOrder, LittleEndian};
use capnp::Word;
use capnp::message::{Allocator, Builder, HeapAllocator, Reader, ReaderOptions, ReaderSegments};
use std::borrow::Borrow;
use std::collections::VecDeque;
use std::fmt;
use std::io::{self, Error, ErrorKind, Result};
use std::marker;
use std::mem;
use std::result;

/// A Cap'n Proto message container.
pub struct Segments {
    segments: Vec<Buf>,
}

impl ReaderSegments for Segments {
    fn get_segment(&self, id: u32) -> Option<&[Word]> {
        self.segments.get(id as usize).map(|buf| Word::bytes_to_words(&*buf))
    }
}

/// A `MessageStream` wraps a stream, and provides methods to read and write
/// Cap'n Proto messages to the stream. `MessageStream` performs its own
/// internal buffering, so the provided stream need not be buffered.
///
/// If the underlying stream is non-blocking, `MessageStream` will automatically
/// pause reading and writing messages, and will resume during the next call to
/// `read_message` or `write`.
///
/// `MessageStream` attempts to reduce the number of required allocations when
/// reading messages by allocating memory in large chunks, which it loans out to
/// messages via reference counting. The reference counting is not thread safe,
/// so messages read by `MessageStream` may not be sent or shared across thread
/// boundaries.
pub struct MessageStream<S, A = HeapAllocator, M = Builder<A>> {
    inner: S,
    options: ReaderOptions,

    /// The current read buffer.
    buf: MutBuf,
    /// The current read offset.
    buf_offset: usize,
    /// The segment sizes of the remaining segments of message currently being
    /// read, in reverse order.
    remaining_segments: Vec<usize>,
    /// The segments of the message currently being read.
    segments: Vec<Buf>,

    /// Queue of outbound messages which have not yet begun being written to the
    /// stream.
    outbound_queue: VecDeque<M>,

    /// The serialized segment table of the message currently being written to
    /// the stream.
    current_segment_table: Vec<u8>,

    /// The progress of the current write. The message currently being written
    /// is a the front of the outbound queue.
    ///
    /// The first corresponds to the segment currently being written, offset by
    /// 1, or 0 if the segment table is being written. The second corresponds to
    /// the offset within the current segment.
    write_progress: Option<(usize, usize)>,

    marker_: marker::PhantomData<A>,
}

impl<S, M, A> MessageStream<S, M, A> {
    /// Creates a new `MessageStream` instance wrapping the provided stream, and
    /// with the provided reader options.
    pub fn new(inner: S, options: ReaderOptions) -> MessageStream<S, M, A> {
        MessageStream {
            inner: inner,
            options: options,
            buf: MutBuf::new(),
            buf_offset: 0,
            remaining_segments: Vec::new(),
            segments: Vec::new(),
            outbound_queue: VecDeque::new(),
            current_segment_table: Vec::new(),
            write_progress: None,
            marker_: marker::PhantomData,
        }
    }

    /// Returns the number of queued outbound messages.
    pub fn outbound_queue_len(&self) -> usize {
        self.outbound_queue.len()
    }

    /// Clears the outbound message queue of all messages that have not begun
    /// writing yet.
    pub fn clear_outbound_queue(&mut self) {
        if self.write_progress.is_some() {
            self.outbound_queue.drain(1..);
        } else {
            self.outbound_queue.clear();
        }
    }

    /// Returns the inner stream.
    pub fn inner_mut(&mut self) -> &mut S {
        &mut self.inner
    }

    /// Returns the inner stream.
    pub fn inner(&self) -> &S {
        &self.inner
    }
}

impl<S, M, A> MessageStream<S, M, A>
where
    S: io::Read,
{
    /// Reads the segment table, populating the `remaining_segments` field of the
    /// reader on success.
    fn read_segment_table(&mut self) -> io::Result<()> {
        let MessageStream {
            ref mut inner,
            ref options,
            ref mut buf,
            ref mut buf_offset,
            ref mut remaining_segments,
            ..
        } = *self;

        loop {
            assert!(remaining_segments.is_empty());
            match parse_segment_table(&buf[*buf_offset..], remaining_segments) {
                Ok(0) => break,
                Ok(n) => try!(buf.fill_or_replace(inner, buf_offset, n)),
                Err(error) => return Err(error),
            }
        }

        *buf_offset += (remaining_segments.len() / 2 + 1) * 8;

        let total_len = remaining_segments.iter().fold(Some(0u64), |acc, &len| acc.and_then(|n| n.checked_add(len as u64)));
        match total_len {
            Some(len) if len <= options.traversal_limit_in_words * 8 => (),
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "Cap'n Proto message is too large".to_string())),
        }

        remaining_segments.reverse();
        Ok(())
    }

    /// Reads a message segment from the stream.
    fn read_segment(&mut self, len: usize) -> Result<Buf> {
        let MessageStream {
            ref mut inner,
            ref mut buf,
            ref mut buf_offset,
            ..
        } = *self;
        try!(buf.fill_or_replace(inner, buf_offset, len));
        let buf = buf.buf(*buf_offset, len);
        *buf_offset += len;
        Ok(buf)
    }

    /// Reads a message from the stream.
    fn read(&mut self) -> io::Result<Reader<Segments>> {
        if self.remaining_segments.is_empty() {
            try!(self.read_segment_table());
        }

        while let Some(&segment_len) = self.remaining_segments.last() {
            let segment = try!(self.read_segment(segment_len));
            self.segments.push(segment);
            // Only pop the segment length once we know there hasn't been an error.
            self.remaining_segments.pop();
        }


        Ok(Reader::new(Segments { segments: mem::replace(&mut self.segments, Vec::new()) }, self.options.clone()))
    }

    /// Returns the next message from the stream, or `None` if the entire
    /// message is not yet available.
    ///
    /// If an `Err` result is returned, then the stream must be considered
    /// corrupt, and `read_message` must not be called again.
    pub fn read_message(&mut self) -> Result<Option<Reader<Segments>>> {
        match self.read() {
            Err(ref error) if error.kind() == io::ErrorKind::WouldBlock => Ok(None),
            Err(error) => Err(From::from(error)),
            Ok(message) => Ok(Some(message)),
        }
    }
}

impl<S, A, M> fmt::Debug for MessageStream<S, A, M>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MessageStream {{ inner: {:?}, outbound_messages: {} }}", self.inner, self.outbound_queue_len())
    }
}

/// Serializes the segment table for the provided segments.
fn serialize_segment_table(segment_table: &mut Vec<u8>, segments: &[&[Word]]) {
    segment_table.clear();

    let mut buf: [u8; 4] = [0; 4];

    <LittleEndian as ByteOrder>::write_u32(&mut buf[..], segments.len() as u32 - 1);
    segment_table.extend(&buf);

    for segment in segments {
        <LittleEndian as ByteOrder>::write_u32(&mut buf[..], segment.len() as u32);
        segment_table.extend(&buf);
    }

    if segments.len() % 2 == 0 {
        segment_table.extend(&[0, 0, 0, 0]);
    }
}

/// Like Write::write_all, but increments `offset` after every successful
/// write.
fn write_segment<W>(write: &mut W, mut buf: &[u8], offset: &mut usize) -> io::Result<()>
where
    W: io::Write,
{
    while !buf.is_empty() {
        match write.write(buf) {
            Ok(0) => return result::Result::Err(io::Error::new(io::ErrorKind::WriteZero, "failed to write whole message")),
            Ok(n) => {
                *offset += n;
                buf = &buf[n..]
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn write_message<W>(write: &mut W, segment_table: &[u8], segments: &[&[Word]], write_progress: &mut (usize, usize)) -> io::Result<()>
where
    W: io::Write,
{
    let (ref mut segment_index, ref mut segment_offset) = *write_progress;

    if *segment_index == 0 {
        try!(write_segment(write, &segment_table[*segment_offset..], segment_offset));
        *segment_offset = 0;
        *segment_index += 1;
    }

    for segment in &segments[(*segment_index - 1)..] {
        try!(write_segment(write, &Word::words_to_bytes(segment)[*segment_offset..], segment_offset));
        *segment_offset = 0;
        *segment_index += 1;
    }
    Ok(())
}

impl<S, A, M> MessageStream<S, A, M>
where
    S: io::Write,
    M: Borrow<Builder<A>>,
    A: Allocator,
{
    /// Writes queued messages to the stream. This should be called when the
    /// stream is in non-blocking mode and writable.
    ///
    /// If an `Err` result is returned, then the stream must be considered
    /// corrupt, and `write` or `write_message` must not be called again.
    pub fn write(&mut self) -> io::Result<()> {

        let MessageStream {
            ref mut inner,
            ref mut outbound_queue,
            ref mut current_segment_table,
            ref mut write_progress,
            ..
        } = *self;

        loop {
            {
                let message: &Builder<A> = match outbound_queue.front() {
                    Some(message) => message.borrow(),
                    None => return Ok(()),
                };

                *write_progress = write_progress.or_else(|| {
                                                             serialize_segment_table(current_segment_table, &*message.get_segments_for_output());
                                                             Some((0, 0))
                                                         });

                let progress: &mut (usize, usize) = write_progress.as_mut().unwrap();
                let segments = &*message.get_segments_for_output();

                match write_message(inner, current_segment_table, segments, progress) {
                    Err(ref error) if error.kind() == io::ErrorKind::WouldBlock => return Ok(()),
                    Ok(_) => (),
                    error => return error,
                }
            }
            outbound_queue.pop_front();
            *write_progress = None;
        }
    }

    /// Queue message for write.
    ///
    /// This method optimistically begins writing to the stream if there is no
    /// message currently being written. This is necessary for the blocking
    /// stream case, and efficient in the non-blocking case as well, since it is
    /// likely that the stream is writable.
    ///
    /// If an `Err` result is returned, then the stream must be considered
    /// corrupt, and `write` or `write_message` must not be called again.
    pub fn write_message(&mut self, message: M) -> io::Result<()> {
        self.outbound_queue.push_back(message);

        if self.outbound_queue_len() == 1 {
            // Swallow NotConnected error when aggressively writing. OS X will
            // return NotConnected when writing to a freshly opened non-blocking
            // socket; see hoverbear/raft#61.
            match self.write() {
                Err(ref error) if error.kind() == io::ErrorKind::NotConnected => Ok(()),
                other => other,
            }
        } else {
            Ok(())
        }
    }
}

/// Parses a segment table into a sequence of segment lengths, and adds the
/// lengths to the provided `Vec`.
///
/// Returns 0 if the parse succeeded, otherwise returns the number of bytes
/// required to make progress with the parse.
fn parse_segment_table(buf: &[u8], lengths: &mut Vec<usize>) -> Result<usize> {
    if buf.len() < 8 {
        return Ok(8);
    }
    let segment_count = <LittleEndian as ByteOrder>::read_u32(&buf[0..4]).wrapping_add(1) as usize;

    if segment_count >= 512 {
        return result::Result::Err(Error::new(ErrorKind::InvalidData, format!("too many segments in Cap'n Proto message: {}", segment_count)));
    } else if segment_count == 0 {
        return result::Result::Err(Error::new(ErrorKind::InvalidData, "zero segments Cap'n Proto message".to_string()));
    }

    let len = (segment_count / 2 + 1) * 8;
    if buf.len() < len {
        return Ok(len);
    }

    for segment in 0..segment_count {
        let offset = (segment + 1) * 4;
        let segment_len = <LittleEndian as ByteOrder>::read_u32(&buf[offset..]) as usize;
        lengths.push(segment_len * 8);
    }

    Ok(0)
}

#[cfg(test)]
pub mod test {

    use super::{MessageStream, parse_segment_table, serialize_segment_table, write_message};

    use capnp::{Word, message};
    use capnp::message::ReaderSegments;
    use quickcheck::{quickcheck, TestResult};

    use std::io::{self, Cursor, Write};

    use test_utils;

    #[test]
    fn test_parse_segment_table() {
        fn compare(expected: &[usize], buf: &[u8]) {
            let mut actual = Vec::new();
            assert_eq!(0, parse_segment_table(buf, &mut actual).unwrap());
            assert_eq!(expected, &*actual);
        }

        compare(&[0 * 8],
                &[0,0,0,0,   // 1 segments
                  0,0,0,0]); // 0 words

        compare(&[1 * 8],
                &[0,0,0,0,   // 1 segments
                  1,0,0,0]); // 1 word

        compare(
            &[1 * 8, 1 * 8],
            &[
                1,
                0,
                0,
                0, // 2 segments
                1,
                0,
                0,
                0, // 1 word
                1,
                0,
                0,
                0, // 1 word
                0,
                0,
                0,
                0,
            ],
        ); // padding

        compare(
            &[1 * 8, 1 * 8, 256 * 8],
            &[
                2,
                0,
                0,
                0, // 3 segments
                1,
                0,
                0,
                0, // 1 word
                1,
                0,
                0,
                0, // 1 word
                0,
                1,
                0,
                0,
            ],
        ); // 256 length

        compare(
            &[77 * 8, 23 * 8, 1 * 8, 99 * 8],
            &[
                3,
                0,
                0,
                0, // 4 segments
                77,
                0,
                0,
                0, // 77 word
                23,
                0,
                0,
                0, // 23 words
                1,
                0,
                0,
                0, // 1 word
                99,
                0,
                0,
                0, // 99 words
                0,
                0,
                0,
                0,
            ],
        ); // padding
    }

    #[test]
    fn test_parse_invalid_segment_table() {
        let mut v = Vec::new();
        assert!(parse_segment_table(&[255, 1, 0, 0, 0, 0, 0, 0], &mut v).is_err());
        assert_eq!(8, parse_segment_table(&[0, 0, 0, 0], &mut v).unwrap());
        assert_eq!(8, parse_segment_table(&[0, 0, 0, 0, 0, 0, 0], &mut v).unwrap());
        assert_eq!(16, parse_segment_table(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], &mut v).unwrap());
        assert!(parse_segment_table(&[255, 255, 255, 255, 0, 0, 0, 0], &mut v).is_err());
    }

    #[test]
    fn check_read_segments() {
        fn read_segments(segments: Vec<Vec<Word>>) -> TestResult {
            if segments.len() == 0 {
                return TestResult::discard();
            }
            let mut cursor = Cursor::new(Vec::new());

            test_utils::write_message_segments(&mut cursor, &segments);
            cursor.set_position(0);

            let mut message_reader = MessageStream::<_, (), ()>::new(&mut cursor, message::ReaderOptions::new());
            let message = message_reader.read_message().unwrap().unwrap();
            let result_segments = message.into_segments();

            TestResult::from_bool(segments.iter()
                                          .enumerate()
                                          .all(|(i, segment)| &segment[..] == result_segments.get_segment(i as u32).unwrap()))
        }

        quickcheck(read_segments as fn(Vec<Vec<Word>>) -> TestResult);
    }

    /// Equivalent to `MessageStream::write`, but works on raw segments instead
    /// of message objects, and automatically retries on `WouldBlock`.
    fn write_message_segments<W>(write: &mut W, segments: &Vec<Vec<Word>>)
    where
        W: Write,
    {
        let segments: &[&[Word]] = &segments.iter().map(|segment| &segment[..]).collect::<Vec<_>>()[..];
        let mut segment_table = Vec::new();
        serialize_segment_table(&mut segment_table, segments);
        let mut write_progress = (0, 0);

        loop {
            match write_message(write, &segment_table, segments, &mut write_progress) {
                Err(ref error) if error.kind() == io::ErrorKind::WouldBlock => continue,
                other => {
                    other.unwrap();
                    return;
                }
            }
        }
    }

    #[test]
    fn check_write_segments() {
        fn write_segments(segments: Vec<Vec<Word>>) -> TestResult {
            if segments.len() == 0 {
                return TestResult::discard();
            }
            let mut cursor = Cursor::new(Vec::new());
            let mut expected_cursor = Cursor::new(Vec::new());

            test_utils::write_message_segments(&mut expected_cursor, &segments);
            expected_cursor.set_position(0);

            write_message_segments(&mut cursor, &segments);

            TestResult::from_bool(expected_cursor.into_inner() == cursor.into_inner())
        }

        quickcheck(write_segments as fn(Vec<Vec<Word>>) -> TestResult);
    }

    #[test]
    fn check_round_trip() {
        fn round_trip(messages: Vec<Vec<Vec<Word>>>) -> TestResult {
            let mut cursor = Cursor::new(Vec::new());

            for segments in &messages {
                if segments.len() == 0 {
                    return TestResult::discard();
                }
                write_message_segments(&mut cursor, segments);
            }
            cursor.set_position(0);

            let mut message_reader = MessageStream::<_, (), ()>::new(&mut cursor, message::ReaderOptions::new());

            for segments in &messages {
                let message = message_reader.read_message().unwrap().unwrap();
                let result_segments = message.into_segments();
                for (i, segment) in segments.into_iter().enumerate() {
                    if &segment[..] != result_segments.get_segment(i as u32).unwrap() {
                        return TestResult::failed();
                    }
                }

            }
            TestResult::passed()
        }

        quickcheck(round_trip as fn(Vec<Vec<Vec<Word>>>) -> TestResult);
    }

    #[test]
    fn check_round_trip_nonblock() {
        fn round_trip_nonblock(messages: Vec<Vec<Vec<Word>>>, frequency: usize) -> TestResult {
            if frequency == 0 {
                return TestResult::discard();
            }
            let mut stream = test_utils::BlockingStream::new(Cursor::new(Vec::new()), frequency);

            for segments in &messages {
                if segments.len() == 0 {
                    return TestResult::discard();
                }
                write_message_segments(&mut stream, segments);
            }
            stream.inner_mut().set_position(0);

            let mut message_reader = MessageStream::<_, (), ()>::new(&mut stream, message::ReaderOptions::new());

            for segments in &messages {
                let mut message = None;
                while let None = message {
                    message = message_reader.read_message().unwrap();
                }
                let result_segments = message.unwrap().into_segments();
                for (i, segment) in segments.into_iter().enumerate() {
                    if &segment[..] != result_segments.get_segment(i as u32).unwrap() {
                        return TestResult::failed();
                    }
                }

            }
            TestResult::passed()
        }

        quickcheck(round_trip_nonblock as fn(Vec<Vec<Vec<Word>>>, usize) -> TestResult);
    }
}
