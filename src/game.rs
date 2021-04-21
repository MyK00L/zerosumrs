use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
	Win,
	Lose,
	Draw,
	Going,
}
pub trait Game: Clone + Debug + Display {
	type M: Copy + PartialEq + Eq + Debug;
	type S: Hash + Copy + Eq + Debug;
	fn new(t: bool) -> Self;
	fn turn(&self) -> bool;
	fn get_moves(&self) -> Vec<Self::M>;
	fn get_moves_sorted(&self) -> Vec<Self::M>;
	fn get_static_state(&self) -> Self::S;
	fn state(&self) -> State;
	fn heuristic(&self) -> i64;
	fn mov(&mut self, m: &Self::M);
	fn rollback(&mut self);
}
