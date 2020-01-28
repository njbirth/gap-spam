/*	Copyright (C) 2020 - Niklas Birth

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use gaps_rust::opt::Opt;
use structopt::StructOpt;

fn main() {
	let opt = Opt::from_args();
	match opt {
		Opt::QTrees(opt_qtrees) => {
			gaps_rust::run_qtrees(opt_qtrees).unwrap();
		},
		Opt::Pars(opt_pars) => {
			gaps_rust::run_pars(opt_pars).unwrap();
		},
		Opt::Nwk(opt_nwk) => {
			gaps_rust::run_nwk(opt_nwk).unwrap();
		},
	}
}