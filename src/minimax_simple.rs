use crate::ai::Ai;
use crate::game::*;
use std::time::Duration;
use std::time::Instant;

pub struct MinimaxSimple<G: Game> {
	pub g: G,
	nnw: u8,
	tl: Duration,
	st: Instant,
	last_ans: G::M,
	ended_early: bool,
}

impl<G: Game> MinimaxSimple<G> {
	fn minimax(&mut self, mut a: i64, mut b: i64, depth: u32) -> i64 {
		if self.g.state() != State::Going || depth == 0 {
			return self.g.heuristic();
		}
		let mut res = if self.g.turn() { a } else { b };
		self.nnw = self.nnw.wrapping_add(1);
		if self.ended_early || (self.nnw == 0 && self.st.elapsed() > self.tl) {
			self.ended_early = true;
			return res;
		}
		let moves = self.g.get_moves_sorted();
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
			if a >= b || self.ended_early {
				break;
			}
		}
		res
	}
	fn minimax_move(&mut self, depth: u32) -> bool {
		let mut a = i64::MIN;
		let mut b = i64::MAX;
		let moves = self.g.get_moves_sorted();
		let mut res = if self.g.turn() { a } else { b };
		let mut ans = moves[0];
		for m in moves.iter() {
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
			if a >= b || self.ended_early {
				break;
			}
		}
		if self.ended_early {
			true
		} else {
			self.last_ans = ans;
			false
		}
	}
}

impl<G: Game> Ai<G> for MinimaxSimple<G> {
	fn new(t: bool) -> Self {
		Self {
			g: G::new(t),
			nnw: 0,
			tl: Duration::ZERO,
			st: Instant::now(),
			last_ans: G::M::default(),
			ended_early: false,
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
	fn get_mov(&mut self, tl: Duration) -> G::M {
		let mut depth = 1;
		self.tl = tl - Duration::from_millis(20);
		self.st = Instant::now();
		self.ended_early = false;
		while !self.minimax_move(depth) {
			depth += 1;
		}
		eprintln!("minimax_simple depth {}", depth - 1);
		self.last_ans
	}
	fn mov(&mut self, m: &G::M) {
		self.g.mov(m);
	}
}
