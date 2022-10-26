//! This module provides access to all traits used in this crate.

pub trait Queue<Item> {
    fn new() -> Self;

    fn push(&mut self, item: Item);

    fn peek(&self) -> Option<&Item>;

    fn peek_mut(&mut self) -> Option<&mut Item>;

    fn pop(&mut self) -> Option<Item>;

    fn is_empty(&self) -> bool;

    fn size(&self) -> usize;
}

pub trait PrioQueue<Item, Prio>
where
    Prio: PartialOrd,
{
    fn new() -> Self;

    fn push(&mut self, item: Item, prio: Prio);

    fn peek(&self) -> Option<(&Item, &Prio)>;

    fn peek_mut(&self) -> Option<(&mut Item, &mut Prio)>;

    fn pop(&mut self) -> Option<(Item, Prio)>;

    fn is_empty(&self) -> bool;

    fn size(&self) -> usize;
}

pub trait Heuristic {
    fn h(&self) -> usize;
}

pub trait Norm<RHS = Self> {
    type Result;
    fn d(&self, other: &RHS) -> Self::Result;
}

/// An interface for Lookup Tables
///
/// This interface is used by all search algorithms that use tables.
/// Specifically non-heuristic algorithms can be optimized by utilizing their variants that use this interface.
/// It is mainly useful to avoid looking at certain problems twice.
/// The interface doesn't restrict the implementation of storing the current state of the problem.
/// It is thus advised to use any compression-algorithms that decrease the amount of memory used.
///
/// The Type-Parameter `T` is a generic, to allow using the same Lookup Table for different types.
/// Usually, `T` will refer to instances of the Problem, which implements the `ProblemTree` trait.
/// That's not a requirement, however.
pub trait Lookup<T> {
    /// Store an instance `el` of `T` in the lookup table.
    ///
    /// Storing shouldn't change the value in `el`, thus only an immutable reference to it is given.
    fn store(&mut self, el: &T);

    /// Check whether the instance `el` of `T` is stored in the lookup table.
    fn contains(&self, el: &T) -> bool;

    /// This function should return the amount of elements currently stored in the lookup table.
    fn size(&self) -> usize;
}

/// An interface for invertable types.
/// A type is invertable if, and only if, every instance of the type has an inverse of the same type.
///
/// If this interface is implemented on the Problem's operation, then
/// "going back" to the previous problem-state is always possible.
/// This is useful to avoid high memory usage and only store the paths
/// of operations to get to certain problem-states, rather than the
/// states themselves.
pub trait Invertable {
    /// The `invert` method should return the inverse of `&self`.
    ///
    /// # Examples:
    ///
    /// ```
    /// use problem_tree_search::Invertable;
    ///
    /// #[derive(Debug, PartialEq)]
    /// enum Direction {
    /// 	North,
    /// 	West,
    /// 	South,
    /// 	East,
    /// }
    ///
    /// impl Invertable for Direction {
    /// 	fn invert(&self) -> Self {
    /// 		match self {
    /// 			Self::North => Self::South,
    /// 			Self::West => Self::East,
    /// 			Self::South => Self::North,
    /// 			Self::East => Self::West,
    /// 		}
    /// 	}
    /// }
    ///
    /// assert_eq!(Direction::South, Direction::North.invert());
    /// ```
    fn invert(&self) -> Self;
}

/// An Interface for allowing an operation on the type.
/// This trait is necessary to implement the `ProblemTree` trait.
pub trait Operable {
    /// The type of the operation on the Problem Space.
    /// To use the `Invert` trait, it needs to be implemented on the `Op` type.
    ///
    /// All search algorithms will (on success) return a list of operations of this type.
    /// This sequence of operators specifies the path of the initial state to the goal state.
    type Op;
    /// The type returned on a successful mutable operation on the problem.
    type S;
    /// The type returned, when an Error occurs when operating on the problem.
    type E;

    /// Operate on the Problem (to reach the next state) without mutating the problem state.
    ///
    /// # Default Implementation
    ///
    /// The default implementation clones the problem state and then calls the mutatable operation.
    /// Thus, only the mutable implementation will be necessary to implement in most cases.
    ///
    /// This default implementation does require `Self` to implement the `Clone` trait, however.
    fn operate(&self, op: Self::Op) -> Result<Self, Self::E>
    where
        Self: Clone,
    {
        let mut c = self.clone();
        match c.operate_mut(op) {
            Ok(_) => Ok(c),
            Err(e) => Err(e),
        }
    }

    /// The main method used to operate on a problem state.
    /// It should mutate the problem in such a way to lead to the next state based on the specific operation.
    fn operate_mut(&mut self, op: Self::Op) -> Result<Self::S, Self::E>;
}

/// The main trait needed for solving the specific problem: The `ProblemTree` trait.
///
/// The type implementing this trait specifies the interface for the search-algorithms,
/// which aim to find a path from the initial problem-state to the goal-state.
pub trait ProblemTree
where
    Self: Operable,
{
    /// This method should return a list of all possible operations.
    /// The length of the returned list is also known as the branching factor.
    /// The reason for that is that each operation transforms the problem to a new state
    /// which would be represented by a child node in the tree.
    /// Thus there are (at most) as many children for any problem-state
    /// as there are possible operations on that state.
    ///
    /// The simplest implementation would simply return a vector of all possible operations.
    /// Even if the operation would be invalid on the current problem-state,
    /// that wouldn't be an issue, due to the default implementation of the `new_states()` method.
    fn possible_operations(&self) -> Vec<Self::Op>;

    /// ...
    fn next_states(&self) -> Vec<Self>
    where
        Self: Clone,
    {
        let ops = self.possible_operations();
        let mut res = Vec::with_capacity(ops.len());
        for op in ops {
            if let Ok(v) = self.operate(op) {
                res.push(v);
            }
        }
        res
    }

    ///...
    fn get_prev_operations(&self) -> Vec<Self::Op>;

    /// ...
    fn is_solved(&self) -> bool;
}
