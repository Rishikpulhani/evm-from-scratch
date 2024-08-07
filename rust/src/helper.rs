use primitive_types::U256;
use std::any::type_name;

pub fn index_rem_push(opcode: usize, stack: &mut Vec<U256>, code: &[u8], pc: &mut usize) {
    let ind_rem = opcode - 95 + 1;
    //println!("the value of ind_rem is {ind_rem}");
    let ind_rem = ind_rem as usize;
    let mut v: U256 = U256::from(0);
    for i in 1..ind_rem {
        //in the for loop like python the last index is not taken
        //let u : usize =  code[i].into();
        //v = v*16 + u;
        let u: usize = code[*pc].into(); // to convert the type from u8 to usize
                                         //this usize is finee as it does not have to store more than 1 byte at once
        println!("value of pc {pc}");
        *pc += 1;
        let w: U256 = U256::from(u);
        v = bytes_shift_by1(v) + w;
        println!("{v}");
    }
    //the max size usize can store is 8 bytes based on 64 bit architecture
    //usize is Primarily used for indexing, memory allocation, and pointer arithmetic, operations that involve sizes or counts of memory-related elements.
    //stack.push(U256::from(v));
    stack.insert(0, U256::from(v));
    //for indexing we can use only usize datatype not u8, so i needs to be usize
}
pub fn bytes_shift_by1(u: U256) -> U256 {
    u << 8
}
//fn terminal_test_fail_helperfn(pc : &usize, stack : &mut Vec<U256>, code : &[u8]){
//  pc = pc + code.len();
//stack.push(U256::from(0));
//}
pub fn test_overflow(n1: U256, n2: U256) -> bool {
    let mut u = U256::MAX;
    println!("{u}");
    u = u / 2;

    let n11 = n1 / 2;
    let n22 = n2 / 2;
    if n11 + n22 > u {
        true
    } else {
        false
    }
}
pub fn div_check(u: U256, d: U256) -> bool {
    if u % d == U256::from(0) {
        true
    } else {
        false
    }
}
pub fn larger_smaller(n1: U256, n2: U256) -> (U256, U256) {
    if n1 > n2 {
        (n1, n2)
    } else {
        (n2, n1)
    }
}
pub fn larger(n1: U256, n2: U256) -> U256 {
    if n1 >= n2 {
        n1
    } else {
        n2
    }
}
pub fn manageoverflowadd(n1: U256, n2: U256) -> U256 {
    let mut v1 = n1;
    let mut v2 = n2;
    let mut c = U256::MAX;

    if v1 > c / 2 {
        v1 = c - v1;
    }
    if v2 > c / 2 {
        v2 = c - v2;
    }
    if v1 != n1 || v2 != n2 {
        v1 + v2 - 1
    } else {
        n1 + n2
    }
}
//pub fn manageoverflowmul(n1 : U256, n2 : U256) -> U256{
//  let (l,s) = larger_smaller(n1,n2);
//let mulval = U256::from(0);
//for i in 0..s{
//  mulval = manageoverflowadd(mulval,l);
//}
//mulval
//}
pub fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}
pub fn pop2(stack: &mut Vec<U256>) -> (U256, U256) {
    let num1 = stack.remove(0);
    let num2 = stack.remove(0);
    (num1, num2)
}
pub fn pop3(stack: &mut Vec<U256>) -> (U256, U256, U256) {
    let num1 = stack.remove(0);
    let num2 = stack.remove(0);
    let num3 = stack.remove(0);
    (num1, num2, num3)
}
pub fn pop4(stack: &mut Vec<U256>) -> (U256, U256, U256, U256) {
    let num1 = stack.remove(0);
    let num2 = stack.remove(0);
    let num3 = stack.remove(0);
    let num4 = stack.remove(0);
    (num1, num2, num3, num4)
}
pub fn push_to_stack(stack: &mut Vec<U256>, element: U256) {
    stack.insert(0, element);
}
pub fn index_bit(n1: U256) -> U256 {
    ((n1 + 1) * 8) - 1
}
pub fn checksign(n1: U256) -> bool {
    let shift_num = n1 >> 255;
    if shift_num == U256::from(0) {
        true
    } else {
        false
    }
}
pub fn neg_pov(n1: U256) -> U256 {
    let m = U256::MAX;
    let result = n1 ^ m;
    println!("{}", result + 1);
    result + 1
}
pub fn signed_comparison_greater(n1: U256, n2: U256) -> U256 {
    if checksign(n1) ^ checksign(n2) {
        //both are of different sign
        if checksign(n2) {
            n2
        } else {
            n1
        }
    } else {
        //both of same sign
        if checksign(n1) {
            //both positive
            if n2 == larger(n1, n2) && n1 != n2 {
                n2
            } else {
                n1
            }
        } else {
            //both negative
            let v1 = neg_pov(n1); //took absolute value
            let v2 = neg_pov(n2);
            let v3 = larger(v1, v2);
            if v3 >= v1 {
                //num1 is bigger
                n1
            } else {
                //num2 is bigger
                n2
            }
        }
    }
}
pub fn find_size(n1: U256) -> U256 {
    let mut size = U256::from(0);
    let mut v1 = n1;
    while v1 != U256::from(0) {
        v1 = v1 >> 4;
        size = size + 4;
    }
    size
}
pub fn create_bitmask_of_1(n1: U256, size: U256) -> U256 {
    let mut mask = U256::from(0);
    let smaller_n1: u64 = n1.low_u64();
    for i in 0..smaller_n1 {
        mask = mask << 1;
        mask = mask + 1;
    }
    let rem_bits = size - n1;
    mask = mask << rem_bits;
    mask
}
pub fn push_multiple_times(n1: U256, n2: U256, stack: &mut Vec<U256>) {
    let smaller_n2: u64 = n2.low_u64();
    for i in 0..smaller_n2 {
        push_to_stack(stack, n1);
    }
}

