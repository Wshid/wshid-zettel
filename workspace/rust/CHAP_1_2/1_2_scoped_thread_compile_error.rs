let mut numbers = vec![1,2,3];

thread::scope(|s| {
    s.spawn(|| {
        numbers.push(1);
    });
    s.spwan(|| {
        numbers.push(2); // 에러 발생, cannot burrow `numbers` as mutable more than once at a time
    })
})