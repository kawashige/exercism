use std::usize;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Vec<Vec<u16>>,
    i: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: vec![vec![]],
            i: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.i == 10 {
            Err(Error::GameComplete)
        } else if self.not_enough_pins_left(pins) {
            Err(Error::NotEnoughPinsLeft)
        } else {
            self.frames[self.i].push(pins);
            if self.frame_completed() {
                self.i += 1;
                self.frames.push(vec![])
            }
            Ok(())
        }
    }

    fn not_enough_pins_left(&self, pins: u16) -> bool {
        let i = self.i;
        let v = &self.frames[i];
        if i < 9 {
            10 < v.iter().sum::<u16>() + pins
        } else {
            if (v.len() == 1 && v[0] != 10) || (v.len() == 2 && v[1] != 10 && v[0] + v[1] != 10) {
                10 < pins + v.last().unwrap()
            } else {
                10 < pins
            }
        }
    }

    fn frame_completed(&self) -> bool {
        let i = self.i;
        let v = &self.frames[i];
        if i < 9 {
            v.len() == 2 || v[0] == 10
        } else {
            (v.len() == 2 && v[0] != 10 && v[0] + v[1] != 10) || v.len() == 3
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.i != 10 {
            return None;
        }
        let mut sum: u16 = 0;
        for i in 0..10 {
            sum += self.frames[i].iter().sum::<u16>();
            if i < 9 {
                if self.frames[i][0] == 10 {
                    if self.frames[i + 1][0] == 10 {
                        sum += self.frames[i + 1][0];
                        if i < 8 {
                            sum += self.frames[i + 2][0];
                        } else {
                            sum += self.frames[i + 1][1];
                        }
                    } else {
                        sum += self.frames[i + 1].iter().sum::<u16>();
                    }
                } else if self.frames[i].iter().sum::<u16>() == 10 {
                    sum += self.frames[i + 1][0];
                }
            }
        }
        Some(sum)
    }
}
