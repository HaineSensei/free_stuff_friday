fn main() {
  println!("{}", (2u64..=29).map(|x| (x-1)*x*(x+1)).sum::<u64>());
}
