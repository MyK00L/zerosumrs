use crate::ai::Ai;
use crate::game::*;
use rand::prelude::SliceRandom;
use rand::*;
use rand_xoshiro::Xoroshiro128Plus;
use std::time::SystemTime;

pub struct MonteCarloTotal<G: Game> {
	pub g: G,
	rng: Xoroshiro128Plus,
}

impl<G: Game> MonteCarloTotal<G> {
	fn explore_branch(&mut self, m0: &G::M, turn: bool) -> u32 {
		self.g.mov(&m0);
		let mut rb = 1usize;
		while self.g.state() == State::Going {
			let moves = self.g.get_moves();
			let m = moves.choose(&mut self.rng).unwrap();
			self.g.mov(&m);
			rb += 1;
		}
		let mut ans = match self.g.state() {
			State::Win => 1,
			State::Lose => 0,
			_ => self.rng.next_u32() % 2,
		};
		if !turn {
			ans = 1 - ans;
		}
		for _ in 0..rb {
			self.g.rollback();
		}
		ans
	}
}

impl<G: Game> Ai<G> for MonteCarloTotal<G> {
	fn new(t: bool) -> Self {
		Self {
			g: G::new(t),
			rng: Xoroshiro128Plus::seed_from_u64(124),
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
		let mut v = vec![(1u32, 2u32); moves.len()];
		let mut i = 0;
		loop {
			if start_time.elapsed().unwrap().as_millis() > 250 {
				break;
			}
			i += 1;
			for mm in moves.iter().enumerate() {
				v[mm.0].1 += 1;
				v[mm.0].0 += self.explore_branch(mm.1, turn);
			}
		}
		let best_ind = v
			.iter()
			.enumerate()
			.max_by(|a, b| ((a.1).0 * (b.1).1).cmp(&((b.1).0 * (a.1).1)))
			.unwrap()
			.0;
		eprintln!("{:?}", v);
		let ans = moves[best_ind];
		eprintln!(
			"monte_carlo_total chose move in {} milliseconds with {} iterations",
			start_time.elapsed().unwrap().as_millis(),
			i
		);
		ans
	}
	fn mov(&mut self, m: &G::M) {
		self.g.mov(m);
	}
}
