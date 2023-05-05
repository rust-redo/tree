use std::collections::VecDeque;

pub struct TreeNode {
    pub value: u32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>
}

impl TreeNode {
    pub fn new(
        value: u32,
        left: Option<Box<TreeNode>>,
        right: Option<Box<TreeNode>>
    ) -> TreeNode {
        TreeNode {
            value,
            left,
            right,
        }
    }

    pub fn dfs(&self) {
        println!("tree node value: {}", self.value);

        match &self.left {
            Some(left) => left.dfs(),
            None => {}
        }

        match &self.right {
            Some(right) => right.dfs(),
            None => {}
        }
    }

    pub fn bfs(&self) {
        let mut queue = VecDeque::from([self]);

        while queue.len() > 0 {
            let node = queue.pop_front();
            match node {
                Some(n) => {
                    println!("tree node value: {}", n.value);
                    match n.left {
                        Some(ref left) =>  queue.push_back(left),
                        None => {}
                    }
                    match n.right {
                        Some(ref right) =>  queue.push_back(right),
                        None => {}
                    }
                },
                None => {}
            }
            
        }
    }

    pub fn insert(&mut self, value: u32) {
        let target_node = if value < self.value {&mut self.left} else {&mut self.right};
        match target_node {
            &mut Some(ref mut t) => {t.insert(value)},
            &mut None => {
                let new_node = Some(Box::new(TreeNode {value, left: None, right: None}));
                *target_node = new_node;
            }
        }
    }
}