use std::hash::Hash;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
	Win,
	Lose,
	Draw,
	Going,
}
pub trait Game {
	type M: Copy;
	type S: Hash + Copy + Eq;
	fn new(t: bool) -> Self;
	fn turn(&self) -> bool;
	fn get_moves(&self) -> Vec<Self::M>;
	fn get_static_state(&self) -> Self::S;
	fn state(&self) -> State;
	fn heuristic(&self) -> i64;
	fn mov(&mut self, m: &Self::M);
	fn rollback(&mut self);
}
