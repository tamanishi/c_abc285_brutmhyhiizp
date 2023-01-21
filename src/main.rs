use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    let mut alphabet = HashMap::new();
    alphabet.insert('A', 1);
    alphabet.insert('B', 2);
    alphabet.insert('C', 3);
    alphabet.insert('D', 4);
    alphabet.insert('E', 5);
    alphabet.insert('F', 6);
    alphabet.insert('G', 7);
    alphabet.insert('H', 8);
    alphabet.insert('I', 9);
    alphabet.insert('J', 10);
    alphabet.insert('K', 11);
    alphabet.insert('L', 12);
    alphabet.insert('M', 13);
    alphabet.insert('N', 14);
    alphabet.insert('O', 15);
    alphabet.insert('P', 16);
    alphabet.insert('Q', 17);
    alphabet.insert('R', 18);
    alphabet.insert('S', 19);
    alphabet.insert('T', 20);
    alphabet.insert('U', 21);
    alphabet.insert('V', 22);
    alphabet.insert('W', 23);
    alphabet.insert('X', 24);
    alphabet.insert('Y', 25);
    alphabet.insert('Z', 26);

    input! {
        mut s: Chars,
    }

    s.reverse();

    let mut num = 0;
    for i in 0..s.len() {
        num += alphabet.get(&s[i]).unwrap() * alphabet.len().pow(i as u32);
    }
    println!("{}", num);
}
