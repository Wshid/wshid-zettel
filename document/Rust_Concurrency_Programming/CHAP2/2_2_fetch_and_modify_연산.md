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

## 예제2. 통계
- 아토믹을 통해 다른 스레드가 수행하는 작업 상황을 보고하는 예제를 확장
	- 각 아이템을 처리하는데 걸리는 시간에 대한 통계 수집
- `num_done` 및 `total_time`과 `max_time`이라는 2개의 아토믹 변수 추가
	- 이 변수를 사용하여 평균 처리 시간과 최대 처리 시간을 보고

```rust
use std::time::Instant;

fn main() {
	let num_done = &AtomicUsize::new(0);
	let total_time = &AtomicU64::new(0);
	let max_time = &AtomicU64::new(0);

	thread::scope(|s| {
		// 아이템 100개를 스레드 4개가 각각 25개씩 처리
		for t in 0..4 {
			s.spawn(move || {
				for i in 0..25 {
					let start = Instant::now();
					process_item(t * 25 + i); // 이 작업에 시간이 소요된다고 가정
					let time_taken = start.elapsed().as_micros() as u64;
					num_done.fetch_add(1, Relaxed);
					total_time.fetch_add(time_taken, Relaxed);
					max_time.fetch_max(time_taken, Relaxed);
				}
			});
		}
		// 메인 스레드는 매초 상태 업데이트를 출력함
		loop {
			let total_time = Duration::from_micros(total_time.load(Relaxed));
			let max_time = Duration::from_micros(max_time.load(Relaxed));
			let n = num_done.load(Relaxed);
			if n == 100 {
				break;
			}
			if n == 0 {
				println!("Working.. nothing done yet.");
			} else {
				println!(
					"Working.. {n}/100 done, {:?} average, {:?} peak",
					total_time / n as u32,
					max_time,
				);
			}
			thread::sleep(Duration::from_secs(1));
		}
	});

	println!("Done!");
}
```
- 백그라운드 스레드는
	- `Instant::now()`, `Instant::elapsed()`를 사용하여 `process_item()`에 소요된 시간을 측정
- 아토믹 덧셈(atomic add) 연산은 `total_time`에 micro 초를 더하는데 사용됨
- 아토믹 최대(atomic max) 연산은 `max_time`에서 가장 높은 측정값을 기록하는데 사용
- 메인 스레드는
	- `총 시간 / 처리된 항목수 = 평균 처리 시간`을 구함
	- `max_time`의 최대 처리 시간과 함께 보고
- 문제 1. 평균 과소 추정
	- 아토믹 변수 3개가 개별적으로 업데이트 됨
	- 메인 스레드는 스레드가 `num_done`을 증가시킨 후에도 `total_time`을 업데이트 하기 전에 값을 읽어들일 수 있음
	- 그 결과, 평균이 **과소 추정**됨
- 문제 2. 평균 과대 추정
	- [[Relaxed]] 메모리 순서는 다른 스레드에서 볼 때 **상대적인 연산 순서를 보장하지 않기 때문에**
		- 새로 업데이트 된 `total_time` 값과
		- 업데이트 되지 않은 `num_done`의 값으로 평균을 과대 추정할 수 있음
- 이 예제에서는 두가지 모두 큰 문제가 되지 않음
- 발생할 수 있는 최악의 상황?
	- 사용자가 부정확한 평균을 잠깐 동안 볼 수 있음
- 위 문제를 방지하려면 세가지 통계를 [[Mutex]]안에 넣으면 됨
	- 이후, 숫자를 3개 업데이트 하는 동안 **뮤텍스를 잠시 잠그면** 더 이상 변수들이 [[Atomic|아토믹]]일 필요가 없음
	- [[Mutex]]를 잠그고 해제하고 스레드를 일시적으로 차단하는데 추가 비용 소모
	- but, 세가지 업데이트를 **단일 아토믹 연산**으로 처리할 수 있음

## 예제 3. ID 할당
- `fetch_add`의 리턴값이 필요한 예제
- `allocate_new_id()`가 호출될 때마다 새로운 숫자를 리턴하는 함수가 필요하다고 가정
	- 이 숫자를 사용해 프로그램에서 어떤 작업과 같은 요소 구분시 사용 가능
