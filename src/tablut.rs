use crate::game::*;

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
	E, // Empty=0
	A, // Attacker=1
	D, // Defender=2
	K, // King=3
}
impl From<u8> for Tile {
	fn from(x: u8) -> Self {
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

const GOAL: [bool; 81] = [
	false, true, true, false, false, false, true, true, false, true, false, false, false, false,
	false, false, false, true, true, false, false, false, false, false, false, false, true, false,
	false, false, false, false, false, false, false, false, false, false, false, false, false, false,
	false, false, false, false, false, false, false, false, false, false, false, false, true, false,
	false, false, false, false, false, false, true, true, false, false, false, false, false, false,
	false, true, false, true, true, false, false, false, true, true, false,
];
const BLOCKS: [bool; 81] = [
	false, false, false, true, true, true, false, false, false, false, false, false, false, true,
	false, false, false, false, false, false, false, false, false, false, false, false, false, true,
	false, false, false, false, false, false, false, true, true, true, false, false, true, false,
	false, true, true, true, false, false, false, false, false, false, false, true, false, false,
	false, false, false, false, false, false, false, false, false, false, false, true, false, false,
	false, false, false, false, false, true, true, true, false, false, false,
];
const CAPTURE_AID: [bool; 81] = [
	false, false, false, true, false, true, false, false, false, false, false, false, false, true,
	false, false, false, false, false, false, false, false, false, false, false, false, false, true,
	false, false, false, false, false, false, false, true, false, true, false, false, true, false,
	false, true, false, true, false, false, false, false, false, false, false, true, false, false,
	false, false, false, false, false, false, false, false, false, false, false, true, false, false,
	false, false, false, false, false, true, false, true, false, false, false,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tablut {
	board: [u8; 21],
	turn: u32, //%2=0 defender, %2=1 attacker
	state: State,
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
	fn get(&self, pos: u8) -> Tile {
		((self.board[(pos >> 2) as usize] >> (pos & 3) >> (pos & 3)) & 3).into()
	}
	fn set(&mut self, pos: u8, v: Tile) {
		self.board[(pos >> 2) as usize] &= !(3 << (pos & 3) << (pos & 3));
		self.board[(pos >> 2) as usize] |= (v as u8) << (pos & 3) << (pos & 3);
	}
	fn captured(&self, a1: u8, a2: u8) -> bool {
		if self.turn() {
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
	type S = ([u8; 21], bool);
	type R = (Self::M, u8); // first: coords, second: ruld tiles
	fn new(t: bool) -> Self {
		let mut ans = Tablut {
			board: <[u8; 21]>::default(),
			turn: if t { 0 } else { 1 },
			state: State::Going,
		};
		for y in 0..9 {
			for x in 0..9 {
				ans.set(mapc(x, y), STARTING_POSITION[y as usize][x as usize]);
			}
		}
		ans
	}
	fn turn(&self) -> bool {
		(self.turn & 1) == 0
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
					if is_block_um(p) && (last == 128 || !is_block_um(last) || p - last > 2) {
						last = 128;
					} else if last != 128 {
						ans.push((last, p));
					}
				} else {
					last = if self.turn() == (t != Tile::A) {
						p
					} else {
						128
					}
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
					if is_block_um(p) && (last == 128 || !is_block_um(last) || last - p > 2) {
						last = 128;
					} else if last != 128 {
						ans.push((last, p));
					}
				} else {
					last = if self.turn() == (t != Tile::A) {
						p
					} else {
						128
					}
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
					if is_block_um(p) && (last == 128 || !is_block_um(last) || p - last > 2 * 9) {
						last = 128;
					} else if last != 128 {
						ans.push((last, p));
					}
				} else {
					last = if self.turn() == (t != Tile::A) {
						p
					} else {
						128
					}
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
					if is_block_um(p) && (last == 128 || !is_block_um(last) || last - p > 2 * 9) {
						last = 128;
					} else if last != 128 {
						ans.push((last, p));
					}
				} else {
					last = if self.turn() == (t != Tile::A) {
						p
					} else {
						128
					}
				}
			}
		}

		ans
	}
	fn get_moves_sorted(&self) -> Vec<Self::M> {
		self.get_moves() // to implement?
	}
	fn get_static_state(&self) -> Self::S {
		(self.board, self.turn())
	}
	fn state(&self) -> State {
		self.state
	}
	fn heuristic(&self) -> i64 {
		match self.state() {
			State::Win => 32768 - (self.turn as i64),
			State::Lose => -32768 + (self.turn as i64),
			State::Draw => 0,
			State::Going => {
				let mut nd = 0;
				let mut md = 0;
				let mut na = 0;
				let mut ma = 0;
				let mut mk = 0;
				for i in 0..81 {
					let t = self.get(i);
					if t == Tile::D {
						nd += 1;
					}
					if t == Tile::A {
						na += 1;
					}
				}
				let mut gc = Tablut {
					board: self.board,
					turn: self.turn,
					state: State::Going,
				};
				// for _ in 0..2 {
				// 	for i in gc.get_moves() {
				// 		match gc.get(i.0) {
				// 			Tile::D => {
				// 				md += 1;
				// 			}
				// 			Tile::K => {
				// 				mk += 1;
				// 			}
				// 			Tile::A => {
				// 				ma += 1;
				// 			}
				// 			_ => {}
				// 		}
				// 	}
				// 	gc.turn ^= 1;
				// }
				nd * 6 - na * 3 - ma + 2 * md + 4 * mk
			}
		}
	}
	fn mov(&mut self, m: &Self::M) {
		let x = self.get(m.0);
		self.set(m.0, Tile::E);
		self.set(m.1, x);
		if m.1 + 18 < 81 && self.captured(m.1 + 9, m.1 + 18) {
			if self.get(m.1 + 9) == Tile::K {
				self.state = State::Lose;
			}
			self.set(m.1 + 9, Tile::E);
		}
		if m.1 >= 18 && self.captured(m.1 - 9, m.1 - 18) {
			if self.get(m.1 - 9) == Tile::K {
				self.state = State::Lose;
			}
			self.set(m.1 - 9, Tile::E);
		}
		if m.1 % 9 < 7 && self.captured(m.1 + 1, m.1 + 2) {
			if self.get(m.1 + 1) == Tile::K {
				self.state = State::Lose;
			}
			self.set(m.1 + 1, Tile::E);
		}
		if m.1 % 9 >= 2 && self.captured(m.1 - 1, m.1 - 2) {
			if self.get(m.1 - 1) == Tile::K {
				self.state = State::Lose;
			}
			self.set(m.1 - 1, Tile::E);
		}
		if GOAL[m.1 as usize] && self.get(m.1) == Tile::K {
			self.state = State::Win;
		}
		self.turn += 1;
		if self.get_moves().is_empty() {
			self.state = match self.turn() {
				true => State::Win,
				false => State::Lose,
			}
		}
	}
	fn mov_with_rollback(&mut self, m: &Self::M) -> Self::R {
		let mut rb = (*m, 0u8);
		if m.1 + 9 < 81 {
			rb.1 |= self.get(m.1 + 9) as u8;
		}
		if m.1 >= 9 {
			rb.1 |= (self.get(m.1 - 9) as u8) << 2;
		}
		if m.1 + 1 < 81 {
			rb.1 |= (self.get(m.1 + 1) as u8) << 4;
		}
		if m.1 >= 1 {
			rb.1 |= (self.get(m.1 - 1) as u8) << 6;
		}
		self.mov(m);
		rb
	}
	fn rollback(&mut self, rbf: Self::R) {
		self.turn -= 1;
		self.state = State::Going;
		let (m, rb) = rbf;
		if m.1 + 9 < 81 {
			self.set(m.1 + 9, rb.into());
		}
		if m.1 >= 9 {
			self.set(m.1 - 9, (rb >> 2).into());
		}
		if m.1 + 1 < 81 {
			self.set(m.1 + 1, (rb >> 4).into());
		}
		if m.1 >= 1 {
			self.set(m.1 - 1, (rb >> 6).into());
		}
		self.set(m.0, self.get(m.1));
		self.set(m.1, Tile::E);
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
