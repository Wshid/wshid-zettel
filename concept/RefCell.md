---
date: 2024-05-17
datetime: 2024-05-17 23:50:42
book: 러스트_동시성_프로그래밍
page:
  - "37"
tags: 
references: 
aliases:
---
- 일반적인 [[Cell]]과 다르게 `std::cell::RefCell`에서 내부 값 대여 가능
	- 대신 Runtime에서의 비효율성 감수 필요
- `RefCell<T>`는 단순히 `T`를 가지고 있는 것이 아닌
	- `T`를 외부에서 몇 번 대여했는지를 내부적으로 추적
- 만일 [[가변 소유권 대여]]로 이미 대여된 상태라면
	- 새롭게 가변 소유권 대여를 만들려고 하는 순간
	- [[panic]]이 발생함
- Cell과 마찬가지로 RefCell 역시 **단일 스레드**에서만 사용 가능 

```rust
use std::cell::RefCell;

fn f(v: &RefCell<Vec<i32>>) {
	v.borrow_mut().push(1); // `Vec`을 직접 수정 가능
}
```