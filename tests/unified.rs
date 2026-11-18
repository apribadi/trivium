#![allow(unused_must_use)]

use std::fmt::Write;
use expect_test::expect;

fn test(o: &mut dyn Write, s: [u8; 20]) {
  let mut g = trivium::Rng::new(s);

  for _ in 0 .. 4 {
    let mut is_first = true;
    for _ in 0 .. 2 {
      for &b in g.next().to_le_bytes().iter() {
        if ! is_first {
          write!(o, " ");
        } else {
          is_first = false;
        }
        write!(o, "{:02X}", b);
      }
    }
    write!(o, "\n");
  }
}

#[test]
fn test_vectors() {
  let mut o = String::new();

  // set 1, vector# 0

  test(
      &mut o,
      [ 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      ]
    );

  expect![[r#"
      38 EB 86 FF 73 0D 7A 9C AF 8D F1 3A 44 20 54 0D
      BB 7B 65 14 64 C8 75 01 55 20 41 C2 49 F2 9A 64
      D2 FB F5 15 61 09 21 EB E0 6C 8F 92 CE CF 7F 80
      98 FF 20 CC CC 6A 62 B9 7B E8 EF 74 54 FC 80 F9
  "#]].assert_eq(o.drain(..).as_str());

  // set 1, vector# 9

  test(
      &mut o,
      [ 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      ]
    );

  expect![[r#"
      61 20 8D 28 6B C1 DC 43 11 71 ED A5 CA F7 9D 95
      60 B1 8A CE F2 64 84 41 7B 65 1A 47 A3 F7 A8 03
      53 F7 9A F8 65 6D A4 30 1A 5E 5A 02 E0 42 65 B1
      82 C6 7F 58 91 22 03 49 F8 CD 1C D0 65 97 B7 7E
  "#]].assert_eq(o.drain(..).as_str());

  // set 2, vector# 0

  test(
      &mut o,
      [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      ]
    );

  expect![[r#"
      FB E0 BF 26 58 59 05 1B 51 7A 2E 4E 23 9F C9 7F
      56 32 03 16 19 07 CF 2D E7 A8 79 0F A1 B2 E9 CD
      F7 52 92 03 02 68 B7 38 2B 4C 1A 75 9A A2 59 9A
      28 55 49 98 6E 74 80 59 03 80 1A 4C B5 A5 D4 F2
  "#]].assert_eq(o.drain(..).as_str());

  // set 2, vector# 9

  test(
      &mut o,
      [ 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      ]
    );

  expect![[r#"
      AB 97 61 6E 7B AF 09 21 F4 24 B2 57 3B FA 15 BD
      CA 01 89 8A BB E6 AB 77 27 9A B1 D7 32 AB D1 05
      12 76 9C C6 9F AB 34 E0 3B 80 7F 92 C9 66 27 C1
      76 56 BC C9 BD 9D 37 72 40 A2 B6 FD BD 78 44 53
  "#]].assert_eq(o.drain(..).as_str());

  // set 5, vector# 0

  test(
      &mut o,
      [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      ]
    );

  expect![[r#"
      F8 90 17 36 64 05 49 E3 BA 7D 42 EA 2D 07 B9 F4
      92 33 C1 8D 77 30 08 BD 75 55 85 B1 A8 CB AB 86
      C1 E9 A9 B9 1F 1A D3 34 83 FD 6E E3 69 6D 65 9C
      93 74 26 04 56 A3 6A AE 11 F0 33 A5 19 CB D5 D7
  "#]].assert_eq(o.drain(..).as_str());


  // set 6, vector# 0

  test(
      &mut o,
      [ 0x00, 0x53, 0xA6, 0xF9, 0x4C, 0x9F, 0xF2, 0x45, 0x98, 0xEB,
        0x0D, 0x74, 0xDB, 0x42, 0xA9, 0x10, 0x77, 0xDE, 0x45, 0xAC,
      ]
    );

  expect![[r#"
      F4 CD 95 4A 71 7F 26 A7 D6 93 08 30 C4 E7 CF 08
      19 F8 0E 03 F2 5F 34 2C 64 AD C6 6A BA 7F 8A 8E
      6E AA 49 F2 36 32 AE 3C D4 1A 7B D2 90 A0 13 2F
      81 C6 D4 04 3B 6E 39 7D 73 88 F3 A0 3B 5F E3 58
  "#]].assert_eq(o.drain(..).as_str());
}
