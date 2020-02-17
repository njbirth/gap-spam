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

use std::env;
use std::fs;
use std::path::PathBuf;
use rand;

pub fn create_tmp_folder() -> PathBuf {
	let mut tmp_dir = env::temp_dir();
	let r: u64 = rand::random();
	tmp_dir.push(format!("gaps_rs_{}", r));
	fs::create_dir(&tmp_dir).unwrap();
	tmp_dir
}