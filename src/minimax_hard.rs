use crate::ai::Ai;
use crate::game::*;
use std::collections::HashMap;
use std::time::SystemTime;

pub struct MinimaxHard<G: Game> {
	pub g: G,
	table: HashMap<G::S, (i64, u32)>,
}

impl<G: Game> MinimaxHard<G> {
	fn minimax(&mut self, mut a: i64, mut b: i64, depth: u32) -> i64 {
		if self.g.state() != State::Going || depth == 0 {
			return self.g.heuristic();
		}
		let mut old_depth = 0;
		if let Some(x) = self.table.get(&self.g.get_static_state()) {
			if depth <= x.1 || (depth<=x.1+4 && depth>4 && depth<14) {
				return x.0;
			}
			old_depth = x.1;
		}
		let mut res = if self.g.turn() { a } else { b };
		let mut moves = self.g.get_moves();
		moves.sort_by_cached_key(|m| {
			self.g.mov(m);
			let ans = self
				.table
				.get(&self.g.get_static_state())
				.unwrap_or(&(res, 0))
				.0;
			self.g.rollback();
			if self.g.turn() {
				-ans
			} else {
				ans
			}
		});
		for m in moves.iter()/*.take(if depth > 3 { 3 } else { 6 })*/ {
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
		if depth > old_depth && depth > 4 {
			self.table.insert(self.g.get_static_state(), (res, depth));
		}
		res
	}
	fn minimax_move(&mut self, depth: u32) -> G::M {
		if self.g.state() != State::Going || depth == 0 {
			panic!();
		}
		let mut a = i64::MIN;
		let mut b = i64::MAX;
		let mut old_depth = 0;
		if let Some(x) = self.table.get(&self.g.get_static_state()) {
			old_depth = x.1;
		}
		let mut res = if self.g.turn() { a } else { b };

		let mut moves = self.g.get_moves();
		let mut ans = moves[0];
		moves.sort_by_cached_key(|m| {
			self.g.mov(m);
			let ans = self
				.table
				.get(&self.g.get_static_state())
				.unwrap_or(&(res, 0))
				.0;
			self.g.rollback();
			if self.g.turn() {
				-ans
			} else {
				ans
			}
		});
		for m in moves.iter().take(if depth > 3 { 3 } else { 6 }) {
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
		if depth > old_depth && depth > 4 {
			self.table.insert(self.g.get_static_state(), (res, depth));
		}
		ans
	}
}

impl<G: Game> Ai<G> for MinimaxHard<G> {
	fn new(t: bool) -> Self {
		Self {
			g: G::new(t),
			table: HashMap::new(),
		}
	}
	fn state(&self) -> State {
		self.g.state()
	}
	fn turn(&self) -> bool {
		self.g.turn()
	}
	fn get_mov(&mut self) -> G::M {
		let start_time = SystemTime::now();
		let mut depth = 1;
		let mut ans = self.minimax_move(1);
		loop {
			if start_time.elapsed().unwrap().as_millis() > 250 {
				break;
			}
			depth += 1;
			ans = self.minimax_move(depth);
		}
		eprintln!(
			"minimax_hard chose move in {} milliseconds with {} depth",
			start_time.elapsed().unwrap().as_millis(),
			depth
		);
		ans
	}
	fn mov(&mut self, m: &G::M) {
		self.g.mov(m);
	}
}
