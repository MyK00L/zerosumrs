use crate::game::*;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Tictactoe {
	a: [u8; 9],
	turn: bool,
}
impl Game for Tictactoe {
	type M = u8;
	type S = ([u8; 9], bool);
	type R = u8;
	fn new(t: bool) -> Self {
		Tictactoe {
			a: [2, 2, 2, 2, 2, 2, 2, 2, 2],
			turn: t,
		}
	}
	fn turn(&self) -> bool {
		self.turn
	}
	fn get_moves(&self) -> Vec<u8> {
		self
			.a
			.iter()
			.enumerate()
			.filter(|x| *x.1 == 2)
			.map(|x| x.0 as u8)
			.collect()
	}
	fn get_moves_sorted(&self) -> Vec<u8> {
		self.get_moves()
	}
	fn get_static_state(&self) -> Self::S {
		(self.a, self.turn)
	}
	fn state(&self) -> State {
		for j in 0..3 {
			let i: usize = j * 3;
			if self.a[i] == self.a[i + 1] && self.a[i] == self.a[i + 2] && self.a[i] != 2 {
				return if self.a[i] == 1 {
					State::Win
				} else {
					State::Lose
				};
			}
			if self.a[j] == self.a[3 + j] && self.a[j] == self.a[6 + j] && self.a[j] != 2 {
				return if self.a[j] == 1 {
					State::Win
				} else {
					State::Lose
				};
			}
		}
		if self.a[0] == self.a[4] && self.a[0] == self.a[8] && self.a[0] != 2 {
			return if self.a[0] == 1 {
				State::Win
			} else {
				State::Lose
			};
		}
		if self.a[2] == self.a[4] && self.a[2] == self.a[6] && self.a[2] != 2 {
			return if self.a[2] == 1 {
				State::Win
			} else {
				State::Lose
			};
		}
		if self.a.iter().filter(|x| **x == 2).count() == 0 {
			State::Draw
		} else {
			State::Going
		}
	}
	fn mov(&mut self, m: &Self::M) {
		self.a[*m as usize] = if self.turn { 1 } else { 0 };
		self.turn = !self.turn;
	}
	fn mov_with_rollback(&mut self, m: &Self::M) -> Self::R {
		self.mov(m);
		*m
	}
	fn rollback(&mut self, m: Self::R) {
		self.a[m as usize] = 2;
		self.turn = !self.turn;
	}
}
impl std::fmt::Display for Tictactoe {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for y in 0..3 {
			for x in 0..3 {
				write!(
					f,
					"{}",
					match self.a[y * 3 + x] {
						0 => 'O',
						1 => 'X',
						_ => '.',
					}
				)?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}
