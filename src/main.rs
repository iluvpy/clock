use chrono::{Timelike, Utc, DateTime};
use std::time::Duration;
use std::thread;

mod numbers;


fn print_time(now: DateTime<Utc>) {
	let (is_pm, hour) = now.hour12();
	let hour = hour + if is_pm {12} else {0};
	let hour_chars: Vec<char> = hour.to_string().chars().collect();
	let mut hour_numbers: Vec<[&str; 6]> = vec![];
	for ch in hour_chars {
		hour_numbers.push(numbers::get_number(ch));
	}
	let minute_chars: Vec<char> = now.minute().to_string().chars().collect();
	let mut minute_numbers: Vec<[&str; 6]> = vec![];
	for ch in minute_chars {
		minute_numbers.push(numbers::get_number(ch));
	}
	let second_chars: Vec<char> = now.second().to_string().chars().collect();
	let mut second_numbers: Vec<[&str; 6]> = vec![];
	for ch in second_chars {
		second_numbers.push(numbers::get_number(ch));
	}
	let mut layers: Vec<String> = vec![];
	for layer in 0..6 { // 6 is the number of layers
		let mut layer_string: String = String::from("|");
		for n in &hour_numbers {
			layer_string.push_str(n[layer]);
		}
		if layer == 1 || layer == 4{
			layer_string.push_str(" ██ ");
		}
		else {
			layer_string.push_str("    ");
		}
		for n in &minute_numbers {
			layer_string.push_str(n[layer]);
		}
		if layer == 1 || layer == 4{
			layer_string.push_str(" ██ ");
		}
		else {
			layer_string.push_str("    ");
		}
		for n in &second_numbers {
			layer_string.push_str(n[layer]);
		}
		layer_string.push('|');
		layers.push(layer_string);
	}
	let mut roof = String::from("");
	let len = layers[0].chars().count();
	for _ in 0..len {
		roof.push('-');
	}
	println!("{}", roof);
	for layer in &layers {
		println!("{}", layer);
	}
	println!("{}", roof);

}

fn main() {
	//print_nums();
	loop {
		let now = Utc::now();
		print_time(now);
		let mut new_now = Utc::now();
		while now.second() == new_now.second() {
			thread::sleep(Duration::from_millis(10));
			new_now = Utc::now();
		}
		print!("{esc}c", esc = 27 as char); // clear console
	}
}
