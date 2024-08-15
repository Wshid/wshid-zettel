---
date: 2024-07-05
datetime: 2024-07-05T23:29:00
book: 러스트_동시성_프로그래밍
page: 
tags: 
references: 
aliases:
---
# 아토믹(Atomic)
- 더 이상 작은 것으로 나누어 질 수 없음
- 완전히 수행되었거나 수행되지 않은 상태
- [[1_4_소유권_대여와_데이터_경합]]에서 처럼,
	- 여러 스레드가 동시에 동일한 변수를 읽고 변경하는 것은 정의되지 x
	- 하지만 아토믹 연산을 사용하면, 여러 스레드가 안전하게 동일한 변수의 RW 가능
- 하드웨어 수준에서 아토믹이 어떻게 작동하는지는 7장에서 언급 예정
- 아토믹 연산은 **멀티 스레드**와 관련된 모든 것의 기초가 됨
	- [[Mutex|뮤텍스]]나 [[1_8_대기_파킹과_조건_변수#1.8.2. 조건 변수|조건 변수]]와 같은 모든 동시성의 기본 구성 요소가 **아토믹 연산**으로 구현된 것들
- `std::sync::atomic`에 정의된 아토믹 타입들의 메서드로 사용 가능
- `AtomicI32`나 `AtomicUsize`와 같이 `Atomic`으로 시작하는 이름을 가짐
- 사용 가능한 아토믹 타입은 **하드웨어 아키텍처**와 **운영체제**에 따라 다르나,
	- 거의 모든 플랫폼에서 최소한 **단일 포인터 크기**까지의 모든 아토믹 타입을 제공
- 대부분의 타입들과 다르게, 아토믹 타입들은 [[reference_&T|공유 레퍼런스]](`&AtomicU8`)을 통한 값의 변경이 가능함
	- [[1_5_내부_가변성|내부 가변성]] 덕분에 가능한 일
- 아래 연산들에 대해 동일한 인터페이스를 가짐
	- `store-and-load`
	- `fetch-and-modify`
	- `compare-and-exchange`

## 메모리 순서(memory ordering)
- 모든 아토믹 연산은 `std::sync::atomic::Ordering` 타입의 파라미터를 입력 받음
- 아토믹 연산 간의 **상대적인 연산 순서**에 대해 무엇을 보장할지를 결정
	- 가장 적은 보장을 제공하는 간단한 변형은 `Relaxed`
- [[Relaxed]]
- 두 스레드에서 변수들에 대한 연산이 **스레드마다 다른 순서로 발생**할 수 있다는 의미
	- e.g. A 스레드가 a 변수에 값을 쓰고, b 변수에 빠르게 쓸 경우
		- B 스레드에서는 해당 내용이 반대 순서로 발생하는 것처럼 보임
- 2장에서는 위 문제 대신 `Relaxed`를 사용하여 아토믹 연산을 간단하게 살펴볼 예정

## 2.1. 아토믹한 데이터 load 및 store 연산
- `load`, `store`는 가장 기본적인 연산
- `AtomicI32`를 예시로 다음과 같이 구성됨
```rust
impl AtomicI32 {
	pub fn load(&self, ordering: Ordering) -> i32;
	pub fn store(&self, value: i32, ordering: Ordering)
}
```
- load: 아토믹 변수에 저장된 값을 아토믹하게 읽어들이기
- store: 메서드는 아토믹의 새 값을 변수에 씀
	- 값을 수정하더라도 [[독점 레퍼런스|&mut]]가 아닌 [[reference_&T|&T]]를 사용함
- 두 메서드의 실질적인 예시들

## 예시 1. 정지 플래그
- `AtomicBool`을 **정지 플래그**(stop flag)로 사용하는 것
- 해당 플래그는 **다른 스레드를 멈추는데 사용함**
```rust
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
	static STOP: AtomicBool::new(false);
	// 작업을 수행할 스레드를 생성함
	let background_thread = thread::spawn(|| {
		while !STOP.load(Relaxed) {
			some_work();
		}
	});

	// 메인 스레드에서 사용자 입력을 받음
	for line in std::io::stdin().lines() {
		match line.unwrap().as_str() {
			"help" => println!("commands: help, stop"),
			"stop" => break,
			cmd => println!("unknown command: {cmd:?}"),
		}
	}

	// 백그라운드 스레드가 멈추도록 함
	STOP.store(true, Relaxed);

	// 백그라운드 스레드가 멈출 때까 기다림
	background_thread.join().unwrap();
}
```
- 백그라운드 스레드가 `some_work()`를 반복적으로 실행하는 동시에,
	- 메인 스레드에서는 사용자가 프로그램과 상호작용 하기 위한 몇가지 명령 입력
	- `stop`이라는 유효한 명령어 사용 가능
- 백그라운드 스레드가 중지되도록 하려면,
	- `STOP` 변수를 사용하여, 이 조건을 백그라운드 스레드에 전달
- 메인 스레드가 사용자로부터 중지 명령을 받으면
	- 플래그를 `true`로 설정하고,
	- 백그라운드 스레드는 새로운 반복이 시작되기 전 이 플래그를 확인
- 메인 스레드는 `join` 메서드를 사용하여 백그라운드 스레드가 현재 반복을 완료할때까지 대기
- 백그라운드 스레드가 정기적으로 확인하는 한, 이 해결책은 잘 작동하나,
	- 스레드가 `some_work()`에 오랫동안 대기한다면,
	- 중지 명령 - 프로그램 종료 사이에 상당한 지연이 발생할 수 있음


## 예시 2. 진행 상황 보고
- 백그라운드 스레드에서 100개의 아이템을 하나씩 처리
- 메인 스레드는 진행 상황을 정기적으로 사용자에게 보고

```rust
use std::{sync::atomic::AtomicUsize, time::Duration};

fn main() {
	let num_done = AtomicUsize::new(0);

	thread::scope(|s| {
		// 백그라운드 스레드가 아이템 100개를 처리함
		s.spawn(|| {
			for i in 0..100 {
				process_item(i); // 이 작업에 시간 소요 가정
				num_done.store(i+1, Relaxed);
			}
		});
		// 메인 스레드는 매초 상태 업데이트를 출력
		loop {
			let n = num_done.load(Relaxed);
			if n == 100 {
				break;
			}
			println("Working.. {n}/100 done");
			thread::sleep(Duration::from_secs(1));
		}
	});
	println!("Done!");
}
```
- [[1_2_범위_스레드|범위 스레드]]를 사용해 스레드가 자동으로 조인되고,
	- 지역 변수의 소유권을 대여할 수 있도록 구성
- 매번 백그라운드 스레드가 아이템 작업을 끝내면,
	- 지금까지 처리한 아에팀 수를 `AtomicUsize`에 저장함
- 메인스레드는 1초에 한번 이 숫자를 사용자에게 보고
- 만약, 메인스레드가 100개 모두 처리되었다고 판단시,
	- 즉시 loop를 벗어나고 스레드를 암시적으로 조인한 다음 사용자에게 모든 작업이 끝났음을 보고

### 동기화
- 마지막 항목이 처리되면, 메인 스레드가 이를 인식하는데 최대 1초가 걸림
	- 불필요한 지연
- 이 문제를 해결하기 위해 [[1_8_대기_파킹과_조건_변수#1.8.1. 스레드 파킹|스레드 파킹]]을 사용하여
	- 관심 가질 만한 새로운 정보가 있을 때마다
	- 메인 스레드를 잠자기 상태에서 깨울 수 있음
- `thread::sleep` 대신 `thread::park_timeout`을 사용

```rust
fn main() {
	let num_done = AtomicUsize::new(0);
	let main_thread = thread::current();
	thread::scope(|s| {
		// 백그라운드 스레드가 아이템 100개를 처리
		s.spawn(|| {
			for i in 0..100 {
				process_item(i); // 이 작업에 시간이 소요된다 가정
				num_done.store(i+1, Relaxed);
				main_thread.unpark(); // 메인 스레드를 깨움
			}
		});

		// 메인 스레드는 매초 상태 업데이트 출력
		loop {
			let n = num_done.load(Relaxed);
			if n = 100 {
				break;
			}
			println!("Working.. {n}/100 done");
			thread::park_timeout(Duration::from_secs(1));
		}
	});
	println!("Done!");
}
```
- 백그라운드 스레드에서 모든 상태가 업데이트 되면, 메인 스레드의 파킹을 해제
	- `thread::current()`를 통해 획득한 메인 스레드 핸들 사용 가능
	- 이제 메인 스레드는 `sleep`대신 `park_timeout`을 사용하여 중단 가능

## 예시 3. 게으른 초기화
- lazy initialization
- 파일에서 읽거나, 운영체제에서 얻거나, 다른 방식으로 계산에서 얻은 값 x
	- 프로그램을 실행 하는 동안 일정할 것으로 가정
	- x는 운영 체제의 버전, 총 메모리 크기 등의 값일 수 있음
- x 값이 변경되지 않을 것이므로,
	- x가 필요한 첫 시점에만 x를 요청하거나 계산하고 결과를 기억해두면 됨
- 이 값이 필요한 첫 번째 스레드가 값을 계산하고
	- 아토믹 `static`으로 값을 저장해두면
	- 첫 스레드를 포함한 모든 스레드에서 이 값이 필요할 때 접근 가능
- 예시에서는 문제를 간단히 하기 위해 `x != 0`이라고 가정
	- x가 계산되기 전의 기본값으로 0을 선택할 수 있음

```rust
use std::sync::atomic::AtomicU64;

fn get_x() -> u64 {
	static X: AtomicU64 = AtomicU64::new(0);
	let mut x = X.load(Relaxed);
	if x == 0 {
		x = calculate_x();
		X.store(x, Relaxed);
	}
	x
}
```
- `get_x()`를 처음 호출하는 스레드는 연산 및 그 결과를 `static`에 저장
	- 나중에 사용할 수 있음
- 이후 `get_x()`를 호출하면 재계산 없이 즉시 반환
- 하지만 첫 번째 스레드가 x를 계산하는 동안, 두번째 스레드가 `get_x()`를 호출하면
	- 동시에 계산이 이루어짐
	- **스레드 경합**(race)이 일어남
- 데이터 경합은 아님
	- `unsfae`를 사용한 정의되지 않은 동작은 아니기 때문
- 하지만 여전히 어떤 스레드가 값에 먼저 접근하는지 모르기 때문에 경합 상태
- 누가 경합에서 이기느냐는 중요하지 않음
	- x 값이 일정할 것이므로
	- 이는 상황에 따라 효과적일수도, 아닐수도 있음
- `calculate_x()`를 실행하는데 오래 걸릴 경우
	- 첫 번째 스레드가 `X`를 계산하는 동안
	- 다른 스레드가 대기 상태로 있다면, 프로세서 시간을 효율적으로 사용할 수 있음
- [[1_8_대기_파킹과_조건_변수#1.8.2. 조건 변수|조건 변수]]나 [[1_8_대기_파킹과_조건_변수#1.8.1. 스레드 파킹|스레드 파킹]]을 사용하여 구현 할 수 있으나,
	- 간단한 과제에 비해 너무 복잡함
- 러스트 표준 라이브러리는 `std::sync::Once`와 `std::sync::OnceLock`를 통해 이 기능을 제공하기 때문에
	- 우리가 직접 구현할 필요는 없음
