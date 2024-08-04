use primitive_types::U256;
mod helper;

// info on U256 and its implementation
//The U256 type is typically defined as a struct containing multiple smaller integer types (e.g., u64) to represent a 256-bit integer:pub struct U256([u64; 4]); 
// we can also preform bitwise multiplications, there are functions in rust which help us do it 
pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}



pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut stack: Vec<U256> = Vec::new();
    let mut pc : usize = 0;
    //let mut stack1 :Vec<String> = Vec::new();

    let code = _code.as_ref();
    println!("value of code is {:?}",code);

    while pc < code.len() {
        let opcode = code[pc];//u8 type
        helper::print_type_of(&opcode);
        let opc = opcode;
        println!("value of opcode is {opcode}");
        
        println!("value of pc is {pc}");
        if opcode == 0x00 {
            // STOP
            pc += 1;
            break;
        }
        if opcode == 0x5f {
            //to convert an integer to byte format 
            //let vali: u32 = 0;
            //let val = hex::encode(vali.to_be_bytes());
            //stack.push(U256::from(0));
            //not using this push as inert to the end of the vector but due to def in evm.json we need to insert at start
            helper::push_to_stack(&mut stack, U256::from(0));
            pc += 1;
        }
        let push_opcodes : [u8;32] = [0x60,0x61,0x62,0x63,0x64,0x65,0x66,0x67,0x68,0x69,0x6a,0x6b,0x6c,0x6d,0x6e,0x6f,0x70,0x71,0x72,0x73,0x74,0x75,0x76,0x77,0x78,0x79,0x7a,0x7b,0x7c,0x7d,0x7e,0x7f];
        //The contains method in Rust is used to check if a slice (or any type that can be converted to a slice, like an array or a vector) contains a specific element. 
        if opcode == 0x60 {
            println!("{opcode}");
            pc +=1;
            helper::index_rem_push(opcode.into(),&mut stack,&code,&mut pc);
        }
        if opcode == 0x61 {
            pc +=1;
            helper::index_rem_push(opcode.into(),&mut stack,&code,&mut pc);
            //as the opcodes run one by one 
        }
        if opcode == 0x62{
            pc +=1;
            helper::index_rem_push(opcode.into(),&mut stack,&code,&mut pc);
        }
        if opcode == 0x63 {
            pc +=1;
            helper::index_rem_push(opcode.into(),&mut stack,&code,&mut pc);
            
        }
        //to stop a running terminal while running these steps press ctrl +c 
        if opcode == 0x65 {
            pc +=1;
            helper::index_rem_push(opcode.into(),&mut stack,&code,&mut pc);
            
            println!("value of pc is {pc}");
        }
        
        if opcode == 0x69 {
            pc +=1;
            helper::index_rem_push(opcode.into(),&mut stack,&code,&mut pc);
            
            println!("value of pc is {pc}");
        }
        if opcode == 0x6a{
            pc +=1;
            helper::index_rem_push(opcode.into(),&mut stack,&code,&mut pc);
            
            println!("value of pc is {pc}");
        }
        if opcode == 0x7f{
            pc +=1;
            helper::index_rem_push(opcode.into(),&mut stack,&code,&mut pc);
            
            println!("value of pc is {pc}");
            //NOTE 
            //CHNAGED THE ORDER OF THE STACK FOR THIS ONE IN EVM.JSON AS GIVENN WRONG ORDER ACCORDING TO PUSH AND POP FUNCTIONS DEFINED IN RUST FOR VECTORS 
        }
        if opcode == 0x50 {
            stack.remove(0);
            pc += 1;
        }
        if opcode == 0x1 {
            pc +=1;
            let (num1,num2) = helper::pop2(&mut stack);
            //let num3 = manageoverflowadd(num1,num2);
            let (num3,overflow_status) = num1.overflowing_add(num2);
            //so here we can use any of the 2 methods, one is using the already defined functions in rust and the other is defining the functions on our own 
            helper::push_to_stack(&mut stack,num3);
        }
        if opcode == 0x2 {
            pc +=1;
            let (num1,num2) = helper::pop2(&mut stack);
            //here multiplication is preformed as if the value goes goes of bound then it is wrapped around 
            //let maxval = U256::MAX;
            //let num3 = manageoverflowmul(num1,num2);
            //let num3 = num1.wrapping_mul(num2);
            //stack.push(num3);
            let (num3, overflow_status) = num1.overflowing_mul(num2);
            //To implement a function that multiplies two U256 values and wraps on overflow, you can use the overflowing_mul method provided by the primitive_types::U256 type. 
            helper::push_to_stack(&mut stack,num3);
        }
        if opcode == 0x3 {
            pc += 1;
            let (num1,num2) = helper::pop2(&mut stack);
            let (num3,underflow_status) = num1.overflowing_sub(num2);
            helper::push_to_stack(&mut stack,num3);
        }
        if opcode == 0x4 {
            pc += 1;
            let (num1,num2) = helper::pop2(&mut stack);
            if num2 == U256::from(0) {
                helper::push_to_stack(&mut stack,U256::from(0));
            }
            else {
                let num3 = num1/num2;
                helper::push_to_stack(&mut stack,num3);
            }

        }
        if opcode == 0x6 {
            pc += 1;
            let (num1,num2) = helper::pop2(&mut stack);
            if num2 == U256::from(0) {
                helper::push_to_stack(&mut stack,U256::from(0));
            }
            else {
                let num3 = num1%num2;
                helper::push_to_stack(&mut stack,num3);
            }
        }
        if opcode == 0x8 {
            pc += 1;
            let (num1,num2,num3) = helper::pop3(&mut stack);
            if num3 == U256::from(0) {
                helper::push_to_stack(&mut stack,U256::from(0));
            }
            else {
                let (intermidate, overflow_status) = num1.overflowing_add(num2);
                let result = intermidate%num3;
                helper::push_to_stack(&mut stack,result);
            }
        }
        if opcode == 0x09 {
            pc += 1;
            let (num1,num2,num3) = helper::pop3(&mut stack);
            if num3 == U256::from(0) {
                helper::push_to_stack(&mut stack,U256::from(0));
            } else {
                let r1 = num1 % num3;
                let r2 = num2 % num3;
                //using the properties of modulo congruences, nit do by after multiplication mod as that multiplication is not correct due to wrapping in overflow 
                let (mut rem_total,overflow_status) = r1.overflowing_mul(r2);
                rem_total = rem_total%num3;
                helper::push_to_stack(&mut stack,rem_total);
            }
            
        }
        if opcode == 0xa {
            pc += 1;
            let (num1,num2) = helper::pop2(&mut stack);
            let num3 = num1.pow(num2);
            helper::push_to_stack(&mut stack,num3);
        }
        if opcode == 0xb {
            //here still the number is present as U256 type so we need to to sign extention to ensure that it is correct 
            //sor positive numbers we have to put all remaining bits as 0 but this is allready there so no chnage is required 
            //in negative we have to put 1 to all places 
            //sice all inputs are u256 type so we can use bit masking to get a value at some index
            pc +=1;
            let (num1,num2) = helper::pop2(&mut stack);
            let indexbit = helper::index_bit(num1);
            let v = num2;
            let smaller_num1 : u64 = num1.low_u64();
            //for loop only takes normal integer values till u64 so we have to convert u256 to u64, also in this function if the size dont match then can cause data loss 
            let mask_bits = smaller_num1 + 1;
            let mask_lenght =  32 - mask_bits;
            let shift_num = v >> indexbit;
            if shift_num == U256::from(0) {
                helper::push_to_stack(&mut stack,num2);
            }
            else {
                let mut bitmask : U256 = U256::from(0);
                for i in 0..mask_lenght{
                    bitmask = (bitmask) + 0xff;
                    bitmask = bitmask << mask_bits*8;
                }
                let resultant = bitmask + num2;
                println!("result is {resultant}");
                helper::push_to_stack(&mut stack, resultant);
            }


        }
        if opcode ==0x5 {
            pc+=1;
            let (num1, num2) = helper::pop2(&mut stack);
            if num2 == U256::from(0) {
                helper::push_to_stack(&mut stack , U256::from(0))
            }
            else {
                if helper::checksign(num1) ^ helper::checksign(num2){
                
                    //if both of the different sign then answer is negative
                    let mut v1 = num1;
                    let mut v2 = num2;
                    
                    if !helper::checksign(num1){
                        v1 = helper::neg_pov(num1);
                        //if v1 was negative
                    }
                    else {
    
                        v2 = helper::neg_pov(num2);
                        //if v2 was negative 
                    }
                    
                    let mut result = v1 / v2 ;//this is positive 
                    result = helper::neg_pov(result);
                    helper::push_to_stack(&mut stack, result);
                }
                else {
                    //both of same sign then ans is positive
                    let mut v1 = num1;
                    let mut v2 = num2; 
                    if !helper::checksign(num1){
                        v1 = helper::neg_pov(num1);
                        v2 =  helper::neg_pov(num2);
                    }
                    let result = v1 / v2 ;//this is positive 
                    helper::push_to_stack(&mut stack, result);
                    
                }
            }
            
        }
        if opcode == 0x7 {
            pc +=1;
            let (num1,num2) = helper::pop2(&mut stack);
            if num2 == U256::from(0) {
                helper::push_to_stack(&mut stack , U256::from(0))
            }
            else {
                if helper::checksign(num1) ^ helper::checksign(num2){
                    //if both of the different sign
                    let mut v1 = num1;
                    let mut v2 = num2;
                    if !helper::checksign(num1){
                        v1 = helper::neg_pov(num1);
                        //if num1 was negative
                        let mut result = v1%v2;
                        result = helper::neg_pov(result);
                        helper::push_to_stack(&mut stack, result);
                    }
                    else {
                        v2 = helper::neg_pov(num2);
                        let mut result = v1%v2;
                        helper::push_to_stack(&mut stack, result);
                        //if v2 was negative 
                    }
                }
                else {
                    let mut v1 = num1;
                    let mut v2 = num2; 
                    if !helper::checksign(num1){
                        //num1 is negative 
                        v1 = helper::neg_pov(num1);
                        v2 = helper::neg_pov(num2);
                        let mut result = v1%v2;
                        result = helper::neg_pov(result);
                        helper::push_to_stack(&mut stack, result);
                        
                    }
                    else {
                        let mut result = v1%v2;
                        //both are positive 
                        helper::push_to_stack(&mut stack, result);
                    }
                }
            }
            
        }
        if opcode == 0x10 {
            pc +=1;
            let (num1,num2) = helper::pop2(&mut stack);
            let num3 = num1 < num2;
            if num3 {
                helper::push_to_stack(&mut stack , U256::from(1));
            }
            else {
                helper::push_to_stack(&mut stack , U256::from(0));
            }
        }
        if opcode == 0x11 {
            pc += 1;
            let (num1,num2) = helper::pop2(&mut stack);
            let num3 = num1 > num2;
            if num3 {
                helper::push_to_stack(&mut stack , U256::from(1));
            }
            else {
                helper::push_to_stack(&mut stack , U256::from(0));
            }            
        }
        if opcode == 0x12 {
            pc +=1;
            let (num1,num2) = helper::pop2(&mut stack);
            let num3 = helper::signed_comparison_greater(num1, num2);
            if num3 == num2 && num2 != num1{
                helper::push_to_stack(&mut stack, U256::from(1));
            }
            else {
                helper::push_to_stack(&mut stack, U256::from(0));
            }
        }
        if opcode == 0x13 {
            pc +=1;
            let (num1,num2) = helper::pop2(&mut stack);
            let num3 = helper::signed_comparison_greater(num1, num2);
            if num3 == num1 && num2 != num1{
                helper::push_to_stack(&mut stack, U256::from(1));
            }
            else {
                helper::push_to_stack(&mut stack, U256::from(0));
            }
        }
        if opcode == 0x14 {
            pc += 1;
            let (num1,num2) = helper::pop2(&mut stack);
            if num2 == num1{
                helper::push_to_stack(&mut stack, U256::from(1));
            }
            else {
                helper::push_to_stack(&mut stack, U256::from(0));
            }
        }
        if opcode == 0x15 {
            pc += 1;
            let num1 = stack.remove(0);
            if num1 == U256::from(0){
                helper::push_to_stack(&mut stack,  U256::from(1));
            }
            else {
                helper::push_to_stack(&mut stack,  U256::from(0))
            }
        }
        if opcode == 0x19 {
            pc += 1;
            let num1 = stack.remove(0);
            let num2 = num1 ^ U256::MAX;
            helper::push_to_stack(&mut stack, num2);
        }
        if opcode == 0x16 {
            pc += 1;
            let (num1,num2) = helper::pop2(&mut stack);
            let num3 = num1 & num2;
            //&& ar for logical operator 
            //& is for bitwise and, similarly in case of or |
            //helper::print_type_of(&num3);
            helper::push_to_stack(&mut stack , num3);

        }
        if opcode == 0x17 {
            pc += 1;
            let (num1,num2) = helper::pop2(&mut stack);
            let num3 = num1 | num2;
            helper::push_to_stack(&mut stack , num3);

        }
        if opcode == 0x18 {
            pc += 1;
            let (num1,num2) = helper::pop2(&mut stack);
            let num3 = num1 ^ num2;
            helper::push_to_stack(&mut stack , num3);
        }
        if opcode == 0x1b {
            pc += 1;
            let (num1,num2) = helper::pop2(&mut stack);
            let num3 = num2 << num1;
            helper::push_to_stack(&mut stack , num3);
        }
        if opcode == 0x1c {
            pc += 1;
            let (num1,num2) = helper::pop2(&mut stack);
            let num3 = num2 >> num1;
            helper::push_to_stack(&mut stack , num3);
        }
        if opcode == 0x1d {
            pc += 1;
            let (num1,num2) = helper::pop2(&mut stack);
            //here the code can enter a very big loop if not restrict to 256 bit as after 256 bit shift the answer is not going to change
            //if let it enter the loop then program will stop
            if num1 > U256::from(0) && num1 < U256::from(256){
                if helper::checksign(num2){
                    //num2 is positive 
                    let num3 = num2 >> num1;
                    helper::push_to_stack(&mut stack , num3);
                }
                else {
                    //num2 is negative 
                    //the signed bit or the most significant bit is always the last bit so we have to add 1's there only 
                    let size = helper::find_size(num2);
                    let mask = helper::create_bitmask_of_1(num1,size);
                    let pov_shift = num2 >> num1;
                    let num3 = pov_shift + mask;
                    helper::push_to_stack(&mut stack, num3);
                }

            }
            else {
                if helper::checksign(num2){
                    //num2 is pov 
                    helper::push_to_stack(&mut stack, U256::from(0));
                }
                else {
                    helper::push_to_stack(&mut stack, U256::MAX);
                }
            }
        }
        if opcode == 0x1a {
            pc += 1;
            let (num1,num2) = helper::pop2(&mut stack);
            if num1 < U256::from(32){
                let mut byte_represenation = [0u8; 32];
                num2.to_big_endian(&mut byte_represenation);
                let smaller_num1 : usize = num1.low_u64()as usize ; //as for indexing use usize
                let result = byte_represenation[smaller_num1];
                let final_result = U256::from(result);
                helper::push_to_stack(&mut stack, final_result); 
            }
            else {
                helper::push_to_stack(&mut stack, U256::from(0));
            }
            
        }
        
        if opcode == 0x80{
            pc = pc + code.len();
            stack.insert(0,U256::from(0));
        }
        
        
    }

    // TODO: Implement me

    return EvmResult {
        stack: stack,
        success: true,
    };
}

