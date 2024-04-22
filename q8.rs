struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
  }
  
  impl TreeNode {
    #[allow(unused)]
    fn new(val: i32) -> Self {
      TreeNode {
        val,
        left: None,
        right: None,
      }
    }
  }
  
  fn max_depth(root: Option<&TreeNode>) -> i32 {
    if root.is_none() {
      return 0;
    }
  
    let left_depth = max_depth(root.unwrap().left.as_ref());
    let right_depth = max_depth(root.unwrap().right.as_ref());
  
    std::cmp::max(left_depth, right_depth) + 1
  }
  