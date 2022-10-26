//! # Breadth-First-Search (BFS)
//!
//!

use crate::traits::*;

/// ...
pub fn without_table<Problem, Q>(problem: Problem) -> Option<Vec<Problem::Op>>
where
    Problem: ProblemTree + Clone,
    Problem::Op: Invertable,
    Q: Queue<Problem>,
{
    let mut queue = Q::new();
    queue.push(problem);

    while !queue.is_empty() {
        let node = match queue.pop() {
            Some(node) => node,
            None => break,
        };
        for n in node.next_states() {
            if n.is_solved() {
                return Some(n.get_prev_operations());
            } else {
                queue.push(n);
            }
        }
        print!(
            "\r |Queue|: {}   -   Depth: {}",
            queue.size(),
            node.get_prev_operations().len()
        );
    }

    None
}

/// ...
pub fn with_table<Problem, Table, Q>(
    problem: &Problem,
    table: &mut Table,
) -> Option<Vec<Problem::Op>>
where
    Problem: ProblemTree + Clone,
    Problem::Op: Invertable,
    Table: Lookup<Problem>,
    Q: Queue<Problem>,
{
    let mut queue = Q::new();
    queue.push(problem.clone());
    table.store(&problem.clone());

    while !queue.is_empty() {
        let node = match queue.pop() {
            Some(node) => node,
            None => break,
        };
        for n in node.next_states() {
            if n.is_solved() {
                return Some(n.get_prev_operations());
            } else if !table.contains(&n) {
                queue.push(n.clone());
                table.store(&n);
            }
        }
        print!(
            "\r|Queue|: {}   -   |Table|: {}   -   Depth: {}",
            queue.size(),
            table.size(),
            node.get_prev_operations().len()
        );
    }

    None
}
