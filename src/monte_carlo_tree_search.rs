use crate::ai::Ai;
use crate::game::*;
use rand::prelude::SliceRandom;
use rand::*;
use rand_xoshiro::Xoroshiro128Plus;
use std::time::SystemTime;

struct Tree<G: Game> {
	wins: u32,
	vis: u32,
	movs: Vec<G::M>,
	children: Vec<Tree<G>>,
}
impl<G: Game> Tree<G> {
	fn new() -> Self {
		Self {
			wins: 0,
			vis: 0,
			movs: vec![],
			children: vec![],
		}
	}
}
impl<G: Game> Default for Tree<G> {
	fn default() -> Self {
		Tree::<G>::new()
	}
}

pub struct MonteCarloTreeSearch<G: Game> {
	pub g: G,
	rng: Xoroshiro128Plus,
	tree: Tree<G>,
}

impl<G: Game> MonteCarloTreeSearch<G> {
	fn result_u32(&mut self, s: State) -> u32 {
		match s {
			State::Win => 1u32,
			State::Lose => 0u32,
			_ => self.rng.next_u32() & 1,
		}
	}
	fn explore_branch(&mut self) -> u32 {
		let mut gc = self.g.clone();
		while gc.state() == State::Going {
			let moves = gc.get_moves();
			let m = moves.choose(&mut self.rng).unwrap();
			gc.mov(&m);
		}
		self.result_u32(gc.state())
	}
	fn step(&mut self, t: &mut Tree<G>) -> u32 {
		let turn = self.g.turn();
		if self.g.state() != State::Going || t.vis == 0 {
			t.vis += 1;
			let mc = self.explore_branch();
			t.wins += mc;
			return mc;
		}
		if t.movs.is_empty() {
			t.movs = self.g.get_moves();
		}
		let movi = if t.children.len() < t.movs.len() {
			t.children.push(Tree::<G>::new());
			t.children.len() - 1
		} else {
			let mut best_val = 0.0f32;
			let mut ans = 0;
			for (i, x) in t.children.iter().enumerate() {
				let val = (if turn { x.wins } else { x.vis - x.wins }) as f32 / x.vis as f32
					+ 1.5 * ((t.vis as f32).ln() / (x.vis as f32)).sqrt();
				if val > best_val {
					best_val = val;
					ans = i;
				}
			}
			ans
		};
		self.g.mov(&t.movs[movi]);
		let x = self.step(&mut t.children[movi]);
		self.g.rollback();
		t.wins += x;
		t.vis += 1;
		return x;
	}
}

impl<G: Game> Ai<G> for MonteCarloTreeSearch<G> {
	fn new(t: bool) -> Self {
		Self {
			g: G::new(t),
			rng: Xoroshiro128Plus::from_rng(rand::thread_rng()).unwrap(),
			tree: Tree::<G>::new(),
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
		let mut t = std::mem::take(&mut self.tree);
		loop {
			for _ in 0..128 {
				self.step(&mut t);
			}
			i += 128;
			if start_time.elapsed().unwrap().as_millis() > 250 {
				break;
			}
		}
		self.tree = std::mem::take(&mut t);
		let mut best_mov = moves[0];
		let mut best_val = 0.0;
		let turn = self.g.turn();
		for (i, t) in self.tree.children.iter().enumerate() {
			//let val = (if turn { t.wins } else { t.vis - t.wins }) as f32 / t.vis as f32
			//	+ 1.6 * ((self.tree.vis as f32).ln() / (t.vis as f32));
			let val = (if turn { t.wins } else { t.vis - t.wins }+1) as f32 / (t.vis+2) as f32;
			eprintln!(
				"{} / {} -> {}",
				if turn { t.wins } else { t.vis - t.wins },
				t.vis,
				val
			);
			if val > best_val {
				best_val = val;
				best_mov = self.tree.movs[i];
			}
		}
		eprintln!(
			"monte_carlo_tree_search chose move in {} milliseconds with {} iterations | confidence: {}",
			start_time.elapsed().unwrap().as_millis(),
			i,
			best_val,
		);
		best_mov
	}
	fn mov(&mut self, m: &G::M) {
		self.g.mov(m);
		let mut t = std::mem::take(&mut self.tree);
		let mut movi = 0;
		for (i, mov) in t.movs.iter().enumerate() {
			if *mov == *m {
				movi = i;
				break;
			}
		}
		if t.children.len() > movi {
			self.tree = std::mem::take(&mut t.children[movi])
		}
	}
}
