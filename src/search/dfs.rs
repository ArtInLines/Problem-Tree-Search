use crate::traits::*;

pub fn default<Problem>(problem: Problem, max_depth: usize) -> Option<Vec<Problem::Op>>
where
    Problem: ProblemTree + Clone,
    Problem::Op: Invertable,
{
    default_helper(problem, max_depth, 0)
}

fn default_helper<Problem>(
    problem: Problem,
    max_depth: usize,
    curret_depth: usize,
) -> Option<Vec<Problem::Op>>
where
    Problem: ProblemTree + Clone,
    Problem::Op: Invertable,
{
    if curret_depth >= max_depth {
        return None;
    }

    for n in problem.next_states() {
        if let Some(path) = default_helper(n.clone(), max_depth, curret_depth + 1) {
            return Some(path);
        }
    }

    return None;
}
