use crate::solutions::Solution;

#[derive(Debug)]
struct Die(i64, i64);

impl Die {
    fn init() -> Self {
        Die(1, 0)
    }
    fn roll(&mut self) -> i64 {
        let n = self.0;
        self.0 += 1;
        if self.0 == 101 {
            self.0 = 1;
        }

        self.1 += 1;

        n
    }

    fn roll_triple(&mut self) -> i64 {
        self.roll() + self.roll() + self.roll()
    }
}

#[derive(Debug)]
struct State {
    die: Die,
    player_1_pos: i64,
    player_2_pos: i64,
    player_1_score: i64,
    player_2_score: i64,
}

impl State {
    fn init(pos: (i64, i64)) -> Self {
        Self {
            die: Die::init(),
            player_1_pos: pos.0,
            player_2_pos: pos.1,
            player_1_score: 0,
            player_2_score: 0,
        }
    }

    fn play_round(&mut self) {
        Self::update_pos(&mut self.die, &mut self.player_1_pos);
        self.player_1_score += self.player_1_pos;

        if self.player_1_score >= 1000 {
            return;
        }

        Self::update_pos(&mut self.die, &mut self.player_2_pos);
        self.player_2_score += self.player_2_pos;
    }

    fn has_winner(&self) -> bool {
        self.player_1_score >= 1000 || self.player_2_score >= 1000
    }

    fn update_pos(die: &mut Die, pos: &mut i64) {
        *pos += die.roll_triple();

        *pos = ((*pos - 1) % 10) + 1;
    }

    fn calc_result(&self) -> i64 {
        if self.player_1_score >= 100 {
            self.player_2_score * self.die.1
        } else {
            self.player_1_score * self.die.1
        }
    }
}

fn parse_position(line: &str) -> (i64, i64) {
    let re = regex::Regex::new(r"Player (\d*) starting position: (\d*)\n?").unwrap();

    let mut iter = re.captures_iter(line);

    let player1 = iter.next().unwrap()[2].parse().unwrap();
    let player2 = iter.next().unwrap()[2].parse().unwrap();

    (player1, player2)
}

pub fn part_a(file: &str) -> Solution {
    let mut state = State::init(parse_position(file));

    while !state.has_winner() {
        state.play_round();
    }

    Solution::Integer(state.calc_result())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::Integer(739785),
            part_a("Player 1 starting position: 4\nPlayer 2 starting position: 8")
        )
    }

    #[test]
    fn rolls() {
        assert_eq!(6, Die::init().roll_triple())
    }

    #[test]
    fn parses_player_start() {
        assert_eq!(
            (4, 8),
            parse_position("Player 1 starting position: 4\nPlayer 2 starting position: 8")
        );

        assert_eq!(
            (8, 4),
            parse_position("Player 1 starting position: 8\nPlayer 2 starting position: 4")
        );
    }
}
