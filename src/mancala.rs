use crate::game::*;
use std::cmp::Ordering;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Mancala {
	a: [u8; 14],
	turn: bool,
	st: Vec<(u8, u8, bool)>,
}

impl Game for Mancala {
	type M = u8;
	type S = ([u8; 14], bool);
	fn new(t: bool) -> Self {
		Mancala {
			a: [4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 0],
			turn: t,
			st: vec![],
		}
	}
	fn turn(&self) -> bool {
		self.turn
	}
	fn get_moves(&self) -> Vec<u8> {
		if self.turn() {
			self
				.a
				.iter()
				.enumerate()
				.filter(|x| *x.1 != 0 && x.0 < 6)
				.map(|x| x.0 as u8)
				.rev()
				.collect()
		} else {
			self
				.a
				.iter()
				.enumerate()
				.filter(|x| *x.1 != 0 && x.0 > 6)
				.map(|x| x.0 as u8)
				.rev()
				.collect()
		}
	}
	fn get_moves_sorted(&self) -> Vec<u8> {
		self.get_moves()
	}
	fn state(&self) -> State {
		let sumhd: u8 = self.a.iter().take(6).sum();
		let sumhu: u8 = self.a.iter().skip(7).take(6).sum();
		if sumhd == 0 || sumhu == 0 {
			let sumd = sumhd + self.a[6];
			let sumu = sumhu + self.a[13];
			match sumd.cmp(&sumu) {
				Ordering::Greater => State::Win,
				Ordering::Less => State::Lose,
				Ordering::Equal => State::Draw,
			}
		} else if self.a[6] > 24 {
			State::Win
		} else if self.a[13] > 24 {
			State::Lose
		} else {
			State::Going
		}
	}
	fn heuristic(&self) -> i64 {
		let s = self.state();
		let w: [i64; 14] = [7, 6, 5, 4, 3, 2, 8, -7, -6, -5, -4, -3, -2, -8];
		match s {
			State::Win => 32768,
			State::Lose => -32768,
			_ => {
				let mut res = 0i64;
				for (i, wi) in w.iter().enumerate() {
					res += wi * self.a[i] as i64;
				}
				res
			}
		}
	}
	fn get_static_state(&self) -> ([u8; 14], bool) {
		(self.a, self.turn)
	}
	fn mov(&mut self, m: &u8) {
		let mut rb: (u8, u8, bool) = (*m, self.a[*m as usize], false);
		let mut i = *m as usize;
		let mut x = self.a[i];
		self.a[i] = 0;
		while x != 0 {
			i = (i + 1) % 14;
			if (i == 13 && self.turn) || (i == 6 && !self.turn) {
				continue;
			}
			self.a[i] += 1;
			x -= 1;
		}
		if self.a[i] == 1 && ((self.turn && i < 6) || (!self.turn && i > 6 && i < 13)) {
			let o = 12 - i;
			rb.2 = true;
			self.a[i] += self.a[o];
			self.a[o] = 0;
		}
		if !((i == 6 && self.turn) || (i == 13 && !self.turn)) {
			self.turn = !self.turn;
		}
		self.st.push(rb);
	}
	fn rollback(&mut self) {
		let rb = self.st.pop().unwrap();
		let mut i = rb.0 as usize;
		let mut x = rb.1;
		self.turn = i < 6;
		while x != 0 {
			i = (i + 1) % 14;
			if (i == 13 && self.turn) || (i == 6 && !self.turn) {
				continue;
			}
			x -= 1;
		}
		if rb.2 {
			self.a[12 - i] += self.a[i] - 1;
			self.a[i] = 1;
		}
		x = rb.1;
		while x != 0 {
			if (i == 13 && self.turn) || (i == 6 && !self.turn) {
				i = (i + 13) % 14;
				continue;
			}
			self.a[i] -= 1;
			i = (i + 13) % 14;
			x -= 1;
		}
		self.a[i] = rb.1;
	}
}
/*
 c b a 9 8 7
d           6
 0 1 2 3 4 5
*/
impl std::fmt::Display for Mancala {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for i in (7..13).rev() {
			write!(f, " {}", self.a[i])?;
		}
		writeln!(f)?;
		writeln!(f, "{}           {}", self.a[13], self.a[6])?;
		for i in 0..6 {
			write!(f, " {}", self.a[i])?;
		}
		writeln!(f)?;
		Ok(())
	}
}
