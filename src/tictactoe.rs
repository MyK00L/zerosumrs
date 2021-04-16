use crate::game::*;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Ttt {
	a: [u8; 9],
	turn: bool,
	st: Vec<u8>,
}
impl Game for Ttt {
	type M = u8;
	type S = ([u8; 9], bool);
	fn new(t: bool) -> Self {
		Ttt {
			a: [2, 2, 2, 2, 2, 2, 2, 2, 2],
			turn: t,
			st: vec![],
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
	fn heuristic(&self) -> i64 {
		let s = self.state();
		match s {
			State::Win => 1,
			State::Lose => -1,
			_ => 0,
		}
	}
	fn mov(&mut self, m: &u8) {
		self.st.push(*m);
		self.a[*m as usize] = if self.turn { 1 } else { 0 };
		self.turn = !self.turn;
	}
	fn rollback(&mut self) {
		let case = self.st.pop().unwrap() as usize;
		self.a[case] = 2;
		self.turn = !self.turn;
	}
}
impl std::fmt::Display for Ttt {
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
