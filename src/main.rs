#[allow(unused_imports)]
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
	buffer: Vec<String>,
}
impl Scanner {
	fn next<T: std::str::FromStr>(&mut self) -> T {
		loop {
			if let Some(token) = self.buffer.pop() {
				return token.parse().ok().expect("Failed parse");
			}
			let mut input = String::new();
			stdin().read_line(&mut input).expect("Failed read");
			self.buffer = input.split_whitespace().rev().map(String::from).collect();
		}
	}
}

mod ai;
mod game;
mod mancala;
mod tictactoe;
mod minimax_hard;
mod minimax_simple;
mod monte_carlo;
mod monte_carlo_total;

use ai::*;
use game::*;
use mancala::*;
use tictactoe::*;
use minimax_hard::*;
use minimax_simple::*;
use monte_carlo::*;
use monte_carlo_total::*;

fn main() {
	let mut scan = Scanner::default();
	let out = &mut BufWriter::new(stdout());
	let x = compete::<Ttt, MinimaxHard<Ttt>, MinimaxSimple<Ttt>>();
	write!(out, "{:?}", x);
}
