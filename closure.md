---
date: 2024-04-14
datetime: 2024-04-14 13:39:02
book: 
page: 
tags: 
references:
  - https://velog.io/@gamjagoon/Rust-%ED%81%B4%EB%A1%9C%EC%A0%80closure
  - https://blacklobster.tistory.com/7
  - https://hanrabong.com/entry/Javascript-Closure%EB%9E%80
aliases:
---
- 일급 객체 함수 개념을 이용하여 스코프에 묶인 변수를 바인딩 하기 위한 기술
- 클로저는 함수를 저장한 레코드
- 스코프틔 인수들은 클로즈 생성시 정의
- 스코프 내부 영역이 소멸되었어도, 그에 대한 접근은 복사본인 클로저에 의해 이루어짐
-  **특정 값을 스레드 안으로 이동** 가능

### Rust에서 클로저를 사용하는 이유

1. 함수를 파라미터로 받는 함수 호출시, 이 한 곳에서만 호출되는 경우 이름이 필요 x
2. 가독성 확보
	1. 익명 함수를 바로 바라보기 때문
3. 리소스 절감
	1. 메모리 및 CPU 사용량이 똑같은 함수의 비용보다 높지 x
4. Stack 할당
	1. Rust의 경우 일부러 Box, Vec등의 컨테이너에 집어 넣지 않을경우 Stack에 할당
	2. Heap을 사용하지 x

### Javascript
-  **함수가 선언될(생성될) 그 당시에 주변의 환경과 함께 갇히는 것**
- **함수가 속한 렉시컬 스코프(Lexical Environment)를 기억하여, 함수가 렉시컬 스코프 밖에서 실행될 때도 이 스코프에 접근할 수 있게 해주는 기능**
	- **렉시컬 스코프란 함수가 선언이 되는 위치에 따라서 상위 스코프가 결정되는 스코프**
	- lexing time에 scope가 정해지는 것
		- 함수를 호출할 때가 아닌(runtime x)
		- 함수를 어디서 선언하였는가에 따라 scope 지정
- 내부함수는 외부함수의 지역변수에 접근할 수 있는데
	- 외부함수의 실행이 끝나서 외부함수가 소멸된 이후에도 
	- 내부함수가 외부함수의 변수에 접근할 수 있는 것
- 모든 자바스크립트 함수는 함수가 선언(생성)될 당시의 closure가 생성
	- 주변 환경을 기억할 수 있음
- OOP의 private와 유사?
	- 외부에서는 접근 불가능, 내부에서만 가능한 변수, 함수 생성 가능
	- 클래스 내부 로직과 유사한 듯 함 
```js
function Counter() {
	let cnt = 0; // private 변수

	this.up = function () {
		return ++cnt;
	}

	this.down = function () {
		return --cnt;
	}
}

let count = new Counter();

console.log(count.up());    //  1
console.log(count.up());    //  2
console.log(count.down());  // 1
```