// TRIVIUM ENDIAN CONVENTIONS
//
// - big endian bit order!
// - little endian byte order!
//
// The Trivium specification gives *lower* indexes to *newer* bits!
//
// In our output `u64`, we place the oldest bit at the least significant (i.e.
// highest index) position, and the newest bit at the most significant (i.e.
// lowest index) position.
//
// For our key (and IV), we assemble 10 octets into a 80 bit word with a little
// endian convention, and then read off the bits from the 80 bit word with a big
// endian convention!
//
// It would be more convenient if the bit indexing were reversed.
//
// Anyway, with this particular set of conventions we match the test vectors
// (which are specified octect by octet) and don't need to do bit reversals in
// our implementation.

#[derive(Clone)]
pub struct Rng {
  x: (u64, u64),
  y: (u64, u64),
  z: (u64, u64),
}

#[inline(always)]
fn get(x: (u64, u64), n: usize) -> u64 {
  (x.0 >> (128 - n)) ^ (x.1 << (n - 64))
}

impl Rng {
  pub fn new(seed: [u8; 20]) -> Self {
    // For the purposes of a RNG, we take a key and IV concatenated into a
    // single seed.
    let mut a = [0u8; 8]; a[6 ..].copy_from_slice(&seed[0 .. 2]);
    let mut b = [0u8; 8]; b.copy_from_slice(&seed[2 .. 10]);
    let mut c = [0u8; 8]; c[6 ..].copy_from_slice(&seed[10 .. 12]);
    let mut d = [0u8; 8]; d.copy_from_slice(&seed[12 .. 20]);
    let x = (u64::from_le_bytes(a), u64::from_le_bytes(b));
    let y = (u64::from_le_bytes(c), u64::from_le_bytes(d));
    let z = (0x0000_0000_000e_0000, 0x0000_0000_0000_0000);
    let mut t = Self { x, y, z };
    for _ in 0 .. 18 { let _: u64 = t.next(); }
    t
  }

  pub fn next(&mut self) -> u64 {
    let x = self.x;
    let y = self.y;
    let z = self.z;
    let a = get(x, 66) ^ get(x, 93);
    let b = get(y, 69) ^ get(y, 84);
    let c = get(z, 66) ^ get(z, 111);
    self.x = (x.1, c ^ (get(z, 109) & get(z, 110)) ^ get(x, 69));
    self.y = (y.1, a ^ (get(x, 91) & get(x, 92)) ^ get(y, 78));
    self.z = (z.1, b ^ (get(y, 82) & get(y, 83)) ^ get(z, 87));
    a ^ b ^ c
  }
}
