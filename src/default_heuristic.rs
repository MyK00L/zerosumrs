use crate::game::*;
use crate::heuristic::Heuristic;
use crate::mancala;
use crate::othello;
use crate::tablut;
use crate::tictactoe;

pub struct DefaultHeuristic;

impl Heuristic<tablut::Tablut> for DefaultHeuristic {
	fn eval(g: &tablut::Tablut) -> i64 {
		match g.state() {
			State::Win => i64::MAX - g.turn as i64,
			State::Lose => i64::MIN + g.turn as i64,
			State::Draw => 0,
			State::Going => {
				let nd = g.d.count_ones() as i64;
				let na = g.a.count_ones() as i64;
				let mut km = 0i64;
				let kp = g.k.trailing_zeros();
				let capturer = g.a | tablut::CAPTURE_AID;
				let pass = !(g.a | g.d | tablut::BLOCK);

				let mut i = kp;
				while (pass >> i) & 1 != 0 {
					i += 1;
					km += 1;
				}
				km -= ((capturer >> i) & 1) as i64;

				i = kp + 11;
				while (pass >> i) & 1 != 0 {
					i += 11;
					km += 1;
				}
				km -= ((capturer >> i) & 1) as i64;

				i = kp - 1;
				while (pass >> i) & 1 != 0 {
					i -= 1;
					km += 1;
				}
				km -= ((capturer >> i) & 1) as i64;

				i = kp - 11;
				while (pass >> i) & 1 != 0 {
					i -= 11;
					km += 1;
				}
				km -= ((capturer >> i) & 1) as i64;

				nd * 8 + km * 4 - na * 16
			}
		}
	}
}

impl Heuristic<tictactoe::Tictactoe> for DefaultHeuristic {
	fn eval(g: &tictactoe::Tictactoe) -> i64 {
		match g.state() {
			State::Win => 1,
			State::Lose => -1,
			_ => 0,
		}
	}
}

impl Heuristic<othello::Othello> for DefaultHeuristic {
	fn eval(g: &othello::Othello) -> i64 {
		const WEIGHTS: [i64; 64] = [
			4, -3, 2, 2, 2, 2, -3, 4, -3, -4, -1, -1, -1, -1, -4, -3, 2, -1, 1, 0, 0, 1, -1, 2, 2, -1, 0,
			1, 1, 0, -1, 2, 2, -1, 0, 1, 1, 0, -1, 2, 2, -1, 1, 0, 0, 1, -1, 2, -3, -4, -1, -1, -1, -1,
			-4, -3, 4, -3, 2, 2, 2, 2, -3, 4,
		];
		match g.state() {
			State::Win => 256,
			State::Lose => -256,
			State::Draw => 0,
			State::Going => {
				let mut ans = 0i64;
				for i in 0..64 {
					ans += if g.has_piece(i) {
						if g.get_piece(i) {
							WEIGHTS[i as usize]
						} else {
							-WEIGHTS[i as usize]
						}
					} else {
						0
					};
				}
				ans
			}
		}
	}
}

impl Heuristic<mancala::Mancala> for DefaultHeuristic {
	fn eval(g: &mancala::Mancala) -> i64 {
		const WEIGHTS: [i64; 14] = [7, 6, 5, 4, 3, 2, 8, -7, -6, -5, -4, -3, -2, -8];
		match g.state() {
			State::Win => 32768,
			State::Lose => -32768,
			State::Draw => 0,
			_ => {
				let mut res = 0i64;
				for (i, wi) in WEIGHTS.iter().enumerate() {
					res += wi * g.a[i] as i64;
				}
				res
			}
		}
	}
}
