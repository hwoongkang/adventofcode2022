# 열아홉째 날

스타크래프트 같은 문제입니다.

SCV를 뽑으려면 미네랄이 필요하고, 미네랄을 더 빨리 캐려면 SCV가 더 필요하고, 가스도 캐야 하고...

가장 효율적인 "빌드"를 찾는 문제였습니다.

처음에는 사실 간단한 DP, BFS 등으로 풀 수 있을 줄 알았는데 24턴이 진행되니까 방문 가능한 상태가 너무 많아서 시간이 너무 오래 걸렸습니다.

결국 또 레딧의 도움을..  
https://www.reddit.com/r/adventofcode/comments/zpihwi/2022_day_19_solutions/

다들 여러 종류의 "Heuristics"를 도입해서 가능한 상태를 줄이려고 하고 있었습니다:

- Ore robot은 4개까지만 만든다.
- Geode robot을 만들 수 있으면 무조건 만든다.

등등..

그런데 문제는 저는 도저히 휴리스틱이 안 찾아지더라구요.

그러던 중에 [이 사람](https://topaz.github.io/paste/#XQAAAQCJBAAAAAAAAAA0m0pnuFI8c/T1npou3Ns2y6cWc/1crsV/u1SklIvndiVj4eISQO2llrtG+JGq5k4Utra6CYic43lLClGeYyctQBxsg++z1PPnUrrtRyQE5sSLwkFX93jh6wOlhiW5TPmlcciR9mmox+BiZPZuR2qeeZc1MHUZnuBdxiu9rzBWLzBz8FJqd0En4e17d+mP80FNrUYpt/M8GhUkGc28X+uilKC8gCv6DZYGesDx+XgniCV/lzuJNXkh6SyrJw5uc0cRt8Rb5rqbEvpjf3KxbRO36sxVvL2o7ueFdutAGQT0uSfBLJnAPmyLg+pBdGnZAe7xp6BqyO7AaUh5GIkPc7YBgAZL1ahuGz3gOQo4Mo9TfBu6bPY2pnah0iD9xyySICYYhZquTwV+pqZiNp+sYGnh595M/IOKVTn4vPrgASUbUypr4AyoJLZfTaw3I7cP+8Wl6BuO1aPRC5AXZKk9uE0kJ/rcoWhZLVfBl9clPIMiboOersPOzohPOkTLG4AHIPyord5TdcXvvoANKomhwJTQzeYxflYO8IEkSrSTQaBrQZ4EFTTcT0QKE1vrnrtiE4rQ9cVvcrrRc3k4ypII/ky7hARPjRZ1cdmqI9iUZQ4hJZuFdArCVnpt5IAIHCDUlYhc7HSWF675NkIB8UG6Eusebr48bY6CCmVDB/Ye3QkDJl9eT1lvBbu8qkZGnDFSASxf+HEssCQ5bQsvCVSOo7XjIn3e6q3bUNv8exGiPNZ2t7umXR2yIveJpYn/2a48mQ==)풀이가 정말 맘에 들었습니다.

어쨌거나 로봇이나 자원이 쌓이는 게 중요하니까, "후반 테크"일수록 중요도를 더 많이 마킹해서, 중요도 높은 순서로 5000개 정도만 남겨가며 보겠다는 아이디어였습니다.

다른 레디터들은 뭐 이딴 식으로 푸냐고 화내는 사람하고 천재 아니냐는 사람 반으로 나뉘던데,  
저는 솔직히 굉장한 아이디어였다고 생각합니다.

어차피 다른 것들도 다 휴리스틱이었으니..

아무튼 이번 문제도 '컨닝'(파이썬->러스트 포팅만 진행)으로 풀어버렸네요.
