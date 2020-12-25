const CARD_PUBLIC_KEY: u64 = 18356117;
const DOOR_PUBLIC_KEY: u64 = 5909654;

fn get_loop_size(public_key: u64) -> u64 {
    let (mut val, mut loop_size) = (1u64, 0u64);
    loop {
        val *= 7;
        val %= 20201227;
        loop_size += 1;
        if val == public_key {
            break loop_size;
        }
    }
}

fn get_encyption_key(public_key : u64, loop_size : u64) -> u64 {
  let mut encryption_key = 1u64;
  for _ in 0..loop_size
  {
    encryption_key *= public_key;
    encryption_key %= 20201227;
  }
  encryption_key
}

fn main() {

  let card_loop_size = get_loop_size(CARD_PUBLIC_KEY as u64);
  println!("Part1 : {}", get_encyption_key(DOOR_PUBLIC_KEY, card_loop_size));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loop_size() {
        assert_eq!(get_loop_size(5764801), 8);
        assert_eq!(get_loop_size(17807724), 11);
    }
}
