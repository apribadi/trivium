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
  pub fn new(key: [u8; 10], iv: [u8; 10]) -> Self {
    let x0 = u64::from_le_bytes(key[0 .. 8].try_into().unwrap());
    let x1 = u16::from_le_bytes(key[8 .. 10].try_into().unwrap()) as u64;
    let y0 = u64::from_le_bytes(iv[0 .. 8].try_into().unwrap());
    let y1 = u16::from_le_bytes(iv[8 .. 10].try_into().unwrap()) as u64;
    let z0 = 0;
    let z1 = 0x0000_7000_0000_0000;

    let x = [ x0, x1 ];
    let y = [ y0, y1 ];
    let z = [ z0, z1 ];

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
    self.y = [ a ^ (get::<91>(x) & get::<92>(x)) ^ get::<78>(y), y[0] ];
    self.z = [ b ^ (get::<82>(y) & get::<83>(y)) ^ get::<87>(z), z[0] ];
    a ^ b ^ c
  }
}
