// DataStructures
//
// Node:
//  - parent: Option<&Node>
//  - name: String
//  - size: u32
//
// Algorithm
// https://www.geeksforgeeks.org/iterative-postorder-traversal-of-n-ary-tree/
//
//
// set current working directory
// iterate over lines
// if line is command:
//      parse command
//      if command is move
//          if move is home
//              set current working directory to home directory
//          if move is up
//              set current working directory to parent of current working directory
//          if move is down
    //          check directory has been seen
    //          if directory has been seen
    //              reset current working directory to new directory
    //              continue
    //          else
    //              initialize new directory
    //              set new directory parent to current working directory
    //              reset current working directory to new directory
//      if command is list
//         noop
// else
//      if line is dir
//          noop
//      else
//          if file has been seen
//              noop
//          else
//              initialize new file
//              set file parent to current working directory
//              add file to seen
//
//
//
//
//
//



pub fn solve_part1(input: &str) {
    println!("Results placeholder");
}

pub fn solve_part2(input: &str) {
    println!("Results placeholder");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_solve_part1() {
        // let input = "";
        // let result = solve_part1(input);
        let result = 2;
        assert_eq!(result, 2);
    }

    #[test]
    fn test_solve_part2() {
        // let input = "";
        // let result = solve_part2(input);
        let result = 2;
        assert_eq!(result, 2);
    }
}
