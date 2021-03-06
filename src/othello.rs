use crate::game::*;

const DIRS: [(i8, i8); 8] = [
	(0, 1),
	(-1, 1),
	(-1, 0),
	(-1, -1),
	(0, -1),
	(1, -1),
	(1, 0),
	(1, 1),
];
const WEIGHTS: [i64; 64] = [
	4, -3, 2, 2, 2, 2, -3, 4, -3, -4, -1, -1, -1, -1, -4, -3, 2, -1, 1, 0, 0, 1, -1, 2, 2, -1, 0, 1,
	1, 0, -1, 2, 2, -1, 0, 1, 1, 0, -1, 2, 2, -1, 1, 0, 0, 1, -1, 2, -3, -4, -1, -1, -1, -1, -4, -3,
	4, -3, 2, 2, 2, 2, -3, 4,
];

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Othello {
	board: (u64, u64), // .0: cell contains piece or not, .1: piece is black or white (1 for player true, 0 for player false)
	turn: bool,
	pass: u8,
}
impl Othello {
	fn add_piece(&mut self, p: u8, c: bool) {
		self.board.0 |= 1u64 << p;
		self.board.1 |= (c as u64) << p;
	}
	fn flip(&mut self, p: u8) {
		self.board.1 ^= 1u64 << p;
	}
	pub fn has_piece(&self, p: u8) -> bool {
		(self.board.0 >> p) & 1 != 0
	}
	pub fn get_piece(&self, p: u8) -> bool {
		(self.board.1 >> p) & 1 != 0
	}
	// if can reverse in a certain directon from pos with a direction dir
	fn reversable(&self, t: bool, mut pos: (i8, i8), dir: (i8, i8)) -> bool {
		pos.0 += dir.0;
		pos.1 += dir.1;
		let mut found = false;
		while pos.0 < 8 && pos.1 < 8 && pos.0 >= 0 && pos.1 >= 0 {
			if !self.has_piece(mapc(pos.0 as u8, pos.1 as u8)) {
				return false;
			} else if self.get_piece(mapc(pos.0 as u8, pos.1 as u8)) == t {
				return found;
			} else {
				found = true;
			}
			pos.0 += dir.0;
			pos.1 += dir.1;
		}
		false
	}
	// assumes reversable
	fn reverse(&mut self, t: bool, mut pos: (i8, i8), dir: (i8, i8)) {
		pos.0 += dir.0;
		pos.1 += dir.1;
		while self.get_piece(mapc(pos.0 as u8, pos.1 as u8)) != t {
			self.flip(mapc(pos.0 as u8, pos.1 as u8));
			pos.0 += dir.0;
			pos.1 += dir.1;
		}
	}
}
fn mapc(x: u8, y: u8) -> u8 {
	(y << 3) | x
}
fn unmapc(p: u8) -> (u8, u8) {
	(p & 7, p >> 3)
}
impl Game for Othello {
	type M = u8;
	type S = (u64, u64, bool);
	type R = (u64, u64);
	fn new(t: bool) -> Self {
		let mut ans = Othello {
			board: (0, 0),
			turn: t,
			pass: 0,
		};
		ans.add_piece(mapc(3, 3), true);
		ans.add_piece(mapc(4, 4), true);
		ans.add_piece(mapc(3, 4), false);
		ans.add_piece(mapc(4, 3), false);
		ans
	}
	fn turn(&self) -> bool {
		self.turn
	}
	fn get_moves(&self) -> Vec<Self::M> {
		let mut ans = Vec::<Self::M>::new();
		for y in 0..8 {
			for x in 0..8 {
				if !self.has_piece(mapc(x, y)) {
					for dir in DIRS.iter() {
						if self.reversable(self.turn, (x as i8, y as i8), *dir) {
							ans.push(mapc(x, y));
							break;
						}
					}
				}
			}
		}
		if ans.is_empty() {
			ans.push(64);
		}
		ans
	}
	fn get_moves_sorted(&self) -> Vec<Self::M> {
		let mut movs = self.get_moves();
		movs.sort_by_key(|x| -WEIGHTS[*x as usize]);
		movs
	}
	fn get_static_state(&self) -> Self::S {
		(self.board.0, self.board.1, self.turn)
	}
	fn state(&self) -> State {
		if self.board.0 != u64::MAX && self.pass < 2 {
			State::Going
		} else {
			match self.board.1.count_ones() {
				0..=31 => State::Lose,
				32 => State::Draw,
				_ => State::Win,
			}
		}
	}
	fn mov(&mut self, m: &u8) {
		if *m != 64 {
			let (x, y) = unmapc(*m);
			for dir in DIRS.iter() {
				if self.reversable(self.turn, (x as i8, y as i8), *dir) {
					self.reverse(self.turn, (x as i8, y as i8), *dir);
				}
			}
			self.add_piece(*m, self.turn);
		} else {
			self.pass += 1;
		}
		self.turn = !self.turn;
	}
	fn mov_with_rollback(&mut self, m: &u8) -> Self::R {
		let ans = self.board;
		self.mov(m);
		ans
	}
	fn rollback(&mut self, rb: Self::R) {
		self.board = rb;
		if self.pass != 0 {
			self.pass -= 1;
		}
		self.turn = !self.turn;
	}
}
impl std::fmt::Display for Othello {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(
			f,
			"{} - {}",
			(self.board.0 & self.board.1).count_ones(),
			(self.board.0 & !self.board.1).count_ones()
		)?;
		for y in 0..8 {
			for x in 0..8 {
				write!(
					f,
					"{}",
					match self.has_piece(mapc(x, y)) {
						true => match self.get_piece(mapc(x, y)) {
							true => 'O',
							false => 'X',
						},
						false => '.',
					}
				)?;
			}
			writeln!(f)?;
		}
		Ok(())
	}
}
