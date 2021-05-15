use crate::game::*;
/*
0000000
00000000000
00001110000
00000100000
00000000000
01000000000
01100000000
01000000000
00000000000
00000000000
00000000000
00000000000
*/
pub const CITADELS_0: u128 = 0b00000000000000000000001110000000001000000000000000001000000000011000000000100000000000000000000000000000000000000000000000000000;

/*
0000000
00000000000
00000000000
00000000000
00000000000
00000000010
00000000110
00000000010
00000000000
00000100000
00001110000
00000000000
*/
pub const CITADELS_1: u128 = 0b00000000000000000000000000000000000000000000000000000000000010000000001100000000001000000000000000001000000000111000000000000000;

/*
0000000
00000000000
00001110000
00000100000
00000000000
01000000010
01100000110
01000000010
00000000000
00000100000
00001110000
00000000000
*/
pub const START_A: u128 = 0b00000000000000000000001110000000001000000000000000001000000010011000001100100000001000000000000000001000000000111000000000000000;

/*
0000000
00000000000
00000000000
00000000000
00000100000
00000100000
00011011000
00000100000
00000100000
00000000000
00000000000
00000000000
*/
pub const START_D: u128 = 0b00000000000000000000000000000000000000000000010000000000100000000110110000000010000000000100000000000000000000000000000000000000;

/*
0000000
00000000000
00000000000
00000000000
00000000000
00000000000
00000100000
00000000000
00000000000
00000000000
00000000000
00000000000
*/
pub const START_K: u128 = 0b00000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000;

/*
0000000
00000000000
00000000000
00000000000
00000000000
00000100000
00001110000
00000100000
00000000000
00000000000
00000000000
00000000000
*/
pub const K_SAFETY: u128 = 0b00000000000000000000000000000000000000000000000000000000100000000011100000000010000000000000000000000000000000000000000000000000;

/*
0000000
11111111111
10001110001
10000100001
10000000001
11000000011
11100100111
11000000011
10000000001
10000100001
10001110001
11111111111
*/
pub const BLOCK: u128 = 0b00000001111111111110001110001100001000011000000000111000000011111001001111100000001110000000001100001000011000111000111111111111;

/*
0000000
00000000000
00001010000
00000100000
00000000000
01000000010
00100100100
01000000010
00000000000
00000100000
00001010000
00000000000
*/
pub const CAPTURE_AID: u128 = 0b00000000000000000000001010000000001000000000000000001000000010001001001000100000001000000000000000001000000000101000000000000000;

/*
0000000
00000000000
00110001100
01000000010
01000000010
00000000000
00000000000
00000000000
01000000010
01000000010
00110001100
00000000000
*/
pub const GOAL: u128 = 0b00000000000000000000110001100010000000100100000001000000000000000000000000000000000001000000010010000000100011000110000000000000;

// a &= !(1u128<<p);
// a |= 1u128<<p;
// (a>>p)&1 != 0

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tablut {
	pub a: u128,
	pub d: u128,
	pub k: u128,
	pub turn: u32,
	pub state: State,
}

impl Default for Tablut {
	fn default() -> Self {
		Tablut::new(true)
	}
}

