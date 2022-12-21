# 스물한째 날

오늘도 풀만했습니다!

간단한 트리 문제입니다.

## 문제 이해

어떻게 보면 위상 정렬스럽기도 한, 이진 트리 문제 입니다.

Leaf node는 value를 가지고 있고,

Branch node는 자식 노드를 통해서 어떻게 자신의 value를 도출해내야 하는지에 대한 정보를 가지고 있습니다.

따라서 Root node의 value를 구하려면, postorder traversal로 한 번 순회하면 됩니다.

## Part 1

`Option<Rc<RefCell<TreeNode>>>` 이용해서 Tree를 구현하면 됩니다.

이번 Advent of Code 덕분에 Rust에서의 트리 사용은 익숙해졌네요.

## Part 2

조금은 문제가 복잡해졌습니다.

Part 1에서는 node의 id는 별로 상관없는 정보였어서 그냥 한 번만 순회하면 되었었는데,

이제는 `humn` node가 중요해집니다.

그래서 `struct Monkey`에 `id: String` 필드를 추가해주었습니다.

그리고 이제 `humn` node가 어디 있는지를 알아야 하는데요, 귀찮아서 그냥 traversal 하는 동안 만났는지 아닌지에 따라 `Option`을 반환하게 처리했습니다.

다행히 root node에 대한 정보를 정확히 가지고 있으니까, 양쪽에 대해 순회를 해서 `humn` 유무와 상관없이 값을 구하고,

왼쪽 오른쪽 각 subtree에 대해,

- `humn` 노드를 애초에 가지고 있는지 (없다면 `None`이 나옴)
- 반대쪽 tree와 같은 값을 내기 위해서는 `humn` 노드의 value를 어떻게 바꿔주어야 하는지 (`Option<i64>`)

를 한 번의 순회로 처리했습니다.

## Refactoring

Part 2를 풀 때 구구절절하게 왼쪽 subtree, 오른쪽 subtree 따로 처리해주는 게 별로라서,

`Op::Eq`를 추가해서 해결했습니다.
