#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub enum Frame<'a> {
    Begin,
    Open(u16, u16, &'a Frame<'a>),
    Roll(u16, &'a Frame<'a>),
    Spare(u16, &'a Frame<'a>),
    Strike(&'a Frame<'a>),
}

pub struct BowlingGame<'a> {
    frame: Frame<'a>,
}

impl BowlingGame<'_> {
    pub fn new() -> Self {
        BowlingGame {
            frame: Frame::Begin,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match self.frame.roll(pins) {
            Ok(frame) => {
                self.frame = frame;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn score(&self) -> Option<u16> {
        None
    }
}

impl<'a> Frame<'a> {
    pub fn roll(&self, pins: u16) -> Result<Frame, Error> {
        let next_frame: Frame;

        match self {
            Frame::Begin => {
                if pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                } else if pins == 10 {
                    next_frame = Frame::Strike(self)
                } else {
                    next_frame = Frame::Roll(pins, self)
                }
            }
            _ => unimplemented!(),
        }

        Ok(next_frame)
    }
}
