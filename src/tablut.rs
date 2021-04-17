use crate::game::*;

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
	E, // Empty=0
	A, // Attacker=1
	D, // Defender=2
	K, // King=3
}
impl From<u64> for Tile {
	fn from(x: u64) -> Self {
		match x & 3 {
			1 => Tile::A,
			2 => Tile::D,
			3 => Tile::K,
			_ => Tile::E,
		}
	}
}

const STARTING_POSITION: [[Tile; 9]; 9] = [
	[
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::A,
		Tile::A,
		Tile::A,
		Tile::E,
		Tile::E,
		Tile::E,
	],
	[
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::A,
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::E,
	],
	[
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::D,
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::E,
	],
	[
		Tile::A,
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::D,
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::A,
	],
	[
		Tile::A,
		Tile::A,
		Tile::D,
		Tile::D,
		Tile::K,
		Tile::D,
		Tile::D,
		Tile::A,
		Tile::A,
	],
	[
		Tile::A,
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::D,
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::A,
	],
	[
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::D,
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::E,
	],
	[
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::A,
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::E,
	],
	[
		Tile::E,
		Tile::E,
		Tile::E,
		Tile::A,
		Tile::A,
		Tile::A,
		Tile::E,
		Tile::E,
		Tile::E,
	],
];

const BLOCKS: [bool; 81] = [
	false, false, false, true, true, true, false, false, false, false, false, false, false, true,
	false, false, false, false, false, false, false, false, false, false, false, false, false, true,
	false, false, false, false, false, false, false, true, true, true, false, false, true, false,
	false, true, true, true, false, false, false, false, false, false, false, true, false, false,
	false, false, false, false, false, false, false, false, false, false, false, true, false, false,
	false, false, false, false, false, true, true, true, false, false, false,
];
const CAPTURE_AID: [bool;81] = [
	false, false, false, true , false, true , false, false, false,
	false, false, false, false, true , false, false, false, false,
	false, false, false, false, false, false, false, false, false,
	true , false, false, false, false, false, false, false, true ,
	false, true , false, false, true , false, false, true , false,
	true , false, false, false, false, false, false, false, true ,
	false, false, false, false, false, false, false, false, false,
	false, false, false, false, true , false, false, false, false,
	false, false, false, true , false , true , false, false, false,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tablut {
	a: (u64, u64, u64),
	turn: bool,
	st: Vec<(u64, u64, u64)>, // whole state
}
fn mapc(x: u8, y: u8) -> u8 {
	y * 9 + x
}
fn unmapc(p: u8) -> (u8, u8) {
	(p % 9, p / 9)
}
fn is_block_um(p: u8) -> bool {
	BLOCKS[p as usize]
}
fn is_capture_aid(p: u8) -> bool {
	CAPTURE_AID[p as usize]
}
impl Tablut {
	fn get(&self, mut pos: u8) -> Tile {
		pos <<= 1;
		if pos >= 128 {
			pos -= 128;
			(self.a.2 >> pos).into()
		} else if pos >= 64 {
			pos -= 64;
			(self.a.1 >> pos).into()
		} else {
			(self.a.0 >> pos).into()
		}
	}
	fn set(&mut self, mut pos: u8, v: Tile) {
		pos <<= 1;
		if pos >= 128 {
			pos -= 128;
			self.a.2 &= !(3 << pos);
			self.a.2 |= (v as u64) << pos;
		} else if pos >= 64 {
			pos -= 64;
			self.a.1 &= !(3 << pos);
			self.a.1 |= (v as u64) << pos;
		} else {
			self.a.0 &= !(3 << pos);
			self.a.0 |= (v as u64) << pos;
		}
	}
	fn captured(&self, a1: u8, a2: u8) -> bool {
		if self.turn {
			self.get(a1) == Tile::A
				&& (self.get(a2) == Tile::D || self.get(a2) == Tile::K || is_capture_aid(a2))
		} else {
			(self.get(a1) == Tile::D && (self.get(a2) == Tile::A || is_capture_aid(a2)))
				|| (self.get(a1) == Tile::K
					&& (self.get(a1 + 9) == Tile::A || is_capture_aid(a1 + 9))
					&& (self.get(a1 - 9) == Tile::A || is_capture_aid(a1 - 9))
					&& (self.get(a1 + 1) == Tile::A || is_capture_aid(a1 + 1))
					&& (self.get(a1 - 1) == Tile::A || is_capture_aid(a1 - 1)))
				|| (self.get(a1) == Tile::K
					&& a1 != mapc(4, 4)
					&& a1 != mapc(4, 3)
					&& a1 != mapc(3, 4)
					&& a1 != mapc(4, 5)
					&& a1 != mapc(5, 4)
					&& (self.get(a2) == Tile::A || is_capture_aid(a2)))
		}
	}
}
impl Game for Tablut {
	type M = (u8, u8); // compressed coords from and to (4bits x, 4bits y)
	type S = ((u64, u64, u64), bool);
	fn new(t: bool) -> Self {
		let mut ans = Tablut {
			a: (0, 0, 0),
			turn: t,
			st: vec![],
		};
		for y in 0..9 {
			for x in 0..9 {
				ans.set(mapc(x, y), STARTING_POSITION[y as usize][x as usize]);
			}
		}
		ans
	}
	fn turn(&self) -> bool {
		self.turn
	}
	fn get_moves(&self) -> Vec<Self::M> {
		let mut ans = Vec::<Self::M>::new();

		// right
		for y in 0..9 {
			let mut last = 128u8;
			for x in 0..9 {
				let p = mapc(x, y);
				let t = self.get(p);
				if t == Tile::E {
					if is_block_um(p) && (last==128 || !is_block_um(last) || p-last>2) {
						last = 128;
					} else if last != 128 {
						ans.push((last, p));
					}
				} else {
					last = if self.turn == (t != Tile::A) { p } else { 128 }
				}
			}
		}
		// left
		for y in 0..9 {
			let mut last = 128u8;
			for x in (0..9).rev() {
				let p = mapc(x, y);
				let t = self.get(p);
				if t == Tile::E {
					if is_block_um(p) && (last==128 || !is_block_um(last) || last-p>2) {
						last = 128;
					} else if last != 128 {
						ans.push((last, p));
					}
				} else {
					last = if self.turn == (t != Tile::A) { p } else { 128 }
				}
			}
		}
		// down
		for x in 0..9 {
			let mut last = 128u8;
			for y in 0..9 {
				let p = mapc(x, y);
				let t = self.get(p);
				if t == Tile::E {
					if is_block_um(p) && (last==128 || !is_block_um(last) || p-last>2*9) {
						last = 128;
					} else if last != 128 {
						ans.push((last, p));
					}
				} else {
					last = if self.turn == (t != Tile::A) { p } else { 128 }
				}
			}
		}
		// up
		for x in 0..9 {
			let mut last = 128u8;
			for y in (0..9).rev() {
				let p = mapc(x, y);
				let t = self.get(p);
				if t == Tile::E {
					if is_block_um(p) && (last==128 || !is_block_um(last) || last-p>2*9) {
						last = 128;
					} else if last != 128 {
						ans.push((last, p));
					}
				} else {
					last = if self.turn == (t != Tile::A) { p } else { 128 }
				}
			}
		}

		ans
	}
	fn get_moves_sorted(&self) -> Vec<Self::M> {
		self.get_moves() // to implement?
	}
	fn get_static_state(&self) -> Self::S {
		(self.a, self.turn)
	}
	fn state(&self) -> State {
		const WINS: [u8; 4] = [0, 1, 5, 7];
		for i in WINS.iter() {
			if self.get(mapc(*i, 0)) == Tile::K
				|| self.get(mapc(*i, 8)) == Tile::K
				|| self.get(mapc(0, *i)) == Tile::K
				|| self.get(mapc(8, *i)) == Tile::K
			{
				return State::Win;
			}
		}
		if self.get_moves().is_empty() {
			return if self.turn {
				State::Lose
			} else {
				State::Win
			};
		}
		for y in 1..8 {
			for x in 1..8 {
				if self.get(mapc(y, x)) == Tile::K {
					return State::Going;
				}
			}
		}
		return State::Lose;
	}
	fn heuristic(&self) -> i64 {
		unimplemented!();
	}
	fn mov(&mut self, m: &Self::M) {
		self.st.push(self.a);
		let x = self.get(m.0);
		self.set(m.0, Tile::E);
		self.set(m.1, x);
		if m.1 + 18 < 81 && self.captured(m.1 + 9, m.1 + 18) {
			self.set(m.1 + 9, Tile::E);
		}
		if m.1 >= 18 && self.captured(m.1 - 9, m.1 - 18) {
			self.set(m.1 - 9, Tile::E);
		}
		if m.1 % 9 < 7 && self.captured(m.1 + 1, m.1 + 2) {
			self.set(m.1 + 1, Tile::E);
		}
		if m.1 % 9 >= 2 && self.captured(m.1 - 1, m.1 - 2) {
			self.set(m.1 - 1, Tile::E);
		}
		self.turn = !self.turn;
	}
	fn rollback(&mut self) {
		self.turn = !self.turn;
		self.a = self.st.pop().unwrap();
	}
}
impl std::fmt::Display for Tablut {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for y in 0..9 {
			for x in 0..9 {
				write!(
					f,
					"{}",
					match self.get(mapc(x, y)) {
						Tile::K => 'K',
						Tile::D => 'D',
						Tile::A => 'A',
						Tile::E => '.',
					}
				)?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}
