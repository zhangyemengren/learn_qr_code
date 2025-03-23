use std::slice::Iter;
use crate::types::{Mode, Version};

static STATE_TRANSITION: [(State, Action); 70] = [
    // STATE_TRANSITION[current_state + next_character] == (next_state, what_to_do)

    // Init state:
    (State::Init, Action::Idle),      // End
    (State::Alpha, Action::Idle),     // Symbol
    (State::Numeric, Action::Idle),   // Numeric
    (State::Alpha, Action::Idle),     // Alpha
    (State::KanjiHi12, Action::Idle), // KanjiHi1
    (State::KanjiHi12, Action::Idle), // KanjiHi2
    (State::KanjiHi3, Action::Idle),  // KanjiHi3
    (State::Byte, Action::Idle),      // KanjiLo1
    (State::Byte, Action::Idle),      // KanjiLo2
    (State::Byte, Action::Idle),      // Byte
    // Numeric state:
    (State::Init, Action::Numeric),      // End
    (State::Alpha, Action::Numeric),     // Symbol
    (State::Numeric, Action::Idle),      // Numeric
    (State::Alpha, Action::Numeric),     // Alpha
    (State::KanjiHi12, Action::Numeric), // KanjiHi1
    (State::KanjiHi12, Action::Numeric), // KanjiHi2
    (State::KanjiHi3, Action::Numeric),  // KanjiHi3
    (State::Byte, Action::Numeric),      // KanjiLo1
    (State::Byte, Action::Numeric),      // KanjiLo2
    (State::Byte, Action::Numeric),      // Byte
    // Alpha state:
    (State::Init, Action::Alpha),      // End
    (State::Alpha, Action::Idle),      // Symbol
    (State::Numeric, Action::Alpha),   // Numeric
    (State::Alpha, Action::Idle),      // Alpha
    (State::KanjiHi12, Action::Alpha), // KanjiHi1
    (State::KanjiHi12, Action::Alpha), // KanjiHi2
    (State::KanjiHi3, Action::Alpha),  // KanjiHi3
    (State::Byte, Action::Alpha),      // KanjiLo1
    (State::Byte, Action::Alpha),      // KanjiLo2
    (State::Byte, Action::Alpha),      // Byte
    // Byte state:
    (State::Init, Action::Byte),      // End
    (State::Alpha, Action::Byte),     // Symbol
    (State::Numeric, Action::Byte),   // Numeric
    (State::Alpha, Action::Byte),     // Alpha
    (State::KanjiHi12, Action::Byte), // KanjiHi1
    (State::KanjiHi12, Action::Byte), // KanjiHi2
    (State::KanjiHi3, Action::Byte),  // KanjiHi3
    (State::Byte, Action::Idle),      // KanjiLo1
    (State::Byte, Action::Idle),      // KanjiLo2
    (State::Byte, Action::Idle),      // Byte
    // KanjiHi12 state:
    (State::Init, Action::KanjiAndSingleByte),    // End
    (State::Alpha, Action::KanjiAndSingleByte),   // Symbol
    (State::Numeric, Action::KanjiAndSingleByte), // Numeric
    (State::Kanji, Action::Idle),                 // Alpha
    (State::Kanji, Action::Idle),                 // KanjiHi1
    (State::Kanji, Action::Idle),                 // KanjiHi2
    (State::Kanji, Action::Idle),                 // KanjiHi3
    (State::Kanji, Action::Idle),                 // KanjiLo1
    (State::Kanji, Action::Idle),                 // KanjiLo2
    (State::Byte, Action::KanjiAndSingleByte),    // Byte
    // KanjiHi3 state:
    (State::Init, Action::KanjiAndSingleByte),      // End
    (State::Alpha, Action::KanjiAndSingleByte),     // Symbol
    (State::Numeric, Action::KanjiAndSingleByte),   // Numeric
    (State::Kanji, Action::Idle),                   // Alpha
    (State::Kanji, Action::Idle),                   // KanjiHi1
    (State::KanjiHi12, Action::KanjiAndSingleByte), // KanjiHi2
    (State::KanjiHi3, Action::KanjiAndSingleByte),  // KanjiHi3
    (State::Kanji, Action::Idle),                   // KanjiLo1
    (State::Byte, Action::KanjiAndSingleByte),      // KanjiLo2
    (State::Byte, Action::KanjiAndSingleByte),      // Byte
    // Kanji state:
    (State::Init, Action::Kanji),     // End
    (State::Alpha, Action::Kanji),    // Symbol
    (State::Numeric, Action::Kanji),  // Numeric
    (State::Alpha, Action::Kanji),    // Alpha
    (State::KanjiHi12, Action::Idle), // KanjiHi1
    (State::KanjiHi12, Action::Idle), // KanjiHi2
    (State::KanjiHi3, Action::Idle),  // KanjiHi3
    (State::Byte, Action::Kanji),     // KanjiLo1
    (State::Byte, Action::Kanji),     // KanjiLo2
    (State::Byte, Action::Kanji),     // Byte
];


