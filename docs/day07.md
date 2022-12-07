# 일곱째 날

Tree를 Rust로 구현하느라 굉장히 고생했습니다.. 두괄식으로:

```Rust
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

struct TreeNode {
    value: i32,
    parent: Option<Weak<RefCell<TreeNode>>>,
    children: HashMap<String, Rc<RefCell<TreeNode>>>
}
```

이런 식으로 구현했습니다.  
`parent`도 처음에 `Weak`이 아니라 `Rc`로 구현했다가, Memory cycle 나서 스택 오버플로우 터지고..

트리만 잘 구현하고 나면, 그냥 평범한 Tree Traversal만 하면 됩니다.  
(트리 구현에 너무 힘을 많이 써서 오늘 문서는 여기까지)
