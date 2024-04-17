---
date: 2024-04-14
datetime: 2024-04-14 13:36:49
book: 러스트_동시성_프로그래밍
page:
  - "23"
tags: 
references: 
aliases:
---
-  **프로그램이 종료될 때까지 존재할 수 있는 함수**만 입력 받음
- `reference`로 **지역 변수**를 캡처하는 클로저는
	- 더 이상 지역변수가 존재하지 않는 순간 
	- reference가 유효하지 않음
	- 영원히 유지 불가
- [[closure]]의 return 값은 thread로 전달
	- `join`메서드 호출시 `Result`로 감싸져 전달
```rust
let numbers = Vec::from_iter(0..=1000);

let t = thread::spawn(move || {
	let len = numbers.len();
	let sum = numbers.into_iter().sum::<usize>();
	sum / len // 1
});

let average = t.join().unwrap(); //2
println!("average: {average}") 
```
- thread의 [[closure]](1)에서 리턴된 값은
	- join(2)를 통해 메인 스레드로 전달
- `if numbers==0`,
	- 1에서 panic 전달
	- unwrap(2)에서 main thread panic 발생