pub fn total_encoded_len(segments: &[Segment], version: Version) -> usize {
    segments.iter().map(|seg| seg.encoded_len(version)).sum()
}

#[derive(Copy, Clone)]
enum Action {
    /// The parser should do nothing.
    Idle,

    /// Push the current segment as a Numeric string, and reset the marks.
    Numeric,

    /// Push the current segment as an Alphanumeric string, and reset the marks.
    Alpha,

    /// Push the current segment as a 8-Bit Byte string, and reset the marks.
    Byte,

    /// Push the current segment as a Kanji string, and reset the marks.
    Kanji,

    /// Push the current segment excluding the last byte as a Kanji string, then
    /// push the remaining single byte as a Byte string, and reset the marks.
    KanjiAndSingleByte,
}

#[derive(Copy, Clone)]
enum ExclCharSet {
    /// The end of string.
    End = 0,

    /// All symbols supported by the Alphanumeric encoding, i.e. space, `$`, `%`,
    /// `*`, `+`, `-`, `.`, `/` and `:`.
    Symbol = 1,

    /// All numbers (0–9).
    Numeric = 2,

    /// All uppercase letters (A–Z). These characters may also appear in the
    /// second byte of a Shift JIS 2-byte encoding.
    Alpha = 3,

    /// The first byte of a Shift JIS 2-byte encoding, in the range 0x81–0x9f.
    KanjiHi1 = 4,

    /// The first byte of a Shift JIS 2-byte encoding, in the range 0xe0–0xea.
    KanjiHi2 = 5,

    /// The first byte of a Shift JIS 2-byte encoding, of value 0xeb. This is
    /// different from the other two range that the second byte has a smaller
    /// range.
    KanjiHi3 = 6,

    /// The second byte of a Shift JIS 2-byte encoding, in the range 0x40–0xbf,
    /// excluding letters (covered by `Alpha`), 0x81–0x9f (covered by `KanjiHi1`),
    /// and the invalid byte 0x7f.
    KanjiLo1 = 7,

    /// The second byte of a Shift JIS 2-byte encoding, in the range 0xc0–0xfc,
    /// excluding the range 0xe0–0xeb (covered by `KanjiHi2` and `KanjiHi3`).
    /// This half of byte-pair cannot appear as the second byte leaded by
    /// `KanjiHi3`.
    KanjiLo2 = 8,

    /// Any other values not covered by the above character sets.
    Byte = 9,
}

impl ExclCharSet {
    /// Determines which character set a byte is in.
    const fn from_u8(c: u8) -> Self {
        match c {
            0x20 | 0x24 | 0x25 | 0x2a | 0x2b | 0x2d..=0x2f | 0x3a => Self::Symbol,
            0x30..=0x39 => Self::Numeric,
            0x41..=0x5a => Self::Alpha,
            0x81..=0x9f => Self::KanjiHi1,
            0xe0..=0xea => Self::KanjiHi2,
            0xeb => Self::KanjiHi3,
            0x40 | 0x5b..=0x7e | 0x80 | 0xa0..=0xbf => Self::KanjiLo1,
            0xc0..=0xdf | 0xec..=0xfc => Self::KanjiLo2,
            _ => Self::Byte,
        }
    }
}

#[derive(Copy, Clone)]
enum State {
    /// Just initialized.
    Init = 0,

    /// Inside a string that can be exclusively encoded as Numeric.
    Numeric = 10,

    /// Inside a string that can be exclusively encoded as Alphanumeric.
    Alpha = 20,

    /// Inside a string that can be exclusively encoded as 8-Bit Byte.
    Byte = 30,

    /// Just encountered the first byte of a Shift JIS 2-byte sequence of the
    /// set `KanjiHi1` or `KanjiHi2`.
    KanjiHi12 = 40,

    /// Just encountered the first byte of a Shift JIS 2-byte sequence of the
    /// set `KanjiHi3`.
    KanjiHi3 = 50,

