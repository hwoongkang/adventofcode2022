# 열셋째 날

트리 형태의 자료 구조를 정의하고,  
중첩 리스트의 형태로 주어지는 입력을 트리 형태로 파싱한 뒤,  
Root 노드 기준으로 트리에 Total ordering(순서?)를 부여해야 하는 문제입니다.

## 입력 파싱

입력 파싱에 제일 많은 시간을 쏟았습니다.

"Leaf node"는 정수입니다: `0`, `1`, `10` 등

"Branch node"는 "Branch node"또는 "Leaf node"를 포함하는 리스트입니다: `[0, 1, 10]`, `[[0], [1]]` 등

입력은 다행히 Leaf node 하나만 주어지는 경우는 없고, 항상 Branch node입니다.

그래서 우선 enum을 트리 형태로 정의했습니다: 다행히 mutability가 필요하진 않아서 Smart pointer는 사용하지 않음.

```Rust
enum Packet {
    Integer(i32), // Leaf node
    List(Vec<Packet>), // Branch node
}
```

입력은 항상 Branch node이므로, Branch node가 들어왔을 때 "가장 바깥의" 대괄호를 삭제한 뒤 `Vec<Packet>`을 파싱해주는 헬퍼 함수를 이용했습니다.

```Rust
impl Packet {
    fn parse(input: &str) -> Self {
        if starts_with_integer { // leaf node
            Self::Integer(input)
        } else { // branch node
            let without_brackets = &input[1..input.len() - 1];
            Self::List(Self::helper(widthout_brackets))
        }
    }
}
```

## Part 1

파트 1까지는 `a <= b` 인지만 검사하면 됩니다.

해당 trait은 `std::cmp::PartialOrd`입니다.

이게 순서 부여 관해서 조금 복잡해서, 빨리 풀고 싶어서 일단은 꼼꼼히 공부는 안했습니다만,

필요 조건은 `PartialEq < Eq, PartialOrd < Ord`로 주어지는 것 같습니다.

### `PartialEq`, `Eq`

`PartialEq`는 `==` 연산자를 사용할 수 있게 해주는 trait입니다. (Copilot 문서 작성도 잘하네요.)

`Eq`는 `PartialEq`만 정의되고 나면 따로 메소드를 더 구현해줄 필요는 없다고 합니다.

궁금해서 찾아보니, `PartialEq`는 `a == b`가 정의되어 있는지만 보장하고,  
`Eq`는 reflexiviy와 transitivity, symmetry등을 보장한다고 합니다.

`PartialEq`만 적용되고 `Eq`가 없는 가장 대표적인 예시는 `NaN != NaN` 이라고 하네요.

- [출처 1](https://users.rust-lang.org/t/what-is-the-difference-between-eq-and-partialeq/15751)
- [출처 2](https://stackoverflow.com/questions/55128808/when-is-it-appropriate-to-require-only-partialeq-and-not-eq)

### `PartialOrd`, `Ord`

이거는 Set theory에서 슬쩍 지나쳤던 기억이 납니다.

Binary Relation이 Partial인지 Total인지 어쩌구..

Rust 공식 Docs(
[PartialOrd](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html),
[Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html))에 각각 링크되어 있는
위키피디아 항목([partial order](https://en.wikipedia.org/wiki/Partially_ordered_set#Partial_order), [total order](https://en.wikipedia.org/wiki/Total_order))을 참고해보니,

Total Order는 집합 내의 임의의 어떤 원소쌍이라도 비교 가능할 때,  
Partial Order는 집합 내 원소쌍 중 비교 불가능한 쌍도 존재하지만, 비교가 가능하다면 reflexive, symmetric, transitive를 만족할 때를 말하는 것 같습니다.

### 아무튼

파트 1에서는 문제에서 주어진 조건에 따라 `PartialOrd`를 구현해서 풀었습니다.

## Part 2

`[[2]]`, `[[6]]`을 주어진 입력에다가 추가한 다음에,  
모든 패킷을 정렬하고,  
`[[2]]`, `[[6]]`의 index를 구해야 합니다.

### 정렬

Rust에서 `Vec<T>`를 정렬하려면 `T: Ord`여야 합니다.  
따라서 Part 1의 `PartialEq + PartialOrd`에 더해 `Eq + Ord`를 구현해 줬습니다.

사실 이거 관련해서 더 작성하려 했는데 Part 1 문서에서 만족스럽게 적혀서..

### index 구하기

그냥 `[[2]]`, `[[6]]`을 넣은 뒤 소팅하고 순회하면서 찾았습니다.

생각해보니 더 멋들어지게 풀려면 binary_search로 풀 수 있겠네요. 바꿔야겠습니다.

### Binary search

처음에는 각각에 대해 이진 탐색을 해본 뒤에 그걸 이용해서 곱했는데요,

생각해보니 decoder를 둘다 집어넣어야 하니까 뒤에꺼엔 이진 탐색 결과값보다 1을 더 더해주어야 합니다.