pub fn invalid_jumpdest(code: &[u8], pc: &mut usize) -> Vec<usize> {
    let bytecode_len = code.len();
    let mut invalid_dest: Vec<usize> = [].to_vec();
    let push_opc_list: [u8; 32] = (96..=127)
        .collect::<Vec<u8>>()
        .try_into()
        .expect("Wrong length");
    let mut i = 0;
    while i < bytecode_len {
        //i is like the pc now
        if push_opc_list.contains(&code[i]) {
            let skip_index = code[i] - 95;
            for j in 0..skip_index {
                i += 1;
                invalid_dest.push(i);
            }
        } else {
            i += 1;
        }
    }
    invalid_dest
    //let num1 = prev_opc_stk[0];//the previos opcode run before this func call
    //let num2 = prev_opc_pcval_stk[0];
    //if push_opc_list.contains(&num1){
    //if it is a push opcode
    //  let following_bytes = num1 - 95;
    //let danger_zone : [u8;following_bytes+1] = (num2..=num2+following_bytes).collect::<vec<u8>>().try_into().expect("wrong lenght");
    //if danger_zone.contains(&(pc))
    //}
}
pub fn add0(mem: &mut Vec<u8>, n1: usize) {
    for i in 0..n1 {
        mem.push(0);
    }
}
pub fn bytes_to_u256(byte_rep: Vec<u8>) -> U256 {
    let mut result: U256 = U256::from(0);
    for i in byte_rep {
        result = (result << 8) + U256::from(i);
    }
    result
}
pub fn bytes_to_u256_ref(byte_rep: &Vec<u8>) -> U256 {
    let mut result: U256 = U256::from(0);
    for i in byte_rep {
        result = (result << 8) + U256::from(*i);
    }
    result
}
pub fn memory_access(num1 : usize,offset : usize, memory_array : &mut Vec<u8>, stack : &mut Vec<U256>) {
    //offset is the size of the memory being accessed 
    //num1 is the index of the place from where the memory will be accessed
    if num1 > memory_array.len() {
        let intermidate_add_0 = num1 - memory_array.len(); //these are len and index so difference of 1 is adjusted
         
        add0(memory_array, intermidate_add_0);
        let req_0 = ((num1 + offset) / 32 + 1) * 32 - memory_array.len();
        add0(memory_array, req_0);
        //since it is already a mutable reference so no need to again specify that here 
    } else {
        if num1 + offset < memory_array.len() {
        } else {
            let mut req_0 = 0;
            if div_check(U256::from(num1 + offset), U256::from(32)) {
                req_0 = ((num1 + offset) / 32) * 32 - memory_array.len();
                //num1+offset is the final byte read as whether to expand memory or not will depend on the last byte read 
            } else {
                req_0 = ((num1 + offset) / 32 + 1) * 32 - memory_array.len();
            }

            add0(memory_array, req_0);
        }
    }
}
pub fn hex_str_to_u256_push (to : &String, stack : &mut Vec<U256>){
    let trimmed_hex = to.trim_start_matches("0x");
    // Remove the "0x" prefix if it's there for this function 
    let result = U256::from_str_radix(trimmed_hex, 16).unwrap();
    push_to_stack(stack, result);
}
pub fn get_addr(addr : U256) -> String{
    let mut addr_bytes = [0u8;32];
    addr.to_big_endian(&mut addr_bytes);
    let addr_str = hex::encode(&addr_bytes);
    let trimmed = addr_str.trim_start_matches('0');
    //u256 -> bytes array -> string 
    let mut address = String::from("0x");
    address.push_str(trimmed);//address of the contract 
    println!("{address}");
    address
}