    /// Inside a string that can be exclusively encoded as Kanji.
    Kanji = 60,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Segment {
    /// The encoding mode of the segment of data.
    pub mode: Mode,

    /// The start index of the segment.
    pub begin: usize,

    /// The end index (exclusive) of the segment.
    pub end: usize,
}

impl Segment {
    pub fn encoded_len(&self, version: Version) -> usize {
        let byte_size = self.end - self.begin;
        let chars_count = if self.mode == Mode::Kanji {
            byte_size / 2
        } else {
            byte_size
        };

        let mode_bits_count = version.mode_bits_count();
        let length_bits_count = self.mode.length_bits_count(version);
        let data_bits_count = self.mode.data_bits_count(chars_count);

        mode_bits_count + length_bits_count + data_bits_count
    }
}

struct EcsIter<I> {
    base: I,
    index: usize,
    ended: bool,
}

impl<'a, I: Iterator<Item = &'a u8>> Iterator for EcsIter<I> {
    type Item = (usize, ExclCharSet);

    fn next(&mut self) -> Option<(usize, ExclCharSet)> {
        if self.ended {
            return None;
        }

        match self.base.next() {
            None => {
                self.ended = true;
                Some((self.index, ExclCharSet::End))
            }
            Some(c) => {
                let old_index = self.index;
                self.index += 1;
                Some((old_index, ExclCharSet::from_u8(*c)))
            }
        }
    }
}

pub struct Parser<'a> {
    ecs_iter: EcsIter<Iter<'a, u8>>,
    state: State,
    begin: usize,
    pending_single_byte: bool,
}

impl<'a> Parser<'a> {
    pub fn new(data: &[u8]) -> Parser {
        Parser {
            ecs_iter: EcsIter {
                base: data.iter(),
                index: 0,
                ended: false,
            },
            state: State::Init,
            begin: 0,
            pending_single_byte: false,
        }
    }
    pub fn optimize(self, version: Version) -> Optimizer<Self> {
        Optimizer::new(self, version)
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Segment;

    fn next(&mut self) -> Option<Segment> {
        if self.pending_single_byte {
            self.pending_single_byte = false;
            self.begin += 1;
            return Some(Segment {
                mode: Mode::Byte,
                begin: self.begin - 1,
                end: self.begin,
            });
        }

        loop {
            let (i, ecs) = match self.ecs_iter.next() {
                None => return None,
                Some(a) => a,
            };
            let (next_state, action) = STATE_TRANSITION[self.state as usize + ecs as usize];
            self.state = next_state;

            let old_begin = self.begin;
            let push_mode = match action {
                Action::Idle => continue,
                Action::Numeric => Mode::Numeric,
                Action::Alpha => Mode::Alphanumeric,
                Action::Byte => Mode::Byte,
                Action::Kanji => Mode::Kanji,
                Action::KanjiAndSingleByte => {
                    let next_begin = i - 1;
                    if self.begin == next_begin {
                        Mode::Byte
                    } else {
                        self.pending_single_byte = true;
                        self.begin = next_begin;
                        return Some(Segment {
                            mode: Mode::Kanji,
                            begin: old_begin,
                            end: next_begin,
                        });
                    }
                }
            };

            self.begin = i;
            return Some(Segment {
                mode: push_mode,
                begin: old_begin,
                end: i,
            });
        }
    }
}

pub struct Optimizer<I> {
    parser: I,
    last_segment: Segment,
    last_segment_size: usize,
    version: Version,
    ended: bool,
}

impl<I: Iterator<Item = Segment>> Optimizer<I> {
    pub fn new(mut segments: I, version: Version) -> Self {
        match segments.next() {
            None => Self {
                parser: segments,
                last_segment: Segment {
                    mode: Mode::Numeric,
                    begin: 0,
                    end: 0,
                },
                last_segment_size: 0,
                version,
                ended: true,
            },
            Some(segment) => Self {
                parser: segments,
                last_segment: segment,
                last_segment_size: segment.encoded_len(version),
                version,
                ended: false,
            },
        }
    }
}

impl<I: Iterator<Item = Segment>> Iterator for Optimizer<I> {
    type Item = Segment;

    fn next(&mut self) -> Option<Segment> {
        if self.ended {
            return None;
        }

        loop {
            match self.parser.next() {
                None => {
                    self.ended = true;
                    return Some(self.last_segment);
                }
                Some(segment) => {
                    let seg_size = segment.encoded_len(self.version);

                    let new_segment = Segment {
                        mode: self.last_segment.mode.max(segment.mode),
                        begin: self.last_segment.begin,
                        end: segment.end,
                    };
                    let new_size = new_segment.encoded_len(self.version);

                    if self.last_segment_size + seg_size >= new_size {
                        self.last_segment = new_segment;
                        self.last_segment_size = new_size;
                    } else {
                        let old_segment = self.last_segment;
                        self.last_segment = segment;
                        self.last_segment_size = seg_size;
                        return Some(old_segment);
                    }
                }
            }
        }
    }
}