---
date: 2024-05-17
datetime: 2024-05-17 23:48:13
book: 러스트_동시성_프로그래밍
page:
  - "36"
tags: 
references: 
aliases:
---
- `std::cell:Cell<T>`
	- 단순히 `T`를 감싸고 있는 타입
	- 공유 레퍼런스로 값을 변경할 수 있음
- 값을 꺼내어 복사하거나, 값 전체를 다른 값으로 교체하는 것만 가능
- 이 작업은 단일 스레드에서만 가능
```rust
use std::cell:Cell;

fn f(a: &Cell<i32>, b: &Cell<i32>) {
	let before = a.get();
	b.set(b.get() + 1);
	let after = a.get();
	if before != after {
		x(); // 실행될 수도 있음
	}
}
```
- 이전 예시에서 `i32 -> Cell<i32>`로 변경
- if문 만족이 가능함
	- `Cell<i32>`의 내부 가변성 때문에
	- 컴파일러는 공유 레퍼런스가 존재하는 동안에는 더 이상 해당 변수의 값이 바뀌지 않는다고 가정할 수 없음
	- a와 b는 같은 값 참조 가능
	- b를 변경하는것이 a 값에 영향을 줄 수 있음
	- 단, 단일 스레드에서만 작동하는 상황만 가정함
- 위 제약은 `Cell`을 사용하기 더 어렵게 만드는 이유중 하나
	- Cell의 값을 직접 대여할 수 없으므로, `Cell`로부터 값을 꺼내고 다른 값을 채워 넣어야 함
```rust
fn f(v: &Cell<Vec<i32>>) {
	let mut v2 = v.take(); // Cell을 빈 Vec로 대체
	v2.push(1);
	v.set(v2); // 값이 변경된 Vec을 다시 입력
}
```

