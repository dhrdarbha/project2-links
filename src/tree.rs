#[allow(unused_imports)]
use std::{cmp::Ord, mem};

#[derive(Clone, Debug)]
pub enum TreeNode<T: Ord> {
    Leaf,
    Node(T, Box<TreeNode<T>>, Box<TreeNode<T>>),
}

// Provided functions
impl<T: Ord> TreeNode<T> {
    pub fn height(&self) -> usize {
        match self {
            TreeNode::Leaf => 0,
            TreeNode::Node(_, left, right) => 1 + std::cmp::max(left.height(), right.height()),
        }
    }

    /// Verifies that the tree is a binary search tree
    fn is_bst(&self) -> bool {
        fn is_bst_helper<T: Ord>(tree: &TreeNode<T>, min: Option<&T>, max: Option<&T>) -> bool {
            match tree {
                TreeNode::Leaf => true,
                TreeNode::Node(value, left, right) => {
                    match min {
                        Some(min) => {
                            if value <= min {
                                return false;
                            }
                        }
                        _ => {}
                    }
                    match max {
                        Some(max) => {
                            if value >= max {
                                return false;
                            }
                        }
                        _ => {}
                    }
                    is_bst_helper(left, min, Some(value)) && is_bst_helper(right, Some(value), max)
                }
            }
        }
        is_bst_helper(self, None, None)
    }

    /// Verifies that the tree is balanced
    pub fn is_balanced(&self) -> bool {
        match self {
            TreeNode::Leaf => true,
            TreeNode::Node(_, left, right) => {
                let left_height = left.height();
                let right_height = right.height();
                let diff = (left_height as i32 - right_height as i32).abs();
                diff <= 1 && left.is_balanced() && right.is_balanced()
            }
        }
    }

    /// Verifies that the tree is a valid balanced binary search tree
    pub fn validate(&self) -> bool {
        self.is_bst() && self.is_balanced()
    }
}

// Required functions
impl<T: Ord> TreeNode<T> {
    /// Creates a new TreeNode<T> with value and children left and right
    pub fn node(value: T, left: TreeNode<T>, right: TreeNode<T>) -> TreeNode<T> {
        TreeNode::Node(value, Box::new(left), Box::new(right))
    }

    /// Creates a new `TreeNode<T>` with no children
    pub fn new() -> TreeNode<T> {
        TreeNode::Leaf
    }

    /// Inserts a new node with value into the tree. If the value already exists in the tree
    /// the function does nothing.
    /// After insertion the tree is rebalanced if necessary
    pub fn insert(&mut self, value: T) {
        let mut owned = mem::take(self);
        match &mut owned {
            TreeNode::Leaf => {
                owned = TreeNode::Node(value, Box::new(TreeNode::Leaf), Box::new(TreeNode::Leaf));
            }
            TreeNode::Node(v, l, r) => {
                if value < *v {
                    l.insert(value);
                } else if value > *v {
                    r.insert(value);
                }
                owned.rebalance();
            }
        }
        *self = owned;
    }

    /// Computes the balance factor of the tree
    fn balance_factor(&self) -> i32 {
        match self {
            TreeNode::Leaf => 0,
            TreeNode::Node(_, l, r) => l.height() as i32 - r.height() as i32,
        }
    }

    /// Performs a left rotation on the tree
    pub fn left_rotate(&mut self) {
        let mut n = mem::take(self);
        if let TreeNode::Node(_, _, ref mut right_box) = n {
            let right = mem::take(right_box);
            match *right {
                TreeNode::Leaf => {
                    *self = n;
                }
                TreeNode::Node(rv, rleft, rright) => match n {
                    TreeNode::Node(ref mut v, ref mut left, ref mut right_slot) => {
                        let old_v = mem::replace(v, rv);
                        let old_left = mem::take(left);
                        let new_left = TreeNode::Node(old_v, old_left, rleft);
                        *left = Box::new(new_left);
                        *right_slot = rright;
                        *self = n;
                    }
                    TreeNode::Leaf => unreachable!(),
                },
            }
        } else {
            *self = n;
        }
    }

    /// Performs a right rotation on the tree
    pub fn right_rotate(&mut self) {
        let mut n = mem::take(self);
        if let TreeNode::Node(_, ref mut left_box, _) = n {
            let left = mem::take(left_box);
            match *left {
                TreeNode::Leaf => {
                    *self = n;
                }
                TreeNode::Node(lv, lleft, lright) => match n {
                    TreeNode::Node(ref mut v, ref mut left_slot, ref mut right) => {
                        let old_v = mem::replace(v, lv);
                        let old_right = mem::take(right);
                        let new_right = TreeNode::Node(old_v, lright, old_right);
                        *right = Box::new(new_right);
                        *left_slot = lleft;
                        *self = n;
                    }
                    TreeNode::Leaf => unreachable!(),
                },
            }
        } else {
            *self = n;
        }
    }

    /// Rebalances the tree using either a single or double rotation as specified in the AVL tree
    fn rebalance(&mut self) {
        let bf = self.balance_factor();
        if bf == -2 {
            if let TreeNode::Node(_, _, ref mut right) = self {
                if right.balance_factor() == 1 {
                    right.right_rotate();
                }
            }
            self.left_rotate();
        } else if bf == 2 {
            if let TreeNode::Node(_, ref mut left, _) = self {
                if left.balance_factor() == -1 {
                    left.left_rotate();
                }
            }
            self.right_rotate();
        }
    }
}

// Implement `Default` for `TreeNode<T>`
impl<T: Ord> Default for TreeNode<T> {
    fn default() -> Self {
        TreeNode::Leaf
    }
}

// Implement `PartialEq` for `TreeNode<T>`
impl<T: Ord + PartialEq> PartialEq for TreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TreeNode::Leaf, TreeNode::Leaf) => true,
            (TreeNode::Node(av, al, ar), TreeNode::Node(bv, bl, br)) => {
                av == bv && al == bl && ar == br
            }
            _ => false,
        }
    }
}

// Implement `Eq` for `TreeNode<T>`
impl<T: Ord + Eq> Eq for TreeNode<T> {}

// Implement `From<Vec<T>>` for `TreeNode<T>`
impl<T: Ord> From<Vec<T>> for TreeNode<T> {
    fn from(v: Vec<T>) -> Self {
        let mut root = TreeNode::new();
        for x in v {
            root.insert(x);
        }
        root
    }
}

// Implement `From<TreeNode<T>>` for `Vec<T>`
impl<T: Ord> From<TreeNode<T>> for Vec<T> {
    fn from(tree: TreeNode<T>) -> Self {
        fn traverse<T: Ord>(node: TreeNode<T>, vec: &mut Vec<T>) {
            match node {
                TreeNode::Leaf => {}
                TreeNode::Node(v, l, r) => {
                    traverse(*l, vec);
                    vec.push(v);
                    traverse(*r, vec);
                }
            }
        }
        let mut result = Vec::new();
        traverse(tree, &mut result);
        result
    }
}
