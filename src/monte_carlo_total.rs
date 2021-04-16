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
		let mut gc = self.g.clone();
		gc.mov(&m0);
		while gc.state() == State::Going {
			let moves = gc.get_moves();
			let m = moves.choose(&mut self.rng).unwrap();
			gc.mov(&m);
		}
		let mut ans = match gc.state() {
			State::Win => 1,
			State::Lose => 0,
			_ => self.rng.next_u32() % 2,
		};
		if !turn {
			ans = 1 - ans;
		}
		ans
	}
}

impl<G: Game> Ai<G> for MonteCarloTotal<G> {
	fn new(t: bool) -> Self {
		Self {
			g: G::new(t),
			rng: Xoroshiro128Plus::from_rng(rand::thread_rng()).unwrap(),
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
		let turn = self.g.turn();
		let mut v = vec![0u32; moves.len()];
		let mut i = 0;
		loop {
			if start_time.elapsed().unwrap().as_millis() > 250 {
				break;
			}
			i += 1;
			for mm in moves.iter().enumerate() {
				v[mm.0] += self.explore_branch(mm.1, turn);
			}
		}
		let best_ind = v.iter().enumerate().max_by_key(|x| x.1).unwrap().0;
		let ans = moves[best_ind];
		eprintln!(
			"monte_carlo_total chose move in {} milliseconds with {} iterations | confidence: {}",
			start_time.elapsed().unwrap().as_millis(),
			i,
			v[best_ind] as f32 / i as f32,
		);
		ans
	}
	fn mov(&mut self, m: &G::M) {
		self.g.mov(m);
	}
}
