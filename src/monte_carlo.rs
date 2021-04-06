use crate::ai::Ai;
use crate::game::*;
use rand::prelude::SliceRandom;
use rand::*;
use rand_xoshiro::Xoroshiro128Plus;
use std::collections::HashMap;
use std::time::SystemTime;

pub struct MonteCarlo<G: Game> {
	pub g: G,
	rng: Xoroshiro128Plus,
	table: HashMap<G::S, (u32, u32)>,
}

impl<G: Game> MonteCarlo<G> {
	fn explore_branch(&mut self, turn: bool) -> u32 {
		let mut rb = 0usize;
		while self.g.state() == State::Going {
			let moves = self.g.get_moves();
			let m = moves.choose(&mut self.rng).unwrap();
			self.g.mov(&m);
			rb += 1;
		}
		let mut ans = match self.g.state() {
			State::Win => 1u32,
			State::Lose => 0u32,
			_ => self.rng.next_u32() & 1,
		};
		if !turn {
			ans = 1 - ans;
		}
		for _ in 0..rb {
			self.g.rollback();
		}
		ans
	}
	fn step(&mut self, turn: bool) -> bool {
		let mut nm0 = 0;
		if !self.table.contains_key(&self.g.get_static_state()) {
			self.table.insert(self.g.get_static_state(), (0, 0));
		}
		loop {
			if self.g.state() != State::Going {
				for _ in 0..nm0 {
					self.g.rollback();
				}
				return false;
			}
			let moves = self.g.get_moves();
			let mut found = false;
			let mut best = moves[0];
			let mut value = 0.0f32;
			for m in moves.iter() {
				let np = self
					.table
					.get(&self.g.get_static_state())
					.unwrap() /*_or(&(0, 0))*/
					.1 as f32;
				self.g.mov(m);
				if let Some(x) = self.table.get(&self.g.get_static_state()) {
					let val = (x.0 as f32 / x.1 as f32) + 1.5 * (np.ln() / (x.1 as f32));
					if val > value {
						value = val;
						best = *m;
					}
					self.g.rollback();
				} else {
					best = *m;
					found = true;
					self.g.rollback();
					break;
				}
			}
			self.g.mov(&best);
			nm0 += 1;
			if found {
				break;
			}
		}
		let mc = self.explore_branch(turn);
		self.table.insert(self.g.get_static_state(), (mc, 1));
		for _ in 0..nm0 {
			self.g.rollback();
			let x = self.table.get_mut(&self.g.get_static_state()).unwrap();
			x.0 += mc;
			x.1 += 1;
		}
		return true;
	}
}

impl<G: Game> Ai<G> for MonteCarlo<G> {
	fn new(t: bool) -> Self {
		Self {
			g: G::new(t),
			rng: Xoroshiro128Plus::seed_from_u64(124),
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
		let moves = self.g.get_moves();
		let turn = self.g.turn();
		let mut i = 0;
		while self.step(turn) {
			if start_time.elapsed().unwrap().as_millis() > 250 {
				break;
			}
			i += 1;
		}
		let mut best_mov = moves[0];
		let mut best_val = 0.0;
		for m in moves {
			self.g.mov(&m);
			let x = self
				.table
				.get(&self.g.get_static_state())
				.unwrap_or(&(0, 0));
			self.g.rollback();
			let val = (x.0 as f32 + 1.0) / (x.1 as f32 + 2.0);
			eprintln!("{}/{}", x.0, x.1);
			if val > best_val {
				best_val = val;
				best_mov = m;
			}
		}
		eprintln!(
			"monte_carlo chose move in {} milliseconds with {} iterations",
			start_time.elapsed().unwrap().as_millis(),
			i
		);
		best_mov
	}
	fn mov(&mut self, m: &G::M) {
		self.g.mov(m);
	}
}
