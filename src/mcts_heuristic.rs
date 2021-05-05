use crate::game::*;
use crate::tablut::Tablut;
use rand::prelude::SliceRandom;
use rand::*;
use rand_xoshiro::Xoroshiro128Plus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Hash, Eq, PartialEq, Clone, Deserialize, Serialize)]
struct Tree {
	wins: u64,
	vis: u64,
	depth: u64,
	state: <Tablut as Game>::S,
	prev_move: <Tablut as Game>::M,
	children: Vec<Box<Tree>>,
}

impl Tree {
	fn new(depth: u64, state: <Tablut as Game>::S, mov: (u8, u8)) -> Self {
		Self {
			wins: 0,
			vis: 0,
			depth,
			state,
			prev_move: mov,
			children: vec![],
		}
	}

	fn count_nodes(&self) -> (u64, u64) {
		(
			self
				.children
				.iter()
				.map(|child| child.count_nodes().0)
				.sum::<u64>()
				+ 1,
			self.children.len() as u64,
		)
	}
}

pub struct MonteCarloTreeSearch {
	pub g: Tablut,
	rng: Xoroshiro128Plus,
}

impl MonteCarloTreeSearch {
	fn result_u64(&mut self, s: State) -> u64 {
		match s {
			State::Win => 1u64,
			State::Lose => 0u64,
			_ => self.rng.next_u64() & 1,
		}
	}
	fn explore_branch(&mut self) -> u64 {
		let mut gc = self.g.clone();
		while gc.state() == State::Going {
			let moves = gc.get_moves();
			// for mov in &moves {
			// 	gc.mov(mov);
			// 	if let State::Win = gc.state() {
			// 		return self.result_u64(gc.state());
			// 	}
			// }
			let m = moves.choose(&mut self.rng).unwrap();
			gc.mov(&m);
		}
		self.result_u64(gc.state())
	}
	fn step(&mut self, t: &mut Tree) -> u64 {
		let turn = self.g.turn();
		if self.g.state() != State::Going || t.vis == 0 {
			t.vis += 1;
			let mc = self.explore_branch();
			t.wins += mc;
			return mc;
		}
		let mut movs = self.g.get_moves();
		//let ((a, b, c), d) = self.g.get_static_state();
		let mut rng = Xoroshiro128Plus::from_seed([0; 16]);
		movs.shuffle(&mut rng);
		let (mov, i) = if t.children.len() < movs.len() {
			self.g.mov(&movs[t.children.len()]);
			let child = Box::new(Tree::new(
				t.depth + 1,
				self.g.get_static_state(),
				movs[t.children.len()],
			));

			t.children.push(child);

			//self.g.rollback();
			(movs[t.children.len() - 1], t.children.len() - 1)
		} else {
			let mut best_val = 0.0f32;
			let mut ans = ((0, 0), 0);
			for (i, x) in t.children.iter().enumerate() {
				let val = (if turn { x.wins } else { x.vis - x.wins }) as f32 / x.vis as f32
					+ 1.2 * ((t.vis as f32).ln() / (x.vis as f32)).sqrt();
				if val > best_val {
					best_val = val;
					ans = (x.prev_move, i);
				}
			}
			ans
		};
		self.g.mov(&mov);
		let x = self.step(&mut t.children[i]);
		//self.g.rollback();
		t.wins += x;
		t.vis += 1;
		x
	}
}
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
impl MonteCarloTreeSearch {
	fn read_from_file() -> Result<(Box<Tree>, u32), Box<dyn std::error::Error>> {
		Ok(bincode::deserialize(&std::fs::read("tree.bincode")?)?)
	}

	pub fn new(t: bool) -> Self {
		let g = Tablut::new(t);
		Self {
			rng: Xoroshiro128Plus::from_rng(rand::thread_rng()).unwrap(),
			g,
		}
	}

