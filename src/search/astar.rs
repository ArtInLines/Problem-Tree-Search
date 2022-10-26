//! # A*
//!
//! ...

pub fn default<Problem, PQ, Prio>(problem: Problem) -> Option<Vec<Problem::Op>>
where
    Problem: ProblemTree,
    problem::Op: Invertable,
    PQ: PrioQueue<Problem, Prio>,
{
    // TODO
}
