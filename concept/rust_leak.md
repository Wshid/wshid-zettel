---
date: 2024-04-28
datetime: 2024-04-28 13:34:31
book: 러스트_동시성_프로그래밍
page:
  - "28"
tags: 
references: 
aliases:
  - 누수
  - 메모리 누수
---
- 값의 할당을 누수(leaking)하는 방법
- [[rust_box|Box]]::leak`
	- Box의 소유권을 해제
	- 이 값이 drop되지 않도록 함
	- Box는 프로그램 종료시까지 존재
	- 어느 스레드에서도 값을 빌릴 수 있음
	- 메모리 누수를 의도적으로 발생
		- 정적 생명 주기를 가진 참조를 얻기 위한 목적
```rust
// Box는 Rust에서 heap 할당을 관리하는 포인터
let x: &'static [i32; 3] = Box::leak(Box::new([1,2,3])); // heap에 할당. Box<[i32; 3]> 타입 반환

thread::spawn(move || dbg!(x));
thread::spawn(move || dbg!(x));
```
- move [[closure]]가 값의 소유권을 스레드로 가져가는 것처럼 보이나,
	- x는 원래 단순히 `Box`의 [[rust_reference|reference]]
- `&'static`
	- 데이터에 대한 무효화되지 않는(raw) 불변 참조
- [[rust_quote_static|'static]]
- 위와 같이 사용할 경우, 메모리가 누수됨
	- 객체를 메모리 할당 후 해제하지 않음
- 이 패턴을 자주 사용하게 되면 program heap memory가 부족해짐