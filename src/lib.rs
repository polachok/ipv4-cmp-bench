#![feature(test,libc)]
extern crate rand;
extern crate test;
use self::test::Bencher;
use std::net::Ipv4Addr;
use std::cmp;

mod ipv4;
use ipv4::Ipv4Addr2;

#[bench]
fn do_compare(bench: &mut Bencher) {
	use rand::Rng;
	let mut rng = rand::thread_rng();
	let mut cc: usize = 0;
	let (a, b, c, d) = rng.gen();
	let (e, f, g, h) = rng.gen();
	bench.iter(|| {
		let ip = Ipv4Addr::new(a, b, c, d);
		let ip2 = Ipv4Addr::new(e, f, g, h);
		if Ord::cmp(&ip, &ip2) == cmp::Ordering::Less {
			cc += 1;
		}
	});
	//println!("c: {}", cc);
}

#[bench]
fn do_compare_2(bench: &mut Bencher) {
	use rand::Rng;
	let mut rng = rand::thread_rng();
	let mut cc: usize = 0;
	let (a, b, c, d) = rng.gen();
	let (e, f, g, h) = rng.gen();
	bench.iter(|| {
		let ip = Ipv4Addr2::new(a, b, c, d);
		let ip2 = Ipv4Addr2::new(e, f, g, h);
		if Ord::cmp(&ip, &ip2) == cmp::Ordering::Less {
			cc += 1;
		}
	});
	//println!("c: {}", cc);
}

#[test]
fn compare_results() {
	use rand::Rng;
	let mut rng = rand::thread_rng();
	for i in 0..429496729 {
		let (a, b, c, d) = rng.gen();
		let (e, f, g, h) = rng.gen();
		let a1 = Ipv4Addr2::new(a, b, c, d);
		let a2 = Ipv4Addr2::new(e, f, g, h);
		let b1 = Ipv4Addr::new(a, b, c, d);
		let b2 = Ipv4Addr::new(e, f, g, h);
		assert!((a1 < a2) == (b1 < b2));
	}
}
