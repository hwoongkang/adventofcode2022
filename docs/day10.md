# 열째 날

CRT 모니터와 관련된 부분입니다.  
영어가 모국어가 아닌 사람에게는,  
클록과 레지스터 관련해서 "during"의 값이 필요한 건지 "after"의 값이 필요한 건지,
그래서 레지스터 값이 어떻게 변동된 걸 보고해야하는지 굉장히 헷갈렸습니다.

다행히 테스트 케이스를 통해 이겨냈고, 역시 테스트할 수 있는 구조를 짜 놓길 잘했다는 생각을 했습니다.

## Part 1

이번 문제는 Part 1과 Part 2를 그냥 나눠서 짰습니다.  
Part 1에서는 `Addx val`을 수행할 때 두 사이클이 걸리는 거를 내부에서만 처리해도 됐는데,  
Part 2에서는 클록 사이클마다 수행해줘야할 일이 더 많더라구요.

보고해야 할 클록 사이클이 되었는지 (20, 60, 100) `Addx val` 내에서 검사하다가, 레지스터 값을 변경하지 않고 early return을 해버리는 실수를 한 번 해서 디버깅에서 고생을 좀 했습니다.

## Part 2

CRT입니다. (옛날식 정전기 나는 모니터)  
이번에도 영어가 어려워서 Sprite는 무슨 뜻이며, 그래서 언제 어떻게 그려야 한다는 건지 이해하느라 시간을 좀 썼습니다.  
게다가 y축 값과 상관없이 무조건 x만 챙겨야 하는거라,  
`clock_cycle`이 아니라 `clock_cycle % 40`을 써야 한다는 사실을 뒤늦게 깨달아서 그 버그를 찾는데 오래 걸렸습니다.

처음에는 그냥 stdout에 그리는 위주로 짜느라 테스트를 못 했구요,  
이후에 최종 디피 스테이트를 우선 `Vec<char>`로 받고, 그걸 다시 `pretty_print`해 주는 식으로 바꿔서 테스트를 추가했습니다.

스포 주의입니다만 저는 advent of code에서 이런 식으로 출력이 나올 때 제일 재밌는 것 같아요.

```
####.###...##..###..#....####.####.#..#.
...#.#..#.#..#.#..#.#....#.......#.#..#.
..#..#..#.#..#.#..#.#....###....#..#..#.
.#...###..####.###..#....#.....#...#..#.
#....#.#..#..#.#.#..#....#....#....#..#.
####.#..#.#..#.#..#.####.#....####..##..
```
