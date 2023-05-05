mod box_tree;
mod rc_tree;

fn main() {
    let mut node1 = box_tree::TreeNode::new(3, None, None);

    /*
     *      3
     *    1  4
     *   2    5
     */
    node1.insert(4);
    node1.insert(1);
    node1.insert(2);
    node1.insert(5);

    println!("======> box tree dfs: ");
    node1.dfs();

    println!("======> box tree bfs: ");
    node1.bfs();

    let mut node2 = rc_tree::TreeNode::new(3, None, None);

     /*
     *      3
     *    1  4
     *   2    5
     */
    node2.insert(4);
    node2.insert(1);
    node2.insert(2);
    node2.insert(5);

    println!("======> rc tree dfs: ");
    node2.dfs();

    println!("======> rc tree bfs: ");
    node2.bfs();
}
