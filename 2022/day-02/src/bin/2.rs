fn main() {
    let input = include_str!("../../input.txt");
    let output = play(input);
    dbg!(output);
}

#[derive(Clone, Debug, Copy, PartialEq)]
enum Hand {
    R,
    P,
    S,
}

impl Hand {
    const HANDS: [Hand; 3] = [Self::R, Self::P, Self::S];

    fn index(&self) -> usize {
        match self {
            Self::R => 0,
            Self::P => 1,
            Self::S => 2,
        }
    }
    fn points(&self) -> u32 {
        (self.index() + 1) as u32
    }
    /// Do a match against another hand and return the points
    fn match_against(&self, enemy: &Hand) -> u32 {
        let ei = enemy.index() as isize;
        let si = self.index() as isize;
        let diff = (si - if ei < si { ei + 3 } else { ei }).abs();
        (match diff {
            1 => 0,
            0 => 3,
            _ => 6,
        } + self.points())
    }
    /// Return another Hand given the end result of a match
    fn from_match_result(&self, match_result: &str) -> Self {
        match match_result {
            "Y" => *self,
            "Z" => self.get_beater(),
            _ => self.get_beaten(),
        }
    }
    /// Return the beater of self, which hand wins against self
    fn get_beater(&self) -> Self {
        Self::HANDS[(self.index() + 1) % Self::HANDS.len()]
    }
    /// Return the hand beaten (lost) by self
    fn get_beaten(&self) -> Self {
        Self::HANDS[((if self.index() == 0 { 2 } else { self.index() - 1 })
            .max(0))
            % Self::HANDS.len()]
    }
}

impl TryFrom<&str> for Hand {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Self::R),
            "B" => Ok(Self::P),
            "C" => Ok(Self::S),
            _ => Err(format!("Invalid char {value}")),
        }
    }
}

/// play rock paper and scissors
fn play(input: &str) -> u32 {
    let mut score: u32 = 0;

    for line in input.lines() {
        let enemy: Hand = line[0..1].try_into().unwrap();
        let me = enemy.from_match_result(&line[2..3]);
        score += me.match_against(&enemy);
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let output = play(
            "A Y
B X
C Z",
        );
        assert_eq!(output, 12);
    }
}
