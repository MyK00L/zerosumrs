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

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Othello {
	board: (u64, u64), // .0: cell contains piece or not, .1: piece is black or white (1 for player true, 0 for player false)
	turn: bool,
	st: Vec<(u8, Vec<u8>)>,
}
impl Othello {
	fn add_piece(&mut self, p: u8, c: bool) {
		self.board.0 |= 1u64 << p;
		self.board.1 |= (c as u64) << p;
	}
	fn rem_piece(&mut self, p: u8) {
		self.board.0 &= !(1u64 << p);
		self.board.1 &= !(1u64 << p);
	}
	fn flip(&mut self, p: u8) {
		self.board.1 ^= 1u64 << p;
	}
	fn has_piece(&self, p: u8) -> bool {
		(self.board.0 >> p) & 1 != 0
	}
	fn get_piece(&self, p: u8) -> bool {
		(self.board.1 >> p) & 1 != 0
	}
	// if can reverse in a certain directon from pos with a direction dir
	fn reversable(&self, t: bool, mut pos: (i8, i8), dir: (i8, i8)) -> bool {
		pos.0 += dir.0;
		pos.1 += dir.1;
		let mut found = false;
		while pos.0 < 8 && pos.1 < 8 && pos.0 >= 0 && pos.1 >= 0 {
			if self.has_piece(mapc(pos.0 as u8, pos.1 as u8)) == false {
				return false;
			} else if self.get_piece(mapc(pos.0 as u8, pos.1 as u8)) == t {
				return found;
			} else {
				found = true;
			}
			pos.0 += dir.0;
			pos.1 += dir.1;
		}
		return false;
	}
	// assumes reversable
	fn reverse(&mut self, t: bool, mut pos: (i8, i8), dir: (i8, i8)) -> Vec<u8> {
		pos.0 += dir.0;
		pos.1 += dir.1;
		let mut ans = Vec::<u8>::new();
		while self.get_piece(mapc(pos.0 as u8, pos.1 as u8)) != t {
			self.flip(mapc(pos.0 as u8, pos.1 as u8));
			ans.push(mapc(pos.0 as u8, pos.1 as u8));
			pos.0 += dir.0;
			pos.1 += dir.1;
		}
		ans
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
	fn new(t: bool) -> Self {
		let mut ans = Othello {
			board: (0, 0),
			turn: t,
			st: vec![],
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
					for di in 0..8 {
						if self.reversable(self.turn, (x as i8, y as i8), DIRS[di]) {
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
		if self.board.0 != u64::MAX
			&& (self.st.len() < 2
				|| self.st[self.st.len() - 1].0 != 64
				|| self.st[self.st.len() - 2].0 != 64)
		{
			State::Going
		} else {
			if self.board.1.count_ones() > 32 {
				State::Win
			} else if self.board.1.count_ones() < 32 {
				State::Lose
			} else {
				State::Draw
			}
		}
	}
	fn heuristic(&self) -> i64 {
		match self.state() {
			State::Lose => -32768,
			State::Win => 32768,
			State::Draw => 0,
			State::Going => {
				let mut ans = 0i64;
				for i in 0..64 {
					ans += if self.has_piece(i) {
						if self.get_piece(i) {
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
	fn mov(&mut self, m: &u8) {
		let mut ste = (*m, Vec::<u8>::new());
		if *m != 64 {
			let (x, y) = unmapc(*m);
			for di in 0..8 {
				if self.reversable(self.turn, (x as i8, y as i8), DIRS[di]) {
					ste
						.1
						.extend(self.reverse(self.turn, (x as i8, y as i8), DIRS[di]));
				}
			}
			self.add_piece(*m, self.turn);
		}
		self.turn = !self.turn;
		self.st.push(ste);
	}
	fn rollback(&mut self) {
		let ste = self.st.pop().unwrap();
		self.turn = !self.turn;
		if ste.0 != 64 {
			self.rem_piece(ste.0);
			for i in ste.1 {
				self.flip(i);
			}
		}
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
