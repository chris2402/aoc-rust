use crate::math::{MathTasks, cephalopod::CephalopodNumberParser, human::HumanNumberParser};

#[allow(dead_code)]
mod math;
mod transpose;

fn main() -> Result<(), anyhow::Error> {
    let input = std::fs::read_to_string("packages/aoc_2025_6/input.txt")
        .expect("Failed to read input file");
    let human_solver: MathTasks = input.as_str().try_into()?;

    let result_1 = human_solver.try_solve::<HumanNumberParser>()?;
    println!("Result part 1: {}", result_1);
    let result_2 = human_solver.try_solve::<CephalopodNumberParser>()?;
    println!("Result part 2: {}", result_2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::math::MathTasks;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn it_runs() {
        let _math_tasks: MathTasks = INPUT.try_into().expect("Failed to parse input");
    }
}
