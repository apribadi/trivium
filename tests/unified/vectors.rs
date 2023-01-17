use crate::prelude::*;

#[test]
fn test_vectors() {
  let mut t =
    Trivium64::initialize(
      [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ],
      [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ]
    );

  expect!["0xdf07fd641a9aa0d8"].assert_eq(&format!("{:#018x}", t.next()));
  expect!["0x8a5e7472c4f993fe"].assert_eq(&format!("{:#018x}", t.next()));
  expect!["0x6a4cc06898e0f3b4"].assert_eq(&format!("{:#018x}", t.next()));
  expect!["0xe7159ef0854d97b3"].assert_eq(&format!("{:#018x}", t.next()));
}
