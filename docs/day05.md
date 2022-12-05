# 다섯째 날

문제 자체는 그냥 스택을 가지고 노는, 간단한 시뮬레이션이었습니다.

## 세팅

1. 초기값 파싱

그런데 처음부터 파싱하긴 조금 힘든 종류의 입력값이 나와 버렸습니다:

```
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
```

화물선착장(`struct Stacks`로 정의함)의 초기값이 첫 세 줄에 주어져 있는데요,  
이걸 기계적으로 파싱하려면 굉장히 피곤할 거 같아서 그냥 `Stacks::new`는 다음과 같이 수작업했습니다:  
다행히 실제 입력값에서도 스택은 9개 정도밖에 없더라구요.

```
let mut stacks = Stacks::new(&["ZN", "MCD", "P"]);
```

2. 명령 파싱

대신, `move 1 from 2 to 1`을 해석할 때부터를 자동화했습니다.

튜플로 넘기는 건 뭔가 간지나지 않아서, `struct Command`를 정의하였고,  
입력의 패턴도 알고 있으니까 또 `std::str::FromStr`도 구현해주었습니다.

## Part 1

아직 "기중기"의 성능이 좋지 않아서, "2번째 스택에서 1번째 스택으로 3개를 옮겨라"라고 해도,  
한 번에 하나씩 세번 밖에 못 옮깁니다.

따락서 다음과 같은 코드면 됩니다!

```Rust
let mut stacks = Stacks::new(&["", "", ""]);
for line in input.lines() {
    let command: Command = line.parse().unwrap(); //std::str::FromStr

    // 이하를 실제로는 Stacks::execute로 구현했음
    for _ in 0..command.num_crates_to_move {
        let from = &mut stacks[command.from_stack];
        let crate_to_move = from.pop().unwrap();
        let to = &mut stacks[command.to_stack];
        to.push(crate_to_move);
    }
}
```

## Part 2

"기중기"가 업그레이드 되어서, 한 번에 여러 개를 옮길 수 있습니다.  
이걸 임시 벡터를 두어서 구현할까도 생각했는데, 역시나 멋이 충분하지 않았습니다:

```Rust
// Snip
let from = &mut stacks[command.from_stack];
let temp = vec![];
for _ in 0..command.num_crates_to_move {
    temp.push(from.pop().unwrap());
}
let to = &mut stacks[command.to_stack];
while Some(cr) = temp.pop() {
    to.push(cr);
}
```

그래서 Rust의 `Vec` 문서를 읽어보니 [`split_off`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_off)
라는 지금 쓰기 딱 좋은 메소드가 있더라구요.

```Rust
let mut vec = vec![1, 2, 3];
let vec2 = vec.split_off(1);
assert_eq!(vec, [1]);
assert_eq!(vec2, [2, 3]);
```

그리고 `Vec` 뒤에 다른 `Vec`은 `append`로 붙일 수 있어서 그걸 활용했습니다.

```Rust
let from = &mut stacks[command.from_stack];
let l = from.len();
let mut temp = from.split_off(l - command.num_crates_to_move);
let to = &mut stacks[command.to_stack];
to.append(&mut temp);
```
