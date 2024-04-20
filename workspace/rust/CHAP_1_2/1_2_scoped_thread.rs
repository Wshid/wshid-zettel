let numbers = vec![1,2,3];

thread::scope(|s| { // 스레드의 범위를 만들기 위해 scope에 클로저 전달, 클로저는 즉시 실행되며 현재 범위를 나타내는 s 인자 입력
	s.spawn(|| { // s를 사용하여 thread 생성. 스레드에 전달되는 클로저는 local 변수인 numbvers 사용 가능
		println!("length: {}", numbers.len());
	});
	s.spawn(|| {
		for n in &numbers {
			println!("{n}");	
		}	
	});
}); // 범위가 끝날때 종료되지 않은 스레드를 기다림