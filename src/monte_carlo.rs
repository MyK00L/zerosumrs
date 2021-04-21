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
	fn explore_branch(&mut self) -> u32 {
		let mut gc = self.g.clone();
		while gc.state() == State::Going {
			let moves = gc.get_moves();
			let m = moves.choose(&mut self.rng).unwrap();
			gc.mov(&m);
		}
		match gc.state() {
			State::Win => 1u32,
			State::Lose => 0u32,
			_ => self.rng.next_u32() & 1,
		}
	}
	fn step(&mut self) -> bool {
		let mut nm0 = 0;
		self
			.table
			.entry(self.g.get_static_state())
			.or_insert((0, 0));
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
			let mut best_val = 0.0f32;
			let turn = self.g.turn();
			for m in moves.iter() {
				let np = self.table.get(&self.g.get_static_state()).unwrap().1 as f32;
				self.g.mov(m);
				if let Some(x) = self.table.get(&self.g.get_static_state()) {
					let val = (if turn { x.0 } else { x.1 - x.0 } as f32 / x.1 as f32)
						+ 1.5 * (np.ln() / (x.1 as f32)).sqrt();
					if val > best_val {
						best_val = val;
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
		let mc = self.explore_branch();
		self.table.insert(self.g.get_static_state(), (mc, 1));
		for _ in 0..nm0 {
			self.g.rollback();
			let x = self.table.get_mut(&self.g.get_static_state()).unwrap();
			x.0 += mc;
			x.1 += 1;
		}
		true
	}
}

impl<G: Game> Ai<G> for MonteCarlo<G> {
	fn new(t: bool) -> Self {
		Self {
			g: G::new(t),
			rng: Xoroshiro128Plus::from_rng(rand::thread_rng()).unwrap(),
			table: HashMap::new(),
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
	fn get_mov(&mut self) -> G::M {
		let start_time = SystemTime::now();
		let moves = self.g.get_moves();
		let mut i = 0;
		while self.step() {
			for _ in 0..128 {
				self.step();
				i += 1;
			}
			if start_time.elapsed().unwrap().as_millis() > 250 {
				break;
			}
			i += 1;
		}
		let mut best_mov = moves[0];
		let mut best_val = 0.0;
		let turn = self.g.turn();
		for m in moves {
			self.g.mov(&m);
			let x = self
				.table
				.get(&self.g.get_static_state())
				.unwrap_or(&(1, 2));
			self.g.rollback();
			let val = (if turn { x.0 } else { x.1 - x.0 } as f32) / (x.1 as f32);
			if val > best_val {
				best_val = val;
				best_mov = m;
			}
		}
		eprintln!(
			"monte_carlo chose move in {} milliseconds with {} iterations | confidence: {}",
			start_time.elapsed().unwrap().as_millis(),
			i,
			best_val
		);
		best_mov
	}
	fn mov(&mut self, m: &G::M) {
		self.g.mov(m);
	}
}
