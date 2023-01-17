pub struct Trivium64 {
  x: [u64; 2],
  y: [u64; 2],
  z: [u64; 2],
}

#[inline(always)]
fn get<const N: usize>(x: [u64; 2]) -> u64 {
  (x[0] >> (N - 64)) | (x[1] << (128 - N))
}

impl Trivium64 {
  pub fn initialize(key: [u8; 10], iv: [u8; 10]) -> Self {
    let mut x = [ 0, 0 ];
    let mut y = [ 0, 0 ];
    let     z = [ 0, 0x0000_7000_0000_0000 ];

    for i in 0 .. 8 { x[0] = x[0] | ((key[i + 0] as u64) << i); }
    for i in 0 .. 2 { x[1] = x[1] | ((key[i + 8] as u64) << i); }
    for i in 0 .. 8 { y[0] = y[0] | (( iv[i + 0] as u64) << i); }
    for i in 0 .. 2 { y[1] = y[1] | (( iv[i + 8] as u64) << i); }

    let mut t = Self { x, y, z };
    for _ in 0 .. 18 { let _: u64 = t.next(); }
    t
  }

  pub fn next(&mut self) -> u64 {
    let x = self.x;
    let y = self.y;
    let z = self.z;
    let a = get::<66>(x) ^ get::<93>(x);
    let b = get::<69>(y) ^ get::<84>(y);
    let c = get::<66>(z) ^ get::<111>(z);
    self.x = [ c ^ (get::<109>(z) & get::<110>(z)) ^ get::<69>(x), x[0] ];
    self.y = [ a ^ (get::< 91>(x) & get::< 92>(x)) ^ get::<78>(y), y[0] ];
    self.z = [ b ^ (get::< 82>(y) & get::< 83>(y)) ^ get::<87>(z), z[0] ];
    a ^ b ^ c
  }
}
