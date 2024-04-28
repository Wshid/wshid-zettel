---
date: 2024-04-28
datetime: 2024-04-28 13:37:00
book: 러스트_동시성_프로그래밍
page:
  - "29"
tags: 
references: 
aliases:
  - Arc
  - 아토믹한 레퍼런스 카운팅
---
- `std::sync::Arc`
- Rc와 동일한 기능을 제공하나, `Arc`는 여러 스레드에서 레퍼런스 카운터를 변경하는 것이 허용됨
- 레퍼런스 카운터가 변경되는 작업이 atomic,
	- 여러 개의 스레드에서 동시에 카운터를 변경하더라도 [[스레드 안전성]]이 보장됨

```rust
use std::sync::Arc;

let a = Arc::new([1,2,3]); // Rc=1
let b = a.clone(); // Rc=2, a의 주소 = b의 주소

// 스레드마다 고유한 Arc를 전달받음(=배열이 스레드 사이에서 공유)
// 각 스레드에서 Arc가 삭제될때마다 Rc가 감소
thread::spawn(move || dbg!(a));
thread::spawn(move || dbg!(b));
```

#### Clone에 이름 붙이기
- Arc를 클론할때마다 새로운 이름을 붙이면 코드의 가독성이 떨어짐
	- 모든 Arc Clone은 모두 별도의 객체이나, 같은 값을 공유
	- Clone의 다른 이름을 붙인다면, 같은 값 공유한다는 사실 표현이 어려움
- shadowing
	- 같은 이름으로 새로운 변수를 만드는 방법
	- 같은 범위 안에서 섀도잉 할 경우, 새로운 변수가 만들어 짐
		- 기존 변수명을 사용할 수 없음
	- 하지만 새로운 범위를 생성하게 되면
		- `let a = a.clone();`과 같은 코드에서 `a`라는 변수명 재사용 가능
		- 기존 변수명은 **범위 바깥**에서 그대로 사용 가능
	- `{}`를 사용하여 새로운 범위에서 [[closure]]를 만들때,
		- 클로저 안으로 변수를 이동시키기 전에 변수 클론 가능
		- 클론된 변수에 새로운 이름을 붙일 필요 x

```rust
let a = Arc::new([1,2,3,]);
let b = a.clone();
thread::spawn(move || {
	dbg!(b);
});
dbt!(a);
```
- Arc와 Arc의 클론이 모두 같은 범위 안에 존재
- 각 스레드가 서로 다른 이름을 갖는 클론을 입력 받음

```rust
let a = Arc::new([1,2,3]);
thread::spawn({
	let a = a.clone();
	move || {
		dbg!(a);
	}
});
dbg!(a);
```
- Arc와 Arc의 클론은 다른범위 존재
- 스레드 안에서 계속 같은 이름 사용 가능