impl Game for Tablut {
	type M = (u8, u8);
	type S = Self;
	type R = Self;
	fn new(t: bool) -> Self {
		Tablut {
			a: START_A,
			d: START_D,
			k: START_K,
			turn: if t { 0 } else { 1 },
			state: State::Going,
		}
	}
	fn turn(&self) -> bool {
		self.turn & 1 == 0
	}
	fn get_moves(&self) -> Vec<Self::M> {
		let mut ans = Vec::<Self::M>::with_capacity(96);
		let mut allies = if self.turn() { self.d | self.k } else { self.a };
		let pawns = self.d | self.k | self.a;
		while allies != 0 {
			let p = allies.trailing_zeros() as u8;
			allies ^= 1u128 << p;
			let pass = !(if self.turn() {
				BLOCK
			} else if (CITADELS_0 >> p) & 1 != 0 {
				BLOCK ^ CITADELS_0
			} else if (CITADELS_1 >> p) & 1 != 0 {
				BLOCK ^ CITADELS_1
			} else {
				BLOCK
			} | pawns);
			let mut i = p + 1;
			while (pass >> i) & 1 != 0 {
				ans.push((p, i));
				i += 1;
			}
			i = p + 11;
			while (pass >> i) & 1 != 0 {
				ans.push((p, i));
				i += 11;
			}
			i = p - 1;
			while (pass >> i) & 1 != 0 {
				ans.push((p, i));
				i -= 1;
			}
			i = p - 11;
			while (pass >> i) & 1 != 0 {
				ans.push((p, i));
				i -= 11;
			}
		}
		if ans.is_empty() {
			ans.push((0, 0));
		}
		ans
	}
	fn get_moves_sorted(&self) -> Vec<Self::M> {
		let mut ans = self.get_moves();
		ans.sort_unstable_by_key(|m| {
			const ORDI: [[u8; 9]; 2] = [
				// 0,1,2,3,4,5,6,7,8   // old order
				[9, 4, 5, 3, 6, 7, 2, 1, 0], // def, lower is better
				[9, 5, 2, 4, 3, 7, 1, 6, 0], // atk, lower is better
			];
			let dif = if m.0 > m.1 { m.0 - m.1 } else { m.1 - m.0 };
			let dist = if dif >= 11 { dif / 11 } else { dif };
			if self.turn() {
				if (self.k >> m.0) & 1 != 0 {
					ORDI[(self.turn & 1) as usize][dist as usize]
				} else {
					ORDI[(self.turn & 1) as usize][dist as usize] + 8
				}
			} else {
				if (self.k >> (m.1 + 1)) & 1 != 0
					|| (self.k >> (m.1 + 11)) & 1 != 0
					|| (self.k >> (m.1 - 1)) & 1 != 0
					|| (self.k >> (m.1 - 11)) & 1 != 0
				{
					ORDI[(self.turn & 1) as usize][dist as usize]
				} else {
					ORDI[(self.turn & 1) as usize][dist as usize] + 8
				}
			}
		});
		ans
	}
	fn get_static_state(&self) -> Self::S {
		*self
	}
	fn state(&self) -> State {
		self.state
	}
	fn mov(&mut self, m: &Self::M) {
		if m.0 == 0 {
			self.state = if self.turn() { State::Win } else { State::Lose };
			return;
		}
		if self.turn() {
			// def
			if (self.d >> m.0) & 1 != 0 {
				// d moved
				self.d &= !(1u128 << m.0);
				self.d |= 1u128 << m.1;
			} else {
				// k moved
				self.k &= !(1u128 << m.0);
				self.k |= 1u128 << m.1;
			}
			let capturers = self.d | self.k | CAPTURE_AID;
			if (capturers >> (m.1 + 2)) & 1 != 0 {
				self.a &= !(1u128 << (m.1 + 1));
			}
			if (capturers >> (m.1 + 22)) & 1 != 0 {
				self.a &= !(1u128 << (m.1 + 11));
			}
			if (capturers >> (m.1 - 2)) & 1 != 0 {
				self.a &= !(1u128 << (m.1 - 1));
			}
			if (capturers >> (m.1 - 22)) & 1 != 0 {
				self.a &= !(1u128 << (m.1 - 11));
			}
		} else {
			// atk
			self.a &= !(1u128 << m.0);
			self.a |= 1u128 << m.1;
			let capturers = self.a | CAPTURE_AID;
			if (capturers >> (m.1 + 2)) & 1 != 0 {
				self.d &= !(1u128 << (m.1 + 1));
			}
			if (capturers >> (m.1 + 22)) & 1 != 0 {
				self.d &= !(1u128 << (m.1 + 11));
			}
			if (capturers >> (m.1 - 2)) & 1 != 0 {
				self.d &= !(1u128 << (m.1 - 1));
			}
			if (capturers >> (m.1 - 22)) & 1 != 0 {
				self.d &= !(1u128 << (m.1 - 11));
			}
			let sides =
				(1u128 << (m.1 + 1)) | (1u128 << (m.1 + 11)) | (1u128 << (m.1 - 1)) | (1u128 << (m.1 - 11));
			if self.k & sides != 0 {
				if self.k & K_SAFETY != 0 {
					let kp = self.k.trailing_zeros();
					let ksides =
						(1u128 << (kp + 1)) | (1u128 << (kp + 11)) | (1u128 << (kp - 1)) | (1u128 << (kp - 11));
					if (capturers & ksides).count_ones() == 4 {
						self.k = 0;
					}
				} else {
					if (capturers >> (m.1 + 2)) & 1 != 0 {
						self.k &= !(1u128 << (m.1 + 1));
					}
					if (capturers >> (m.1 + 22)) & 1 != 0 {
						self.k &= !(1u128 << (m.1 + 11));
					}
					if (capturers >> (m.1 - 2)) & 1 != 0 {
						self.k &= !(1u128 << (m.1 - 1));
					}
					if (capturers >> (m.1 - 22)) & 1 != 0 {
						self.k &= !(1u128 << (m.1 - 11));
					}
				}
			}
		}
		self.turn += 1;
		self.state = if self.k == 0 {
			State::Lose
		} else if self.k & GOAL != 0 {
			State::Win
		} else {
			State::Going
		}
	}
	fn mov_with_rollback(&mut self, m: &Self::M) -> Self::R {
		let t = *self;
		self.mov(m);
		t
	}
	fn rollback(&mut self, rbf: Self::R) {
		*self = rbf;
	}
}
impl std::fmt::Display for Tablut {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for y in 1..10 {
			for x in 1..10 {
				let p = y * 11 + x;
				if (self.a >> p) & 1 != 0 {
					write!(f, "A")?;
				} else if (self.d >> p) & 1 != 0 {
					write!(f, "D")?;
				} else if (self.k >> p) & 1 != 0 {
					write!(f, "K")?;
				} else if (BLOCK >> p) & 1 != 0 {
					write!(f, ",")?;
				} else {
					write!(f, ".")?;
				}
			}
			writeln!(f)?;
		}
		Ok(())
	}
}
