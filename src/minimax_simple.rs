use crate::ai::Ai;
use crate::game::*;
use std::time::SystemTime;

pub struct MinimaxSimple<G: Game> {
	pub g: G,
}

impl<G: Game> MinimaxSimple<G> {
	fn minimax(&mut self, mut a: i64, mut b: i64, depth: u32) -> i64 {
		if self.g.state() != State::Going || depth == 0 {
			return self.g.heuristic();
		}
		let moves = self.g.get_moves_sorted();
		let mut res = if self.g.turn() { a } else { b };
		for m in moves.iter() {
			self.g.mov(m);
			let h = self.minimax(a, b, depth - 1);
			self.g.rollback();
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
	fn minimax_move(&mut self, depth: u32) -> G::M {
		if self.g.state() != State::Going || depth == 0 {
			panic!();
		}
		let mut a = i64::MIN;
		let mut b = i64::MAX;
		let moves = self.g.get_moves_sorted();
		let mut res = if self.g.turn() { a } else { b };
		let mut ans = moves[0];
		for m in moves.iter() {
			self.g.mov(m);
			let h = self.minimax(a, b, depth - 1);
			self.g.rollback();
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
			if a >= b {
				break;
			}
		}
		ans
	}
}

impl<G: Game> Ai<G> for MinimaxSimple<G> {
	fn new(t: bool) -> Self {
		Self { g: G::new(t) }
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
	fn get_mov(&mut self) -> G::M {
		let start_time = SystemTime::now();
		let mut depth = 1;
		let mut ans = self.minimax_move(1);
		loop {
			if start_time.elapsed().unwrap().as_millis() * 10 > 500 {
				break;
			}
			depth += 1;
			ans = self.minimax_move(depth);
		}
		eprintln!(
			"minimax_simple chose move in {} milliseconds with {} depth",
			start_time.elapsed().unwrap().as_millis(),
			depth
		);
		ans
	}
	fn mov(&mut self, m: &G::M) {
		self.g.mov(m);
	}
}
