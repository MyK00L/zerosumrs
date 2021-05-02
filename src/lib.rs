pub mod ai;
pub mod game;
pub mod mancala;
pub mod minimax_final;
pub mod minimax_hard;
pub mod minimax_simple;
pub mod monte_carlo_total;
pub mod monte_carlo_tree_search;
pub mod othello;
pub mod random_agent;
pub mod tablut_with_draw;
//pub mod tablut;
pub mod tictactoe;

use crate::ai::*;
use crate::game::*;
use rand::prelude::SliceRandom;
use rand::*;
use rand_xoshiro::Xoroshiro128Plus;
use std::fmt::Display;
use std::time::Instant;

fn random_play<G: Game>() -> (State, usize) {
	let mut rng = Xoroshiro128Plus::from_rng(rand::thread_rng()).unwrap();
	let mut g = G::new(true);
	let mut nmovs = 0;
	while g.state() == State::Going {
		let moves = g.get_moves();
		let m = moves.choose(&mut rng).unwrap();
		g.mov(&m);
		nmovs += 1;
	}
	(g.state(), nmovs)
}

pub fn print_stats<G: Game>() {
	let mut nw = 0;
	let mut nl = 0;
	let mut nd = 0;
	let mut ne = 0;
	let mut al = 0;
	for _ in 0..1024 {
		let cacca = random_play::<G>();
		al += cacca.1;
		match cacca.0 {
			State::Win => {
				nw += 1;
			}
			State::Lose => {
				nl += 1;
			}
			State::Draw => {
				nd += 1;
			}
			_ => {
				ne += 1;
			}
		}
	}
	eprintln!(
		"win {}\nlos {}\ndrw {}\nerr {}\navg len {}",
		nw,
		nl,
		nd,
		ne,
		al / 1024
	);
}

pub fn compete<G: Game + Display, A: Ai<G>, B: Ai<G>>() {
	eprintln!(
		"Start {} vs {} in {}",
		std::any::type_name::<A>(),
		std::any::type_name::<B>(),
		std::any::type_name::<G>()
	);
	let mut a = A::new(true);
	let mut b = B::new(true);
	let mut tta = 0;
	let mut ttb = 0;
	let mut na = 0;
	let mut nb = 0;
	while a.state() == State::Going {
		let tts = Instant::now();
		let m = match a.turn() {
			true => a.get_mov(),
			false => b.get_mov(),
		};
		if a.turn() {
			na += 1;
			tta += tts.elapsed().as_millis();
		} else {
			nb += 1;
			ttb += tts.elapsed().as_millis();
		}
		a.mov(&m);
		b.mov(&m);
		a.print2game();
	}
	if b.state() != a.state() {
		eprintln!("WTF STATES ARE DESYNCED HELP!!?");
		eprintln!(
			"{} state: {:?}\n{} state: {:?}",
			std::any::type_name::<A>(),
			a.state(),
			std::any::type_name::<B>(),
			b.state()
		);
	}
	eprintln!(
		"{} avg think time: {}ms",
		std::any::type_name::<A>(),
		tta as f64 / na as f64
	);
	eprintln!(
		"{} avg think time: {}ms",
		std::any::type_name::<B>(),
		ttb as f64 / nb as f64
	);
	eprintln!(
		"{}\tvs\t{}",
		std::any::type_name::<A>(),
		std::any::type_name::<B>()
	);
	eprintln!(
		"{}\t-\t{}",
		if a.state() == State::Win { 1 } else { 0 },
		if a.state() == State::Lose { 1 } else { 0 }
	);
}

#[cfg(test)]
mod tests {
	use crate::ai::*;
	use crate::game::*;
	use crate::mancala::*;
	//use crate::minimax_final::*;
	//use crate::minimax_hard::*;
	//use crate::minimax_simple::*;
	//use crate::monte_carlo_total::*;
	//use crate::monte_carlo_tree_search::*;
	use crate::othello::*;
	use crate::random_agent::*;
	use crate::tablut_with_draw::*;
	//use crate::tablut::*;
	use crate::tictactoe::*;

	fn test_rollback<G: Game, A: Ai<G>, B: Ai<G>>() {
		let mut a = A::new(true);
		let mut b = B::new(true);
		let mut g = G::new(true);
		let mut v = vec![(g.clone(), G::R::default())];
		while g.state() == State::Going {
			let m = match g.turn() {
				true => a.get_mov(),
				false => b.get_mov(),
			};
			let rb = g.mov_with_rollback(&m);
			a.mov(&m);
			b.mov(&m);
			v.push((g.clone(), rb));
		}
		while !v.is_empty() {
			let x = v.pop().unwrap();
			assert_eq!(x.0.get_static_state(), g.get_static_state());
			if !v.is_empty() {
				g.rollback(x.1);
			}
		}
	}
	fn test_rollback_game<G: Game>() {
		/*test_rollback::<G,MinimaxSimple<G>,RandomAgent<G>>();
		test_rollback::<G,MinimaxHard<G>,RandomAgent<G>>();
		test_rollback::<G,MinimaxFinal<G>,RandomAgent<G>>();
		test_rollback::<G,MonteCarloTotal<G>,RandomAgent<G>>();
		test_rollback::<G,MonteCarloTreeSearch<G>,RandomAgent<G>>();*/
		test_rollback::<G, RandomAgent<G>, RandomAgent<G>>();
	}
	#[test]
	fn rollbacks_test() {
		test_rollback_game::<Mancala>();
		test_rollback_game::<Ttt>();
		test_rollback_game::<Tablut>();
		test_rollback_game::<Othello>();
	}
}
