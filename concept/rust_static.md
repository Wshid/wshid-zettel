---
date: 2024-04-28
datetime: 2024-04-28 13:33:14
book: 러스트_동시성_프로그래밍
page:
  - "27"
tags: 
references: 
aliases:
  - static
---
 특정 스레드에 종속되지 않는 변수를 만드는 방법
- `static`은 프로그램 자체가 소유권을 가짐
	- 어떤 스레드보다 오래 존재 가능
- 아래 두 스레드에서 변수 접근은 가능하나, 소유 불가
```rust
static X: [i32; 3] = [1,2,3]; 
thread::spawn(|| dbg!(&X));
thread::spawn(|| dbg!(&X));
```
- `static`으로 선언된 변수는 일정한 `initializer`를 가짐
	- drop되지 않으며, 프로그램 실정 전에 생성
	- 어떠한 스레드도 `static` 변수로부터 값을 빌려올 수 있음