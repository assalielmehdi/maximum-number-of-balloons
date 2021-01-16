mod solution_seq;
mod solution_par;
mod custom_math;

fn main() {
  let mut text = String::new();
  std::io::stdin().read_line(&mut text).unwrap();

  fast_tracer::svg("maximum-number-of-balloons-parallel.svg", || {
    let levels = custom_math::Math::log10(text.len() as u32);
    let answer = solution_par::Solution::solve(&text, levels);
    println!("{}", answer);
  })
  .expect("Failed saving svg file");
}