	pub fn explore(&mut self, running: Arc<AtomicBool>) {
		let start_time = Instant::now();
		let moves = self.g.get_moves();
		println!("possible moves: {}", moves.len());
		let mut i = 0;
		// tree will not be read until we put t inside it again
		let (mut tree, mut depth) = Self::read_from_file().unwrap_or_else(|e| {
			eprintln!("cloud not deserialize {}", e);
			(Box::new(Tree::new(0, self.g.get_static_state(), (0, 0))), 0)
		});
		eprintln!("num states: {:?} | depth {}", tree.count_nodes(), depth);
		let mut t = &mut tree;
		// retraverse the tree until the last step
		while depth > 0 {
			let mut best_mov = (0, 0);
			let mut best_val = 0;
			let mut idx = 0;
			for (i, t) in t.children.iter().enumerate() {
				let val = t.vis;
				if val > best_val {
					idx = i;
					best_val = val;
					best_mov = t.prev_move;
				}
			}
			println!("moving down");
			t = &mut t.children[idx];
			self.g.mov(&best_mov);
			depth -= 1;
		}
		let mut last = Instant::now();
		loop {
			let tt = Instant::now();
			for _ in 0..1024 {
				self.step(t);
			}
			println!("elapsed: {}", tt.elapsed().as_millis());
			assert_eq!(t.children.len(), self.g.get_moves().len());
			i += 1024;
			if (i % 65536) == 0 {
				println!("iteration {}", i);
			}

			if last.elapsed().as_secs() > 60 * 30 {
				last = Instant::now();
				println!(
					"pruning tree, continuing with best move so far, depth {}",
					t.depth
				);
				Self::prune(t);
				let mut best_mov = (0, 0);
				let mut best_val = 0;
				let mut idx = 0;
				for (i, t) in t.children.iter().enumerate() {
					let val = t.vis;
					if val > best_val {
						idx = i;
						best_val = val;
						best_mov = t.prev_move;
					}
				}
				t = &mut t.children[idx];
				self.g.mov(&best_mov);
				if self.g.state() != State::Going {
					println!("game finished");
					break;
				}
			}
			if !running.load(Ordering::SeqCst) {
				break;
			}
		}
		let depth = t.depth;
		//		eprintln!("pre-prune: {:?}", tree.count_nodes());
		let mut save = HashMap::new();
		let mut levels = HashMap::new();
		let mut levels1 = HashMap::new();
		Self::get_stats(&tree, &mut save, &mut levels, &mut levels1);
		eprintln!(
			"filtered: {} | levels views {:?} | levels states {:?}",
			save.len(),
			levels,
			levels1
		);
		save.clear();
		levels.clear();
		levels1.clear();

		Self::prune(&mut tree);
		std::fs::write(
			format!("tree{}.bincode", self.rng.next_u64()),
			bincode::serialize(&(&tree, depth)).unwrap(),
		)
		.unwrap();
		let mut save = HashMap::new();
		let mut levels = HashMap::new();
		let mut levels1 = HashMap::new();
		Self::get_stats(&tree, &mut save, &mut levels, &mut levels1);
		eprintln!(
			"filtered: {} | levels views {:?} | levels states {:?}",
			save.len(),
			levels,
			levels1
		);
		eprintln!(
			"monte_carlo_tree_search chose move in {} milliseconds with {} iterations| {}/{}",
			start_time.elapsed().as_millis(),
			i,
			tree.wins,
			tree.vis,
		);
		std::fs::write("states.bincode", bincode::serialize(&save).unwrap()).unwrap();
		//		eprintln!("states: {:?}", tree.count_nodes());
	}

	fn prune(tree: &mut Tree) {
		let mut roots = vec![tree];
		while !roots.is_empty() {
			let root = roots.pop().unwrap();
			root.children.retain(|child| child.vis > 3);
			for child in &mut root.children {
				roots.push(child);
			}
		}
	}

	fn get_stats(
		tree: &Tree,
		save: &mut HashMap<<Tablut as Game>::S, u64>,
		lev: &mut HashMap<u64, u64>,
		lev1: &mut HashMap<u64, u64>,
	) {
		let mut levels: HashMap<(<Tablut as Game>::S, u64), u64> = HashMap::new();
		let mut roots = vec![tree];
		while !roots.is_empty() {
			let root = roots.pop().unwrap();
			for child in &root.children {
				let res = save.entry(child.state).or_insert(0);
				*res += child.vis;

				let res = levels.entry((child.state, child.depth)).or_insert(0);
				*res += child.vis;
				roots.push(child);
			}
		}

		for (k, v) in levels.iter() {
			*lev.entry(k.1).or_insert(0) += v;
			*lev1.entry(k.1).or_insert(0) += 1;
		}
	}
}
