---
date: 2024-08-15
datetime: 2024-08-15T20:12:00
book: 러스트_동시성_프로그래밍
page: 
tags: 
references: 
aliases:
---
- fetch-and-modify
	- 아토믹 변수의 값을 변경하는 연산
	- 원래 값을 읽어오는 (가져오는) 아토믹 연산
- `fetch_add`: 덧셈
- `fetch_sub`: 뺄셈
- `fetch_or, fetch_and`: 비트연산
- `fetch_max, fetch_min`: 최대/최소 계산

### `AtomicI32`를 사용하는 시그니처
```rust
impl AtomicI32 {
	pub fn fetch_add(&self, v: i32, ordering: Ordering) -> i32;
	pub fn fetch_sub(&self, v: i32, ordering: Ordering) -> i32;
	...
	pub fn swap(&self, v: i32, ordering: Ordering) -> i32; // fetch_store 대신 사용
}
```
- 이전값과 무관하게 새 값을 저장하는 연산
	- `fetch_store` 대신 `swap`이라는 네이밍을 가짐

### fech_add 함수 예시
```rust
use std::sync::atomic::AtomicI32;

let a = AtomicI32::new(100);
let b = a.fetch_add(23, Relaxed);
let c = a.load(Relaxed);

assert_eq!(b, 100);
assert_eq!(c, 123);
```
- `fetch_add`는 `100 -> 123`으로 값을 증가 시킴
	- 하지만 리턴시, `100`을 리턴(증가되기 전 값)
- 이 다음에 이루어지는 어떤 연산이든 사용하는 값은 123
- **연산에서 리턴되는 값이 항상 의미가 있지는 않음**
- 아토믹 변수에 연산이 적용되기만을 원하고, **원래 값 자체는 필요 없다면**
	- 리턴값은 무시해도 됨
- 중요한점?
	- `fetch_add/fetch_sub`의 구현이 오버플로 wrapping 구현
		- 내부 자료형이 표현 가능한 최대값에 도달시, 최소값이 할당됨
		- 일반 정수 자료형에 대해 디버그 모드에서 오버플로 발생하는 경우 [[panic]]을 일으키는 것과는 다른 결과
- 아래 내용들은 `fetch-and-modify` 연산의 실제 사례

## 예제1. 여러 스레드의 진행 상황 보고
- [[2_1_아토믹한_데이터_load_및_store_연산#예시 2. 진행 상황 보고|2.1.2]]절에서 `AtomicUsize`를 백그라운드 스레드의 진행 상황을 확인하는데 사용
	- 만일 아이템 100개를 4개의 스레드로 분산하여, 25개씩 처리시
	- 스레드 4개의 각 진행상황을 알고 싶을 것
- 각 스레드마다 `AtomicUsize`를 만들고, 메인스레드에서 4개의 값들을 불러와 더하면
	- 전체 진행도를 알 수 있음
- 하지만 더 쉽게 하나의 `AtomicUsize`로 전체 스레드의 진행 상황을 계산하면 됨
- `store`메서드는 다른 스레드의 진행 상황을 덮어쓰기 때문에 사용 X
	- 대신, **아토믹 덧셈 연산**을 사용해 모든 항목이 처리된 후 **카운트 증가**
- [[2_1_아토믹한_데이터_load_및_store_연산#예시 2. 진행 상황 보고|2.1.2]]절의 예시를 다음과 같이 4개의 스레드로 분산하기

```rust
fn main() {
	let num_done = &AtomicUsize::new(0);

	thread::scope(|s| {
		// 아이템 100개를 스레드 4개가 각각 25개씩 처리
		for t in 0..4 {
			s.spawn(move || {
				for i in 0..25 {
					process_item(t * 25 + i); // 이 작업에 시간이 소요된다고 가정
					num_done.fetch_add(1, Relaxed);
				}
			});
		}
		// 메인 스레드는 매초 상태 업데이트를 출력
		loop {
			let n = num_done.load(Relaxed);
			if n == 100 {
				break;
			}
			println!("Working.. {n}/100 done");
			thread::sleep(Duration::from_secs(1));
		}
	});
	println!("Done!");
}
```
- 기존 예시와의 차이
	- 백그라운드에 4개의 스레드가 생성됨
	- `store`메서드 대신 `fetch_add`를 사용하여 `num_done` 변수를 변경함
- 백그라운드 스레드의 클로저가 `move` 키워드를 사용하고,
	- `num_done`은 레퍼런스가 됨
	- `fetch_add` 때문이 아닌 4개의 스레드를 만들기 위해서임
- [[closure]]는 `t`를 캡쳐해서 어떤 스레드인지를 파악하고
	- 값을 `0, 25, 50, 75` 중에 어디서부터 더해갈지를 결정
- `move`가 없다면 [[closure]]는 `t`를 레퍼런스로 캡쳐하려고 하지만,
	- `t`의 라이프 타임은 루프 안에서만 유효하기 때문에
	- 이보다 더 오래 생존하는 클로저에서는 `t`를 캡쳐할 수 없음
- [[move closure]]
- 클로저가 `num_done`역시 캡처하기 때문에, 이 값은 레퍼런스가 됨
- 모두 동일한 `AtomicUsize`의 소유권을 대여하기 때문
- 아토믹 타입들은 `Copy` trait을 갖고 있지 않아서,
	- 해당 타입을 하나 이상의 스레드로 이동시키려고 하면 에러가 발생함
- 클로저에서 값을 캡처하는 것의 차이점을 제외한다면,
	- `fetch_add` 함수를 쓰도록 바꾼 것은 매우 간단한 변경
- 스레드가 어떤 순서로 `num_done`을 증가시킬지는 알 수 없지만,
	- 덧셈 연산은 아토믹하기 때문에 모든 스레드의 작업이 끝나면
	- 정확히 결과가 100이 될 것이라고 확신할 수 있음