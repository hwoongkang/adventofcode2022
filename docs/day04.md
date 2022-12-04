# 넷째 날

확실히 아직까지는 쉽습니다.  
`Range`라는 `struct`를 정의했고,  
`{min}-{max},{MIN}-{MAX}` 를 파싱하기 위한 로직을 만들었습니다:  
우선 input line을 `","`로 split한 다음, `{u32}-{u32}` 를 `Range`로 파싱할 수 있도록 하였습니다.

## Part 1

`Range`가 다른 `Range`를 "포함하고 있는지(contains)"를 체크하면 됩니다.  
me > other, other > me를 한 번에 테스트해도 되었지만,  
`contains`라는 어감이 하나만 테스트하는 것에 가까운 것 같아 me > other만 구현하고,  
콜할 때 양쪽을 다 테스트했습니다.

## Part 2

이번엔 두 `Range`가 "겹치는지(overlaps)"를 체크하면 됩니다.  
이번에는 `a.overlaps(b)`를 한 번만 콜하는 것으로 해결되게 짰습니다.

## 여담

`input_file_name`을 받는 게 `main`단에선 편한데, 내부에서 은근히 귀찮습니다.  
내일은 `Solution` trait을 살짝 고칠지도..
