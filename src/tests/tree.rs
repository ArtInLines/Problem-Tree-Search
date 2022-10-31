use crate::*;

pub struct Tree {
    parent: Option<Box<Tree>>,
    children: Vec<Box<Tree>>,
    costs: Vec<usize>,
    is_solved: bool,
    path: Vec<usize>,
}

impl Tree {
    // TODO: Finish test
    pub fn balanced(depth: usize, branching_factor: usize) -> Self {
        Self::balanced_helper(depth, branching_factor, None)
    }

    fn balanced_helper(depth: usize, branching_factor: usize, parent: Option<Box<Tree>>) -> Self {
        let mut t = Tree {
            parent,
            children: Vec::new(),
            costs: Vec::new(),
            is_solved: false,
            path: Vec::new(),
        };
        if depth > 0 {
            let children = Vec::with_capacity(branching_factor);
            for i in 0..branching_factor {
                children.push(Box::new(Self::balanced_helper(
                    depth - 1,
                    branching_factor,
                    Some(Box::new(t)),
                )))
            }
            t.children = children;
        }
        t
    }
}

impl Operable for Tree {
    type Op = usize;
    type E = &str;
    type S = (Box<Tree>, usize);

    fn operate_mut(&mut self, op: Self::Op) -> Result<Self::S, Self::E> {
        if op >= self.children.len() {
            Err(&"Invalid Move")
        } else {
            Ok((self.children[op], self.costs[op]))
        }
    }
}

impl ProblemTree for Tree {
    fn is_solved(&self) -> bool {
        self.is_solved
    }

    fn possible_operations(&self) -> Vec<Self::Op> {
        (0..self.children.len()).into_iter().collect()
    }

    fn get_prev_operations(&self) -> Vec<Self::Op> {
        self.path
    }
}
