# 아홉째 날

플랑크 길이가 어쩌구하질 않나, 밧줄이라고 해놓고서는 되게 짧은 snake game스럽지 않나, 조금 직관에서 어긋나서 Part 1 풀 때까지만 해도 좀 아리송했습니다.

## Part 1

밧줄이 딱 두 개의 매듭으로만 이루어져 있습니다.

다행히 "head"는 동서남북으로만 움직일 수 있고, "tail"은 "head"와 최대한 붙어있도록, head가 1칸 움직일 때마다 대각선 방향을 포함하여 8개 방향 중 하나로 움직입니다.

간단한 시뮬레이션 문제여서 그렇게 풀었습니다.

- 이때, 어찌보면 "무한 평면", 즉 정해진 사이즈의 보드가 아니라 밧줄은 어디로든 움직일 수 있기에, "tail"이 방문했던 좌표를 셀 때 2차원 벡터를 사용하지 않고 HashSet을 사용했습니다.

- `i32::signum(&self)`라는 굉장히 좋은 유틸 함수가 있더라구요.

```Rust
let mut sample = 0;
let mut x = 134;

if x > 0 {
    sample += 1;
} else if x < 0 {
    sample -= 1;
}
```

이 코드를 ->

```Rust
sample += x.signum();
```

으로 바꿔쓸 수 있습니다. Copilot이 알려줌..

## Part 2

드디어 "밧줄"스럽게 바뀌었습니다.  
파트 2의 밧줄은 10개의 매듭으로 이루어져 있습니다.

그런데 딱히 로직이 변하는 건 없기에, Part 1에서 쓴 코드를 조금만 응용하면 `struct Rope`를 만들 수 있습니다.

이때

```Rust
    fn follow(&mut self) {
        let l = self.knots.len();
        for i in 1..l {
            self.knots[i].follow(&self.knots[i-1]);
        }
    }
```

으로 한 줄로 쓰고 싶었는데, `follow`가 mutable borrow를 해야 해서 한 줄에 못 씁니다.  
그래서 조금 장황하게

```Rust
let prev = &self.knots[i-1];
let curr = &mut self.knots[i];
curr.follow(prev);
```

라고 해야합니다.

이정도는 compiler 단에서 이해해줬으면 하는 바램이 있네요..

## 정리

Part 2까지 풀고나니, Part 1 솔루션이 조금 장황하게 느껴져서,  
`struct Rope`를 길이를 이용해 초기화할 수 있게 만들고,  
Part 2는

```Rust
let mut rope = Rope::new(10);
rope.foobar();
```

로,

Part 1은 `Rope::new(2);`로 풀 수 있도록 추상화했습니다.
