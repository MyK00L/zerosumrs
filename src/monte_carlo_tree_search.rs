use crate::ai::Ai;
use crate::game::*;
use crate::minimax_simple::MinimaxSimple;
use rand::prelude::SliceRandom;
use rand::*;
use rand_xoshiro::Xoroshiro128Plus;
use std::time::SystemTime;

struct Tree<G: Game> {
	wins: f32,
	vis: f32,
	movs: Vec<G::M>,
	children: Vec<Tree<G>>,
}
impl<G: Game> Tree<G> {
	fn new() -> Self {
		Self {
			wins: 0.0,
			vis: 0.0,
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
use std::collections::HashMap;
pub struct MonteCarloTreeSearch<G: Game> {
	pub g: G,
	rng: Xoroshiro128Plus,
	tree: Tree<G>,
	games: HashMap<G::S, f32>,
	hits: u32,
	tot: u32,
}

impl<G: Game> MonteCarloTreeSearch<G> {
	fn result_u32(&self, s: State) -> f32 {
		match s {
			State::Win => 1.0,
			State::Lose => 0.0,
			_ => 0.5,
		}
	}
	fn explore_branch(&mut self) -> f32 {
		//println!("game started {:?}", self.g.state());
		let mut a = MinimaxSimple::new_with_game(self.g);
		let mut b = MinimaxSimple::new_with_game(self.g);

		let mut i = 0;
		let init = self.g;
		if let Some(res) = self.games.get(&init.get_static_state()) {
			self.hits += 1;
			return *res;
		}
		while a.state() == State::Going {
			let m = match a.turn() {
				true => a.get_mov_with_fixed_depth(2),
				false => b.get_mov_with_fixed_depth(2),
			};
			a.mov(&m);
			b.mov(&m);
			//a.print2game();
			i += 1;
			if i > 50 {
				//println!("Game finished draw");
				let res = self.result_u32(State::Draw);
				self.games.insert(init.get_static_state(), res);
				return res;
			}
			// if a.state() != b.state() {
			// 	panic!("{:?}, {:?}", a.state(), b.state());
			// }
		}
		let res = self.result_u32(a.state());
		self.games.insert(init.get_static_state(), res);
		//println!("game finished {:?}", i);
		self.result_u32(a.state())
	}
	fn step(&mut self, t: &mut Tree<G>) -> f32 {
		let turn = self.g.turn();
		if self.g.state() != State::Going || t.vis as u32 == 0 {
			t.vis += 1.0;
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
					+ 1.3 * ((t.vis as f32).ln() / (x.vis as f32)).sqrt();
				if val > best_val {
					best_val = val;
					ans = i;
				}
			}
			ans
		};
		self.g.mov(&t.movs[movi]);
		let x = self.step(&mut t.children[movi]);
		t.wins += x;
		t.vis += 1.0;
		x
	}
}

impl<G: Game> Ai<G> for MonteCarloTreeSearch<G> {
	fn new(t: bool) -> Self {
		Self {
			g: G::new(t),
			rng: Xoroshiro128Plus::from_rng(rand::thread_rng()).unwrap(),
			tree: Tree::<G>::new(),
			games: HashMap::new(),
			hits: 0,
			tot: 0,
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
		let g0 = self.g;
		loop {
			for _ in 0..128 {
				self.step(&mut t);
				self.g = g0;
			}
			i += 128;
			self.tot += 128;
			if start_time.elapsed().unwrap().as_secs() > 1 {
				break;
			}
		}
		self.tree = std::mem::take(&mut t);
		let mut best_mov = moves[0];
		let mut best_val = 0;
		let mut p = 0;
		//let turn = self.g.turn();
		let mut v = self
			.tree
			.children
			.iter()
			.map(|n| (n.vis as u32, n.wins as u32))
			.collect::<Vec<_>>();
		v.sort();
		v.reverse();
		for t in v {
			print!("{}/{} ", t.1 as u32, t.0 as u32);
		}
		println!();
		for (i, t) in self.tree.children.iter().enumerate() {
			let val = t.vis as u32;
			if val > best_val {
				best_val = val;
				p = t.wins as u32;
				best_mov = self.tree.movs[i];
			}
			let rb = self.g.mov_with_rollback(&self.tree.movs[i]);
			if self.g.state() == State::Win {
				best_val = u32::MAX;
				best_mov = self.tree.movs[i];
			}
			self.g.rollback(rb);
		}
		eprintln!(
			"monte_carlo_tree_search chose move in {} milliseconds with {} iterations | cache: {}/{} | prob: {}/{}",
			start_time.elapsed().unwrap().as_millis(),
			i,
			self.hits,
			self.tot,
			p,
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
