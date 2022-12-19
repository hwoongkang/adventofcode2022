# 열여섯째 날

Floyd-Warshall, bitmask가 필요하다고 하네요.

너무 어려워서 컨닝했습니다:

https://www.reddit.com/r/adventofcode/comments/zn6k1l/2022_day_16_solutions/

https://github.com/juanplopes/advent-of-code-2022/blob/main/day16.py

## 베끼면서 느낀 점

"그래프"에서의 "백트래킹"이라고 생각하고, 비선형성을 어떻게 감당할지 고민 중이었습니다.

그런데 그래프 상에서 인접하지 않은 경우를 그냥 선형적으로 "풀어내버리는" (`distances`의 생성) 풀이가 인상적이었습니다.
