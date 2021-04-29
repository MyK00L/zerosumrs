use crate::ai::Ai;
use crate::game::*;
use crate::tablut_with_draw::*;
use std::time::Instant;

pub struct MinimaxTablut {
	pub g: Tablut,
	pub mf: [usize; 81 * 81],
	pub mftot: [usize; 81 * 81],
	pub mdf: [usize; 9],
	pub mdftot: [usize; 9],
}

const D_TABLE: [usize; 81] = [
	1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 3, 3, 1, 0, 1, 3, 3, 1, 1, 3, 3, 3, 2, 3, 3, 3, 1, 0, 1, 3, 3, 2,
	3, 3, 1, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 1, 3, 3, 2, 3, 3, 1, 0, 1, 3, 3, 3, 2, 3, 3, 3, 1, 1,
	3, 3, 1, 0, 1, 3, 3, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1,
];
const D_VALS: [i64; 4] = [32, 33, 34, 35];
const K_TABLE: [usize; 81] = [
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 3, 0, 3, 5, 5, 0, 0, 5, 6, 4, 4, 4, 6, 5, 0, 0, 3, 4, 1, 2,
	1, 4, 3, 0, 0, 0, 4, 2, 3, 2, 4, 0, 0, 0, 3, 4, 1, 2, 1, 4, 3, 0, 0, 5, 6, 4, 4, 4, 6, 5, 0, 0,
	5, 5, 3, 0, 3, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const K_VALS: [i64; 7] = [64, 65, 66, 67, 68, 72, 74];
const A_TABLE: [usize; 81] = [
	1, 4, 3, 3, 3, 3, 3, 4, 1, 4, 6, 6, 2, 3, 2, 6, 6, 4, 3, 6, 7, 3, 2, 3, 7, 6, 3, 3, 2, 3, 5, 4,
	5, 3, 2, 3, 3, 3, 2, 4, 0, 4, 2, 3, 3, 3, 2, 3, 5, 4, 5, 3, 2, 3, 3, 6, 7, 3, 2, 3, 7, 6, 3, 4,
	6, 6, 2, 3, 2, 6, 6, 4, 1, 4, 3, 3, 3, 3, 3, 4, 1,
];
const A_VALS: [i64; 8] = [-16, -17, -18, -19, -20, -21, -22, -26];

impl MinimaxTablut {
	fn heur(&mut self) -> i64 {
		match self.g.state() {
			State::Win => 32768 - self.g.turn as i64,
			State::Lose => -32768 + self.g.turn as i64,
			State::Draw => 0,
			State::Going => {
				//nd * 6 - na * 3 - ma + 2 * md + 4 * mk
				let mut ans = if self.turn() { 1 } else { -1 } - 16;
				for i in 0..81 {
					let t = self.g.get(i);
					if t == Tile::D {
						ans += 6;
					}
					if t == Tile::A {
						ans -= 3;
					}
					/*if t == Tile::K {
						let mut tl = if i==mapc(4,4) || i==mapc(3,4) || i==mapc(5,4) || i==mapc(4,3) || i==mapc(4,5) { 4 } else { 2 };
						if self.g.get(i-9) == Tile::A || is_capture_aid(i-9) {
							tl-=1;
						}
						if self.g.get(i-1) == Tile::A || is_capture_aid(i-1) {
							tl-=1;
						}
						if self.g.get(i+1) == Tile::A || is_capture_aid(i+1) {
							tl-=1;
						}
						if self.g.get(i+9) == Tile::A || is_capture_aid(i+9) {
							tl-=1;
						}
						ans+=tl*4;
					}*/
				}
				// right
				for y in 0..9 {
					let mut last = Tile::E;
					let mut lastp = 128u8;
					for x in 0..9 {
						let p = mapc(x, y);
						let t = self.g.get(p);
						if t == Tile::E {
							if is_block_um(p) && (last == Tile::E || !is_block_um(lastp) || p - lastp > 2) {
								last = Tile::E;
							} else if last != Tile::E {
								ans += match last {
									Tile::D => 2,
									Tile::K => 4,
									Tile::A => -1,
									_ => 0,
								}
							}
						} else {
							last = t;
							lastp = mapc(x, y);
						}
					}
				}
				// left
				for y in 0..9 {
					let mut last = Tile::E;
					let mut lastp = 128u8;
					for x in (0..9).rev() {
						let p = mapc(x, y);
						let t = self.g.get(p);
						if t == Tile::E {
							if is_block_um(p) && (last == Tile::E || !is_block_um(lastp) || lastp - p > 2) {
								last = Tile::E;
							} else if last != Tile::E {
								ans += match last {
									Tile::D => 2,
									Tile::K => 4,
									Tile::A => -1,
									_ => 0,
								}
							}
						} else {
							last = t;
							lastp = mapc(x, y);
						}
					}
				}
				// down
				for x in 0..9 {
					let mut last = Tile::E;
					let mut lastp = 128u8;
					for y in 0..9 {
						let p = mapc(x, y);
						let t = self.g.get(p);
						if t == Tile::E {
							if is_block_um(p) && (last == Tile::E || !is_block_um(lastp) || p - lastp > 2 * 9) {
								last = Tile::E;
							} else if last != Tile::E {
								ans += match last {
									Tile::D => 2,
									Tile::K => 4,
									Tile::A => -1,
									_ => 0,
								}
							}
						} else {
							last = t;
							lastp = mapc(x, y);
						}
					}
				}
				// up
				for x in 0..9 {
					let mut last = Tile::E;
					let mut lastp = 128u8;
					for y in (0..9).rev() {
						let p = mapc(x, y);
						let t = self.g.get(p);
						if t == Tile::E {
							if is_block_um(p) && (last == Tile::E || !is_block_um(lastp) || lastp - p > 2 * 9) {
								last = Tile::E;
							} else if last != Tile::E {
								ans += match last {
									Tile::D => 2,
									Tile::K => 4,
									Tile::A => -1,
									_ => 0,
								}
							}
						} else {
							last = t;
							lastp = mapc(x, y);
						}
					}
				}
				ans
			}
		}
	}
	fn minimax(&mut self, mut a: i64, mut b: i64, depth: u32) -> i64 {
		if self.g.state() != State::Going || depth == 0 {
			return self.heur();
		}
		let moves = self.g.get_moves_sorted();
		let mut res = if self.g.turn() { a } else { b };
		for m in moves.iter() {
			let rb = self.g.mov_with_rollback(m);
			let h = self.minimax(a, b, depth - 1);
			self.g.rollback(rb);
			if self.g.turn() {
				res = res.max(h);
				a = a.max(h);
			} else {
				res = res.min(h);
				b = b.min(h);
			}
			if a >= b {
				break;
			}
		}
		res
	}
	fn minimax_move(&mut self, depth: u32) -> <Tablut as Game>::M {
		if self.g.state() != State::Going || depth == 0 {
			panic!();
		}
		let mut a = i64::MIN;
		let mut b = i64::MAX;
		let moves = self.g.get_moves_sorted();
		let mut res = if self.g.turn() { a } else { b };
		let mut ans = moves[0];
		for m in moves.iter() {
			let dif = if m.0 > m.1 { m.0 - m.1 } else { m.1 - m.0 };
			let dist = if dif >= 9 { dif / 9 } else { dif };
			self.mdftot[dist as usize] += 1;
			self.mftot[(m.0 * 81 + m.1) as usize] += 1;
			let rb = self.g.mov_with_rollback(m);
			let h = self.minimax(a, b, depth - 1);
			self.g.rollback(rb);
			if self.g.turn() {
				if h > res {
					res = h;
					ans = *m;
				}
				a = a.max(h);
			} else {
				if h < res {
					res = h;
					ans = *m;
				}
				b = b.min(h);
			}
			/*if a >= b {
				break;
			}*/
		}
		let dif = if ans.0 > ans.1 {
			ans.0 - ans.1
		} else {
			ans.1 - ans.0
		};
		let dist = if dif >= 9 { dif / 9 } else { dif };
		self.mdf[dist as usize] += 1;
		self.mf[(ans.0 * 81 + ans.1) as usize] += 1;
		ans
	}
	pub fn print_stats(&self) {
		for i in 0..(81 * 81) {
			print!("{} ", (self.mf[i] + 1) as f32 / (self.mftot[i] + 2) as f32);
		}
		println!();
		for i in 0..9 {
			print!(
				"{} ",
				(self.mdf[i] + 1) as f32 / (self.mdftot[i] + 2) as f32
			);
		}
		println!();
	}
}

impl Ai<Tablut> for MinimaxTablut {
	fn new(t: bool) -> Self {
		Self {
			g: Tablut::new(t),
			mf: [0usize; 81 * 81],
			mftot: [0usize; 81 * 81],
			mdf: [0usize; 9],
			mdftot: [0usize; 9],
		}
	}
	fn state(&self) -> State {
		self.g.state()
	}
	fn print2game(&self) {
		eprintln!("{}", self.g)
	}
	fn turn(&self) -> bool {
		self.g.turn()
	}
	fn get_mov(&mut self) -> <Tablut as Game>::M {
		let start_time = Instant::now();
		let mut depth = 1;
		let mut ans = self.minimax_move(1);
		loop {
			if start_time.elapsed().as_millis() * 20 > 2000 {
				break;
			}
			depth += 1;
			ans = self.minimax_move(depth);
		}
		eprintln!(
			"minimax_tablut chose move in {} milliseconds with {} depth",
			start_time.elapsed().as_millis(),
			depth
		);
		ans
	}
	fn mov(&mut self, m: &<Tablut as Game>::M) {
		self.g.mov(m);
	}
}
