use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

pub struct TreeNode {
  pub value: u32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  pub fn new(
    value: u32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
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
      Some(left) => left.borrow().dfs(),
      None => {}
    }

    match &self.right {
      Some(right) => right.borrow().dfs(),
      None => {}
    }
  }

  pub fn bfs(&self) {
    println!("tree node value: {}", self.value);

    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

    match &self.left {
      Some(left) => {queue.push_back(Rc::clone(left))},
      None => {}
    }

    match &self.right {
      Some(right) => {queue.push_back(Rc::clone(right))},
      None => {}
    }

    while queue.len() > 0 {
        let node = queue.pop_front();
        match node {
            Some(n) => {
              let inner_node = n.borrow();
                println!("tree node value: {}", inner_node.value);
                match &inner_node.left {
                    Some(left) =>  queue.push_back(Rc::clone(left)),
                    None => {}
                }
                match &inner_node.right {
                  Some(right) =>  queue.push_back(Rc::clone(right)),
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
      Some(t) => {t.borrow_mut().insert(value)},
      None => {
        let new_node = Some(Rc::new(RefCell::new(TreeNode {value, left: None, right: None})));
        *target_node = new_node;
      }
    }
  }
}