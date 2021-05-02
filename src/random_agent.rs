use crate::ai::Ai;
use crate::game::*;
use rand::prelude::SliceRandom;
use rand::*;
use rand_xoshiro::Xoroshiro128Plus;

pub struct RandomAgent<G: Game> {
	pub g: G,
	rng: Xoroshiro128Plus,
}

impl<G: Game> Ai<G> for RandomAgent<G> {
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
	fn get_mov(&mut self, _tl: std::time::Duration) -> G::M {
		*self.g.get_moves().choose(&mut self.rng).unwrap()
	}
	fn mov(&mut self, m: &G::M) {
		self.g.mov(m);
	}
}
