---
date: 2024-04-28
datetime: 2024-04-28 13:35:17
book: 데이터_품질의_비밀
page:
  - "28"
  - "29"
tags: 
references: 
aliases:
  - 레퍼런스 카운팅
  - Rc
---
- 스레드간 공유 데이터가 확실히 drop, memory 해제 하려면
	- **소유권을 포기해서는 안됨**
	- 단, **소유권을 공유**하면 가능
- 레퍼런스 카운팅
	- 데이터 소유자들을 지속적으로 관리,
	- 해당 소유자가 없을 때 객체를 제거
	- `std::rc::Rc` 타입 사용
- [[rust_box|Box]]와 유사하나, 데이터를 복제하면
	- 새로운 데이터가 메모리가 할당되는 것이 아닌,
	- 레퍼런스 카운터의 값이 증가
- 원본 `Rc`와 복제된 `Rc` 모두 같은 메모리에 할당된 값 참조
	- 소유권을 공유한다는 의미
```rust
use std::rc::Rc;

let a = Rc::new([1,2,3]);
let b = a.clone();

assert_eq!(a.as_ptr(), b.as_ptr()); // 동일한 메모리를 가리킴
```
- `a:Rc`를 `clone`하여 `b`를 생성
- `Rc`가 삭제되면, 카운터가 감소됨
	- 마지막 존재하는 Rc가 drop되면, counter=0, 메모리에서 값 할당이 해제됨
- Rc를 다른 스레드로 보내려고 하면 에러 발생
	```rust
	// Rc cannot be sent between threads safely
	thread::spwan(move || dbg!(b));
	```
- Rc는 [[스레드 안전성]]이 보장되지 않음
	- 여러개의 스레드가, 특정 값에 대해 Rc 사용시,
	- 각 스레드에서 Rc를 동시에 변경 가능 => 예측 불가능한 상황 발생
#### [[rust_arc|아토믹한 레퍼런스 카운팅]]

