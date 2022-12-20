# 스무째 날

파트 2에서 또 컨닝했습니다ㅎ..

https://topaz.github.io/paste/#XQAAAQCKAQAAAAAAAAAyGUj/Tv+WIaytRxiNt+Ae3OYWjW54wnpI1Zi4BeM0xKrmxa6+905+SUxU+lmRUmLKKrcqn3I78z4M/R9cW2WMRVr3kSNG+mMTSWFDi4e+rFq6s7CiY9+F0QcNxJnvdvIO/JItuoNLmfJl1uV5jmdTfRVebkBNzsNQgO75R65VUP3Q5z7fqZ3hB+1LsKszKLIq53E+ILyYtKA8xEDCMHvd3xbHKbxmh/Nv5oJDEv2hqzszL8iAkacr7clRn3WNWtsDGRWzfC+7zRnQKVbsRNk1Pj+3kaYIZXx2uRKiFakqdJX4t2uPJl+nMAj5hSKz/r2NUg==

## 문제 이해

"Circular Queue"인데, 그 안에서 element가 마구 움직이는 문제입니다.

Doubly Linked List로 풀까 했는데요, 역시나 파트 2에서 말도 안 되는 숫자의 iteration 수를 줘 버려서,

그냥 간단한 정수 배열을 통한 operation이 더 빠른 것으로 확인했습니다.

## Part 1

파트 1까지는 일단 doubly linked list로 풀었습니다.

이때부터 이미 5000ms가 넘게 걸리는 문제가 있었는데요,

풀이를 적으려다 보니 컨닝한 솔루션이랑 시간 복잡도가 비슷한 것 같아서 조금 헷갈리네요.

아무래도 `Vec<tuple>`을 돌아다니면서 `m`번 수정하는 연산 속도보다는 `vec.insert(m)`이 같은 시간 복잡도여도 훨씬 빠른가 봅니다.

## Part 2

"숫자"를 빼서 "민다"라는 개념을 transform해서,

"원래 숫자 배열"은 유지하고, "숫자들이 현재 위치한 인덱스"를 바꾼다로 생각하는 것 같은데,

솔직히 잘 이해가 가지 않습니다,,

## 다시 Doubly Linked List로 수정

아무리 생각해도, Circular Queue의 길이가 `L`일 때, 제가 컨닝한 풀이도 `O(L^2)`이고, doubly linked list로 풀었을 때도 `O(L^2)`입니다.

물론 구현하고 보니 사이클 수가 이제 컨닝한 풀이의 20배는 되어 보이긴 하네요.

제가 실수한 부분은, 길이가 `L`일 때 한 바퀴를 다 도는 시점이 `L-1`번을 움직였을 때지 `L`번을 움직였을 때가 아니라는 것이었습니다.

그 부분만 수정해서 modulo 연산과 함께 돌리니까 되네요.
