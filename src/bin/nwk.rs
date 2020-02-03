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

use gaps_rs::opt::Nwk;
use structopt::StructOpt;

fn main() {
	let opt = Nwk::from_args();

	if opt.pars {
		println!("pars not implemented yet!");
		return;
	}

	println!("{}", gaps_rs::build_tree::max_cut_from_file(&opt.infile));
}