- 정수와 같이 값의 크기가 작으면서 스레드간 자유롭게 전달할 수 있는 타입 필요
- 이 함수를 만들다보면, `fetch_add`를 사용해야 함
```rust
use std::sync::atomic::AtomicU32;

fn allocate_new_id() -> u32 {
	static NEXT_ID: AtomicU32 = AtomicU32::new(0);
	NEXT_ID.fetch_add(1, Relaxed)
}
```
- 다음에 제공할 숫자를 기록하고 값을 읽을 때마다 값을 1씩 증가 시키면 됨
	- 함수를 처음 호출한 스레드에는 0, 두번째에는 1을 보내는 식
- 위 코드의 유일한 문제?
	- 오버플로우가 발생할 때
	- 32비트 정수의 범위를 벗어날때, 다음 호출은 0을 리턴하게 됨
- 이 코드를 어떻게 사용하느냐에 따라 문제의 양상이 달라짐
	- 이 함수를 얼마나 자주 호출하는가
	- 중복된 숫자 리턴시 문제는 무엇인가
- 현대 컴퓨너는 수 초면 이정도 개수의 함수 호출이 가능함
	- **메모리 안전성이 리턴된 값들의 유일성의 의존한다면, 함수는 이렇게 구현해서는 안됨**
- 너무 많은 함수가 호출되는 경우 [[panic]]이 발생할 수 있음

```rust
// 이 코드는 문제가 있음
fn allocate_new_id() -> u32 {
	static NEXT_ID: AtomicU32 = AtomicU32:new(0);
	let id = NEXT_ID.fetch_add(1, Relaxed);
	assert!(id < 1000, "too many IDs!");
	id
}
```
- `assert`문은 함수가 1000번 호출된 후에 [[panic]]상태에 빠짐
- 하지만 이미 아토믹 덧셈 연산이 수행된 이후 발생하기 때문에
	- `NEXT_ID=1001` 상태일 것
- 다른 스레드가 이 함수를 호출하면, 패닉에 빠지기 전에 이미 `1002`로 증가하게 됨
- 결국 여러번의 패닉이 발생된 후, `NEXT_ID=0`으로 오버플로되면서 동일 문제 발생

### 위 문제에 대한 3가지 해결책

#### 1. 패닉을 발생시키지 말고 오버플로 발생시 프로세스를 완전히 중지하기
- `std::process:abort` 함수는 전체 프로세스를 중단. 함수가 계속 호출될 수 없도록 함
- 프로세스를 중단하면, 다른 스레드에서 함수를 계속 호출할 수 있는 틈이 생기나,
	- 프로그램이 실제 중단되기 전까지 함수가 42억번 호출될 일은 없음
- 실제 표준 라이브러리 `Arc::clone()`에서 오버플로 검사가 구현되는 방식
	- `isize::MAX`만큼 `Arc`를 복제할 경우를 대비하여 이렇게 구현되어 있음

#### 2. 패닉이 발생하기 전에 `fetch_sub`를 사용하여 카운트를 줄이기
```rust
fn allocate_new_id() -> u32 {
	static NEXT_ID: AtomicU32 = AtomicU32::new(0);
	let id = NEXT_ID.fetch_add(1, Relaxed);
	if id >= 1000 {
		NEXT_ID.fetch_sub(1, Relaxed);
		panic!("too many IDs!");
	}
}
```
- 물론, 아주 잠깐 동안 `1000`이상으로 카운트가 증가할 수 있으나
	- 대부분의 경우 활성 스레드 수가 정해져 있어, 오버플로가 일어날 일은 많지 않음
- 전제
	- 동시에 수십억 개의 활성 스레드가 존재하지 않고
	- `fetch_add, fetch_sub` 사이의 짧은 순간에 모두 동일한 함수를 동시에 실행하지 않음
- 표준 라이브러리의 `thread::scope` 구현에서
	- 실행중인 스레드 수에 대한 오버플로가 처리되는 방식

#### 3. 오버플로가 발생할 경우 덧셈이 일어나지 않아서 실제로 가장 좋은 방식
- 하지만 `Atomic`연산은 이를 구현하기 어려움
- `compare_and_exchange`연산이 필요함