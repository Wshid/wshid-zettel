---
date: 2024-05-25
datetime: 2024-05-25 08:13:53
book: 러스트_동시성_프로그래밍
page:
  - "39"
tags: 
references: 
aliases:
  - std::marker::PhantomData<T>
---
[[Send]] 및 [[Sync]]를 구현하는 필드로 인해,
- 구조체 자체가 [[Send]], [[Sync]]를 자동으로 구현하는 것을 막을 수 있음
이 타입은 컴파일러가 `T`타입으로 인식하지만, 런타임에 해당 타입은 실제 존재 x
크기가 0인 타입. 메모리를 전혀 차지하지 않음

```rust
use std::marker::PhantomData;

struct X {
	handle: i32,
	_not_sync: PhantomData<Cell<()>>,
}
```
- `X`의 필드가 오직 handle이었다면
	- `Send`와 `Sync`를 구현할 것
- `_not_sync` 필드는 `PhantomData<Cell<()>>` 타입이고
	- `Cell<()>`은 `Sync` [[rust_trait|trait]]을 구현하지 않기 때문에
		- `X`는 `Sync` trait을 가지지 않음
	- `Cell<()>`는 `Send` trait을 구현하기 때문에
		- X는 `Send` trait을 갖음

컴파일러는 `*const T`나 `*mut T`와 같은 **원시 포인터**들이
- 어떤 값을 가리키도 있는지 알기 어렵기 때문에
- 이들은 [[Sync]], [[Send]] 모두 구현하지 x
이 경우에 Send, Sync trait을 구현하는 방법은
- 다른 모든 [[rust_trait|trait]]과 동일하게 `impl` 블록을 구조체에 추가하는 것

```rust
struct X {
	p: *mut i32,
}

unsafe impl Send for X {}
unsafe impl Sync for X {}
```
- 위 경우 컴파일러가 실제로 이 포인터가 가리키는 대상이
	- Send, Sync를 갖고 있는지 확인할 수 없기 때문에
	- `impl` 블록에 `unsafe`키워드를 추가해야 함
- `unsafe`를 사용하면, 컴파일러가 있다고 믿게됨
	- 코드가 제대로 동작하지 않을 경우 개발자 책임

`Send` trait을 구현하지 않는 타입의 객체를
- 다른 스레드로 소유권을 넘기려고 하면
- 컴파일러는 이 동작을 허용하지 않음
	- 컴파일 불가

```rust
fn main() {
	let a = Rc::new(123);
	thread::spawn(move || {
		// ERROR!
		dbg!(a);
	})
}
```
- 다른 스레드로 `Rc<i32>`를 보내려고 하지만,
	- `Arc<i32>`와 달리 `Rc<i32>`는 `Send` trait을 구현하지 x
- [[spawn|std::thread::spawn]] 함수는 인수가 모두 [[Send]] trait을 구현해야 하며,
	- [[closure]]가 캡쳐된 모든 변수가 [[Send]] [[rust_trait|trait]]을 구현하는 경우에만 클로저가 `Send` trait 구현 가능
- 여기서 `Send`가 아닌 것을 캡쳐하면
	- 정의되지 않은 동작 발생으로 코드가 컴파일 x