use crate::game::*;

pub trait Ai<G: Game> {
	fn new(t: bool) -> Self;
	fn state(&self) -> State;
	fn turn(&self) -> bool;
	fn get_mov(&mut self) -> G::M;
	fn mov(&mut self, m: &G::M);
}

pub fn compete<G: Game, A: Ai<G>, B: Ai<G>>() -> State {
	let mut a = A::new(true);
	let mut b = B::new(true);
	while a.state() == State::Going {
		let m = match a.turn() {
			true => a.get_mov(),
			false => b.get_mov(),
		};
		a.mov(&m);
		b.mov(&m);
	}
	if b.state() != a.state() {
		eprintln!("WTF STATES ARE DESYNCED HELP!!?");
		eprintln!("A state: {:?}\nB state: {:?}", a.state(), b.state());
	}
	a.state()
}
