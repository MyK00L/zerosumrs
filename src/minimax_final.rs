use crate::ai::Ai;
use crate::game::*;
use std::mem::take;
use std::time::Duration;
use std::time::Instant;

struct Tree<G: Game> {
	val: i64,
	depth: u32,
	children: Vec<(G::M, Tree<G>)>,
}
impl<G: Game> Tree<G> {
	fn new() -> Self {
		Self {
			val: 0,
			depth: 0,
			children: vec![],
		}
	}
}
impl<G: Game> Default for Tree<G> {
	fn default() -> Self {
		Tree::<G>::new()
	}
}

pub struct MinimaxFinal<G: Game> {
	pub g: G,
	cur_depth: u32,
	tree: Tree<G>,
	nnw: u8,
	st: Instant,
	tl: Duration,
	ended_early: bool,
}

impl<G: Game> MinimaxFinal<G> {
	// assumes to be called with depth always increased by 1 relative to Tree
	fn minimax(&mut self, mut a: i64, mut b: i64, depth: u32, t: &mut Tree<G>) {
		if self.g.state() != State::Going || depth == 0 {
			t.val = self.g.heuristic();
			t.depth = depth;
			return;
		}
		// if win/loss is certain, no need to check again
		if t.val > 30000 || t.val < -30000 || t.depth >= depth {
			return;
		}

		self.nnw = self.nnw.wrapping_add(1);
		if self.ended_early || (self.nnw == 0 && self.st.elapsed() > self.tl) {
			self.ended_early = true;
			return;
		}

		if t.children.is_empty() {
			t.children = self
				.g
				.get_moves()
				.iter()
				.map(|x| (*x, Tree::<G>::new()))
				.collect();
		} else {
			t.children
				.sort_unstable_by_key(|x| if self.g.turn() { -x.1.val } else { x.1.val });
		}
		// where 42 is maximum change of heuristic in a turn
		// this is bug, pls help
		/*if self.g.turn() {
			a = a.max(t.val-42);
		} else {
			b = b.min(t.val+42);
		}*/
		if self.g.turn() {
			for c in t.children.iter_mut() {
				let rb = self.g.mov_with_rollback(&c.0);
				self.minimax(a, b, depth - 1, &mut c.1);
				let h = c.1.val;
				self.g.rollback(rb);
				a = a.max(h);
				if a >= b {
					break;
				}
			}
		} else {
			for c in t.children.iter_mut() {
				let rb = self.g.mov_with_rollback(&c.0);
				self.minimax(a, b, depth - 1, &mut c.1);
				let h = c.1.val;
				self.g.rollback(rb);
				b = b.min(h);
				if a >= b {
					break;
				}
			}
		}
		t.val = if self.g.turn() { a } else { b };
		t.depth = depth;
	}
}

impl<G: Game> Ai<G> for MinimaxFinal<G> {
	fn new(t: bool) -> Self {
		Self {
			g: G::new(t),
			tree: Tree::new(),
			cur_depth: 0,
			nnw: 0,
			st: Instant::now(),
			tl: Duration::ZERO,
			ended_early: false,
		}
	}
	fn state(&self) -> State {
		self.g.state()
	}
	fn print2game(&self) {
		eprintln!("{}", self.g);
	}
	fn turn(&self) -> bool {
		self.g.turn()
	}
	fn get_mov(&mut self, tl: Duration) -> G::M {
		self.st = Instant::now();
		self.tl = tl - Duration::from_millis(20);
		self.ended_early = false;
		let mut t = take(&mut self.tree);
		while t.val > -30000 && t.val < 30000 && !self.ended_early {
			self.cur_depth += 1;
			self.minimax(i64::MIN, i64::MAX, self.cur_depth, &mut t);
		}
		if self.ended_early && self.cur_depth != 0 {
			self.cur_depth -= 1;
		}
		let ans = t
			.children
			.iter()
			.max_by_key(|x| (x.1.depth, if self.g.turn() { x.1.val } else { -x.1.val }))
			.unwrap()
			.0;
		self.tree = t;
		eprintln!("minimax_final depth {}", self.cur_depth);
		ans
	}
	fn mov(&mut self, m: &G::M) {
		let t = take(&mut self.tree);
		for c in t.children {
			if c.0 == *m {
				self.tree = c.1;
				break;
			}
		}
		if self.cur_depth != 0 {
			self.cur_depth -= 1;
		}
		self.g.mov(m);
	}
}
