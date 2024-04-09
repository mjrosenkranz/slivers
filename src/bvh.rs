use crate::bounding::{Bounds, AABB};

pub enum BVHNode {
    Node {
        /// index of left child
        l: usize,
        /// index of right child
        r: usize,
    },
    Leaf {
        /// index of shape impl [`Bounds`]
        idx: usize,
    },
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node() {
        let n = Node;
    }
}
