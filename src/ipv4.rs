#![feature(libc)]
extern crate libc as c;
use std::cmp;

trait NetInt {
    fn from_be(i: Self) -> Self;
    fn to_be(&self) -> Self;
}
macro_rules! doit {
    ($($t:ident)*) => ($(impl NetInt for $t {
        fn from_be(i: Self) -> Self { <$t>::from_be(i) }
        fn to_be(&self) -> Self { <$t>::to_be(*self) }
    })*)
}
doit! { i8 i16 i32 i64 isize u8 u16 u32 u64 usize }

fn hton<I: NetInt>(i: I) -> I { i.to_be() }
fn ntoh<I: NetInt>(i: I) -> I { I::from_be(i) }

#[derive(Copy)]
pub struct Ipv4Addr2 {
    inner: c::in_addr,
}

impl Clone for Ipv4Addr2 {
    fn clone(&self) -> Ipv4Addr2 { *self }
}

impl Ipv4Addr2 {
    /// Creates a new IPv4 address from four eight-bit octets.
    ///
    /// The result will represent the IP address `a`.`b`.`c`.`d`.
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr2 {
        Ipv4Addr2 {
            inner: c::in_addr {
                s_addr: hton(((a as u32) << 24) |
                             ((b as u32) << 16) |
                             ((c as u32) <<  8) |
                              (d as u32)),
            }
        }
    }
}

impl PartialEq for Ipv4Addr2 {
	fn eq(&self, other: &Self) -> bool {
		self.cmp(other) == cmp::Ordering::Equal
	}
}

impl Eq for Ipv4Addr2 {}

impl PartialOrd for Ipv4Addr2 {
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Ipv4Addr2 {
	fn cmp(&self, other: &Self) -> cmp::Ordering {
		return Ord::cmp(&ntoh(self.inner.s_addr), &ntoh(other.inner.s_addr));
	}
}
