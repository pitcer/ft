use anyhow::{anyhow, Result};

const BELL: u8 = 7;
const BACKSPACE: u8 = 8;
const ESCAPE: u8 = 27;

#[derive(Debug)]
pub enum ParserAction {
    InsertCharacter(char),
    CarriageReturn,
    NewLine,
    MoveCursorUp(u32),
    MoveCursorDown(u32),
    MoveCursorForward(u32),
    MoveCursorBack(u32),
    MoveCursorToNextMultipleOf(u32),
    EnableBracketedPasteMode,
    DisableBracketedPasteMode,
    Clear,
    MoreBytes,
    Ignore,
    UnsupportedSequence,
}

#[derive(Debug)]
pub enum ParserState {
    Empty,
    Escape,
    Csi,
    CsiPrivate,
}

#[derive(Debug)]
pub struct Parser {
    state: ParserState,
    buffer: Vec<u8>,
    numbers: Vec<u32>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            state: ParserState::Empty,
            buffer: Vec::with_capacity(16),
            numbers: Vec::with_capacity(4),
        }
    }

    pub fn push_byte(&mut self, byte: u8) -> Result<ParserAction> {
        let action = match self.state {
            ParserState::Empty => self.parse_empty(byte),
            ParserState::Escape => self.parse_escape(byte),
            ParserState::Csi => self.parse_csi(byte),
            ParserState::CsiPrivate => self.parse_csi_private(byte),
        }?;
        if !matches!(action, ParserAction::MoreBytes) {
            self.state = ParserState::Empty;
        }
        Ok(action)
    }

    fn parse_empty(&mut self, byte: u8) -> Result<ParserAction> {
        Ok(match byte {
            BELL => ParserAction::Ignore,
            BACKSPACE => ParserAction::MoveCursorBack(1),
            b'\r' => ParserAction::CarriageReturn,
            b'\n' => ParserAction::NewLine,
            b'\t' => ParserAction::MoveCursorToNextMultipleOf(8),
            ESCAPE => {
                self.state = ParserState::Escape;
                ParserAction::MoreBytes
            }
            _ => ParserAction::InsertCharacter(byte as char),
        })
    }

    fn parse_escape(&mut self, byte: u8) -> Result<ParserAction> {
        Ok(match byte {
            b'[' => {
                self.state = ParserState::Csi;
                ParserAction::MoreBytes
            }
            _ => ParserAction::UnsupportedSequence,
        })
    }

    fn parse_csi(&mut self, byte: u8) -> Result<ParserAction> {
        Ok(match byte {
            b'?' => {
                self.state = ParserState::CsiPrivate;
                ParserAction::MoreBytes
            }
            b'0'..=b'9' => {
                self.buffer.push(byte);
                ParserAction::MoreBytes
            }
            b';' => {
                self.push_number()?;
                ParserAction::MoreBytes
            }
            _ => {
                self.push_number()?;
                match byte {
                    b'A' => {
                        if let Some(number) = self.numbers.last().copied() {
                            self.numbers.clear();
                            ParserAction::MoveCursorUp(number)
                        } else {
                            ParserAction::MoveCursorUp(1)
                        }
                    }
                    b'B' => {
                        if let Some(number) = self.numbers.last().copied() {
                            self.numbers.clear();
                            ParserAction::MoveCursorDown(number)
                        } else {
                            ParserAction::MoveCursorDown(1)
                        }
                    }
                    b'C' => {
                        if let Some(number) = self.numbers.last().copied() {
                            self.numbers.clear();
                            ParserAction::MoveCursorForward(number)
                        } else {
                            ParserAction::MoveCursorForward(1)
                        }
                    }
                    b'D' => {
                        if let Some(number) = self.numbers.last().copied() {
                            self.numbers.clear();
                            ParserAction::MoveCursorBack(number)
                        } else {
                            ParserAction::MoveCursorBack(1)
                        }
                    }
                    b'J' => {
                        if let Some(3) = self.numbers.last().copied() {
                            self.numbers.clear();
                            ParserAction::Clear
                        } else {
                            ParserAction::UnsupportedSequence
                        }
                    }
                    _ => ParserAction::UnsupportedSequence,
                }
            }
        })
    }

    fn parse_csi_private(&mut self, byte: u8) -> Result<ParserAction> {
        Ok(match byte {
            b'0'..=b'9' => {
                self.buffer.push(byte);
                ParserAction::MoreBytes
            }
            b'h' => {
                let number = self.parse_number()?;
                match number {
                    Some(2004) => ParserAction::EnableBracketedPasteMode,
                    _ => ParserAction::UnsupportedSequence,
                }
            }
            b'l' => {
                let number = self.parse_number()?;
                match number {
                    Some(2004) => ParserAction::DisableBracketedPasteMode,
                    _ => ParserAction::UnsupportedSequence,
                }
            }
            _ => ParserAction::UnsupportedSequence,
        })
    }

    fn push_number(&mut self) -> Result<()> {
        let number = self.parse_number()?;
        if let Some(number) = number {
            self.numbers.push(number);
        }
        Ok(())
    }

    fn parse_number(&mut self) -> Result<Option<u32>> {
        if self.buffer.is_empty() {
            return Ok(None);
        }
        let number = atoi::atoi(&self.buffer)
            .ok_or_else(|| anyhow!("Cannot parse number in escape sequence"))?;
        self.buffer.clear();
        Ok(Some(number))
    }
}
