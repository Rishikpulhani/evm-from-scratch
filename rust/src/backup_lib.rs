if opcode == 0x09 {
    pc += 1;
    let num1 = stack.pop().unwrap();
    let hex_string_format = format!("{:x}", num1);
    println!("Hexadecimal string using format! macro: {}", hex_string_format);
    let num2 = stack.pop().unwrap();
    println!("num2 is {num2}");
    let num3 = stack.pop().unwrap();
    println!("num3 is {num3}");
    if num3 == U256::from(0) {
        stack.push(U256::from(0));
    } else {
        let (intermediate, overflow_status) = num1.overflowing_mul(num2);
        println!("Intermediate is {intermediate}, overflow status: {overflow_status}");
        let result = intermediate % num3;
        stack.push(result);
    }
}