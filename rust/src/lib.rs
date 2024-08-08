use primitive_types::U256;
use tiny_keccak::{Hasher, Keccak};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
mod helper;

// info on U256 and its implementation
//The U256 type is typically defined as a struct containing multiple smaller integer types (e.g., u64) to represent a 256-bit integer:pub struct U256([u64; 4]);
// we can also preform bitwise multiplications, there are functions in rust which help us do it

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
    pub logs: Log,
    //pub ret: U256,
    pub ret: Vec<u8>,
}
#[derive(Debug,Serialize, Deserialize)]
pub struct LogTest {
    #[serde(default = "default_log_str")]
    pub address : String,
    #[serde(default = "default_log_str")]
    pub data : String,
    #[serde(default = "default_log_vec")]
    pub topics : Vec<String>,
}
pub fn default_stack() -> Option<Vec<String>>{
    Some(Vec::new())
}
pub struct Log {
    pub address : String,
    pub data : U256,
    pub topics : Vec<U256>,
}
//these default values for log will be used when the expect object in created in the main 
pub fn default_log_str() -> String{
    String::from("")
}
pub fn default_log_vec() -> Vec<String>{
    Vec::new()
}
pub fn default_logs() -> Option<Vec<LogTest>> {
    Some(Vec::new())
}
#[derive(Debug,Serialize, Deserialize)]
pub struct Block {
    #[serde(default = "default_block_data_internal_data")]//this attribute is used to assign default values to fileds of a struct if their value is not given 
    pub basefee: String,
    //as serde will also go and compare the parameters inside and see if their value is present and if not then assign a default value if specified 
    #[serde(default = "default_block_data_internal_data")]
    pub coinbase : String,
    #[serde(default = "default_block_data_internal_data")]
    pub timestamp : String,
    #[serde(default = "default_block_data_internal_data")]
    pub number : String,
    #[serde(default = "default_block_data_internal_data")]
    pub difficulty : String,
    #[serde(default = "default_block_data_internal_data")]
    pub gaslimit : String,
    #[serde(default = "default_block_data_internal_data")]
    pub chainid : String,
}
pub fn default_block_data() -> Block{
    Block{
        basefee : String::from("0x00".to_string()),
        coinbase : String::from("0x00".to_string()),
        timestamp : String::from("0x00".to_string()),
        number : String::from("0x00".to_string()),
        difficulty : String::from("0x00".to_string()),
        gaslimit : String::from("0x00".to_string()),
        chainid : String::from("0x00".to_string()),
    }
}
pub fn default_block_data_internal_data() -> String{
    String::from("0x00".to_string())
}
pub fn default_tx_data_internal_data_data() -> String{
    String::from("".to_string())//here give an empty string as the function we are using in the opcode calldatasize is hex decode and in case of the data field the input is a hex string and not a hex number stored as a string for the json file 
}
#[derive(Debug,Serialize, Deserialize)]
pub struct Tx {
    #[serde(default = "default_tx_data_internal_data")]//this attribute is used to assign default values to fileds of a struct if their value is not given 
    pub to: String,
    //as serde will also go and compare the parameters inside and see if their value is present and if not then assign a default value if specified 
    #[serde(default = "default_tx_data_internal_data")]
    pub from: String,
    #[serde(default = "default_tx_data_internal_data")]
    pub origin : String,
    #[serde(default = "default_tx_data_internal_data")]
    pub gasprice : String,
    #[serde(default = "default_tx_data_internal_data")]
    pub value : String,
    #[serde(default = "default_tx_data_internal_data_data")]
    pub data : String,
}
pub fn default_tx_data() -> Tx{
    Tx{
        to : String::from("".to_string()),
        from : String::from("".to_string()),
        origin : String::from("".to_string()),
        gasprice : String::from("".to_string()),
        value : String::from("".to_string()),
        data : String::from("".to_string()),
    }
}

pub fn default_tx_data_internal_data() -> String{
    String::from("".to_string())
}
//Serde also handles nested structures. For example, the state field in Evmtest is a HashMap<String, State_Account_Data>, and Serde will recursively deserialize the state object in the JSON into this map.
#[derive(Debug,Serialize, Deserialize)]
pub struct StateAccountData{
    #[serde(default = "default_state_data")]
    pub balance : String,//using hashmap as the key value i.e the address of the account will be given in the push statement 
    #[serde(default = "default_statecode")]
    pub code : Statecode,
    //To handle a null value in a JSON object using the serde_json crate, you need to ensure that the corresponding field in your Rust struct is of type Option<T>. This tells serde_json that the field can be either a value of type T or null.
}
pub fn default_state_data() -> String{
    String::from("0x00".to_string())
}
#[derive(Debug,Serialize, Deserialize)]
pub struct Code {
    pub asm: String,
    pub bin: String,
}
#[derive(Debug,Serialize, Deserialize)]
pub struct Statecode {
    pub asm: Option<String>,
    pub bin: String,
}

pub fn default_statecode_internal() -> String{
  String::from("".to_string())
}
pub fn default_statecode() -> Statecode{
    Statecode{
        asm : Some(String::from("".to_string())),
        bin : String::from("".to_string()),
    }
}

pub fn default_state() -> HashMap<String, StateAccountData>{
    HashMap::new()
}

//pub fn evm(_code: impl AsRef<[u8]>, _tx_to : &Vec<u8>, _tx_from : &Vec<u8>) -> EvmResult



pub fn evm(_code: impl AsRef<[u8]>, _tx_to : &String, _tx_from : &String, _tx_origin : &String, _tx_gasprice  : &String,_tx_value  : &String, _tx_data : &String,_block_basefee : &String, _block_coinbase : &String,_block_timestamp : &String,_block_number : &String,_block_difficulty : &String,_block_gaslimit : &String,_block_chainid : &String, _account_state : &Option<HashMap<String, StateAccountData>>,storage : &mut HashMap<U256,U256>) -> EvmResult {
    let mut stack: Vec<U256> = Vec::new();
    let mut logs_stack: Vec<U256> = Vec::new();
    let mut _data = U256::from(0);
    let mut _address = String::new();
    //let mut return_val = U256::from(0);
    let mut return_val : Vec<u8> = Vec::new();
    let mut call_result : EvmResult = EvmResult{
        stack : Vec::new(),
        success : true,
        logs : Log{
            address : String::new(),
            data : U256::from(0),            
            topics : Vec::new(),
        },
        //ret : U256::from(0),
        ret : Vec::new(),
    };
    let mut storage_changed_slot_list : Vec<U256> = Vec::new();
    let mut strorage_changed_initial_value_list : Vec<U256> = Vec::new();
    let mut pc: usize = 0;
    //let mut stack1 :Vec<String> = Vec::new();
    let mut status: bool = true;
    let mut prev_opcode_stack: Vec<u8> = Vec::new();
    let mut prev_opcode_pcvalue_stack: Vec<u8> = Vec::new();
    //when a new txn starts it initialses the memory from 0
    //this evm function runs a new txn execution
    let mut memory_array: Vec<u8> = vec![0; 0];
    let mut mem_size = 0;
    let mem_ptr: usize = 0;
    let accountaddress = String::from(_tx_to);


    let code = _code.as_ref();
    
    println!("value of code is {:?}", code);
    //let tx_to = _tx_to;
    //let tx_from = _tx_from;
    let mut valid_opcodes: Vec<u8> = (0..=255).collect();

    // Define the ranges and individual numbers to remove
    let to_remove = [
        12..=15,
        30..=31,
        33..=47,
        75..=79,
        165..=240,
        246..=249,
        251..=252,
    ];

    // Remove the specified numbers
    valid_opcodes.retain(|&x| {
        !to_remove.iter().any(|range| range.contains(&x))
    });
    while pc < code.len() {
        let opcode = code[pc]; //u8 type
        helper::print_type_of(&opcode);
        let opc = opcode;
        println!("value of opcode is {opcode}");

        println!("value of pc is {pc}");
        if opcode == 0x00 {
            // STOP
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            break;
        }
        if opcode == 0x5f {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            //to convert an integer to byte format
            //let vali: u32 = 0;
            //let val = hex::encode(vali.to_be_bytes());
            //stack.push(U256::from(0));
            //not using this push as inert to the end of the vector but due to def in evm.json we need to insert at start
            helper::push_to_stack(&mut stack, U256::from(0));
            pc += 1;
        }
        //let push_opcodes : [u8;32] = [0x60,0x61,0x62,0x63,0x64,0x65,0x66,0x67,0x68,0x69,0x6a,0x6b,0x6c,0x6d,0x6e,0x6f,0x70,0x71,0x72,0x73,0x74,0x75,0x76,0x77,0x78,0x79,0x7a,0x7b,0x7c,0x7d,0x7e,0x7f];
        //The contains method in Rust is used to check if a slice (or any type that can be converted to a slice, like an array or a vector) contains a specific element.
        let push_opcodes: [u8; 32] = (96..=127)
            .collect::<Vec<u8>>()
            .try_into()
            .expect("Wrong length");
        if push_opcodes.contains(&opcode) {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            println!("{opcode}");
            pc += 1;
            helper::index_rem_push(opcode.into(), &mut stack, &code, &mut pc);
        }

        if opcode == 0x50 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            stack.remove(0);
            pc += 1;
        }

        //to stop a running terminal while running these steps press ctrl +c
        if opcode == 0x1 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            //let num3 = manageoverflowadd(num1,num2);
            let (num3, overflow_status) = num1.overflowing_add(num2);
            //so here we can use any of the 2 methods, one is using the already defined functions in rust and the other is defining the functions on our own
            helper::push_to_stack(&mut stack, num3);
        }
        if opcode == 0x2 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            //here multiplication is preformed as if the value goes goes of bound then it is wrapped around
            //let maxval = U256::MAX;
            //let num3 = manageoverflowmul(num1,num2);
            //let num3 = num1.wrapping_mul(num2);
            //stack.push(num3);
            let (num3, overflow_status) = num1.overflowing_mul(num2);
            //To implement a function that multiplies two U256 values and wraps on overflow, you can use the overflowing_mul method provided by the primitive_types::U256 type.
            helper::push_to_stack(&mut stack, num3);
        }
        if opcode == 0x3 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            let (num3, underflow_status) = num1.overflowing_sub(num2);
            helper::push_to_stack(&mut stack, num3);
        }
        if opcode == 0x4 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            if num2 == U256::from(0) {
                helper::push_to_stack(&mut stack, U256::from(0));
            } else {
                let num3 = num1 / num2;
                helper::push_to_stack(&mut stack, num3);
            }
        }
        if opcode == 0x6 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            if num2 == U256::from(0) {
                helper::push_to_stack(&mut stack, U256::from(0));
            } else {
                let num3 = num1 % num2;
                helper::push_to_stack(&mut stack, num3);
            }
        }
        if opcode == 0x8 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2, num3) = helper::pop3(&mut stack);
            if num3 == U256::from(0) {
                helper::push_to_stack(&mut stack, U256::from(0));
            } else {
                let (intermidate, overflow_status) = num1.overflowing_add(num2);
                let result = intermidate % num3;
                helper::push_to_stack(&mut stack, result);
            }
        }
        if opcode == 0x09 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2, num3) = helper::pop3(&mut stack);
            if num3 == U256::from(0) {
                helper::push_to_stack(&mut stack, U256::from(0));
            } else {
                let r1 = num1 % num3;
                let r2 = num2 % num3;
                //using the properties of modulo congruences, nit do by after multiplication mod as that multiplication is not correct due to wrapping in overflow
                let (mut rem_total, overflow_status) = r1.overflowing_mul(r2);
                rem_total = rem_total % num3;
                helper::push_to_stack(&mut stack, rem_total);
            }
        }
        if opcode == 0xa {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            let num3 = num1.pow(num2);
            helper::push_to_stack(&mut stack, num3);
        }
        if opcode == 0xb {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            //here still the number is present as U256 type so we need to to sign extention to ensure that it is correct
            //sor positive numbers we have to put all remaining bits as 0 but this is allready there so no chnage is required
            //in negative we have to put 1 to all places
            //sice all inputs are u256 type so we can use bit masking to get a value at some index
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            let indexbit = helper::index_bit(num1);
            let v = num2;
            let smaller_num1: u64 = num1.low_u64();
            //for loop only takes normal integer values till u64 so we have to convert u256 to u64, also in this function if the size dont match then can cause data loss
            let mask_bits = smaller_num1 + 1;
            let mask_lenght = 32 - mask_bits;
            let shift_num = v >> indexbit;
            if shift_num == U256::from(0) {
                helper::push_to_stack(&mut stack, num2);
            } else {
                let mut bitmask: U256 = U256::from(0);
                for i in 0..mask_lenght {
                    bitmask = (bitmask) + 0xff;
                    bitmask = bitmask << mask_bits * 8;
                }
                let resultant = bitmask + num2;
                println!("result is {resultant}");
                helper::push_to_stack(&mut stack, resultant);
            }
        }
        if opcode == 0x5 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            if num2 == U256::from(0) {
                helper::push_to_stack(&mut stack, U256::from(0))
            } else {
                if helper::checksign(num1) ^ helper::checksign(num2) {
                    //if both of the different sign then answer is negative
                    let mut v1 = num1;
                    let mut v2 = num2;

                    if !helper::checksign(num1) {
                        v1 = helper::neg_pov(num1);
                        //if v1 was negative
                    } else {
                        v2 = helper::neg_pov(num2);
                        //if v2 was negative
                    }

                    let mut result = v1 / v2; //this is positive
                    result = helper::neg_pov(result);
                    helper::push_to_stack(&mut stack, result);
                } else {
                    //both of same sign then ans is positive
                    let mut v1 = num1;
                    let mut v2 = num2;
                    if !helper::checksign(num1) {
                        v1 = helper::neg_pov(num1);
                        v2 = helper::neg_pov(num2);
                    }
                    let result = v1 / v2; //this is positive
                    helper::push_to_stack(&mut stack, result);
                }
            }
        }
        if opcode == 0x7 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            if num2 == U256::from(0) {
                helper::push_to_stack(&mut stack, U256::from(0))
            } else {
                if helper::checksign(num1) ^ helper::checksign(num2) {
                    //if both of the different sign
                    let mut v1 = num1;
                    let mut v2 = num2;
                    if !helper::checksign(num1) {
                        v1 = helper::neg_pov(num1);
                        //if num1 was negative
                        let mut result = v1 % v2;
                        result = helper::neg_pov(result);
                        helper::push_to_stack(&mut stack, result);
                    } else {
                        v2 = helper::neg_pov(num2);
                        let mut result = v1 % v2;
                        helper::push_to_stack(&mut stack, result);
                        //if v2 was negative
                    }
                } else {
                    let mut v1 = num1;
                    let mut v2 = num2;
                    if !helper::checksign(num1) {
                        //num1 is negative
                        v1 = helper::neg_pov(num1);
                        v2 = helper::neg_pov(num2);
                        let mut result = v1 % v2;
                        result = helper::neg_pov(result);
                        helper::push_to_stack(&mut stack, result);
                    } else {
                        let mut result = v1 % v2;
                        //both are positive
                        helper::push_to_stack(&mut stack, result);
                    }
                }
            }
        }
        if opcode == 0x10 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            let num3 = num1 < num2;
            if num3 {
                helper::push_to_stack(&mut stack, U256::from(1));
            } else {
                helper::push_to_stack(&mut stack, U256::from(0));
            }
        }
        if opcode == 0x11 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            let num3 = num1 > num2;
            if num3 {
                helper::push_to_stack(&mut stack, U256::from(1));
            } else {
                helper::push_to_stack(&mut stack, U256::from(0));
            }
        }
        if opcode == 0x12 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            let num3 = helper::signed_comparison_greater(num1, num2);
            if num3 == num2 && num2 != num1 {
                helper::push_to_stack(&mut stack, U256::from(1));
            } else {
                helper::push_to_stack(&mut stack, U256::from(0));
            }
        }
        if opcode == 0x13 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            let num3 = helper::signed_comparison_greater(num1, num2);
            if num3 == num1 && num2 != num1 {
                helper::push_to_stack(&mut stack, U256::from(1));
            } else {
                helper::push_to_stack(&mut stack, U256::from(0));
            }
        }
        if opcode == 0x14 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            if num2 == num1 {
                helper::push_to_stack(&mut stack, U256::from(1));
            } else {
                helper::push_to_stack(&mut stack, U256::from(0));
            }
        }
        if opcode == 0x15 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let num1 = stack.remove(0);
            if num1 == U256::from(0) {
                helper::push_to_stack(&mut stack, U256::from(1));
            } else {
                helper::push_to_stack(&mut stack, U256::from(0))
            }
        }
        if opcode == 0x19 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let num1 = stack.remove(0);
            let num2 = num1 ^ U256::MAX;
            helper::push_to_stack(&mut stack, num2);
        }
        if opcode == 0x16 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            let num3 = num1 & num2;
            //&& ar for logical operator
            //& is for bitwise and, similarly in case of or |
            //helper::print_type_of(&num3);
            helper::push_to_stack(&mut stack, num3);
        }
        if opcode == 0x17 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            let num3 = num1 | num2;
            helper::push_to_stack(&mut stack, num3);
        }
        if opcode == 0x18 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            let num3 = num1 ^ num2;
            helper::push_to_stack(&mut stack, num3);
        }
        if opcode == 0x1b {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            let num3 = num2 << num1;
            helper::push_to_stack(&mut stack, num3);
        }
        if opcode == 0x1c {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            let num3 = num2 >> num1;
            helper::push_to_stack(&mut stack, num3);
        }
        if opcode == 0x1d {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            //here the code can enter a very big loop if not restrict to 256 bit as after 256 bit shift the answer is not going to change
            //if let it enter the loop then program will stop
            if num1 > U256::from(0) && num1 < U256::from(256) {
                if helper::checksign(num2) {
                    //num2 is positive
                    let num3 = num2 >> num1;
                    helper::push_to_stack(&mut stack, num3);
                } else {
                    //num2 is negative
                    //the signed bit or the most significant bit is always the last bit so we have to add 1's there only
                    let size = helper::find_size(num2);
                    let mask = helper::create_bitmask_of_1(num1, size);
                    let pov_shift = num2 >> num1;
                    let num3 = pov_shift + mask;
                    helper::push_to_stack(&mut stack, num3);
                }
            } else {
                if helper::checksign(num2) {
                    //num2 is pov
                    helper::push_to_stack(&mut stack, U256::from(0));
                } else {
                    helper::push_to_stack(&mut stack, U256::MAX);
                }
            }
        }
        if opcode == 0x1a {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            if num1 < U256::from(32) {
                let mut byte_represenation = [0u8; 32];
                num2.to_big_endian(&mut byte_represenation);
                let smaller_num1: usize = num1.low_u64() as usize; //as for indexing use usize
                let result = byte_represenation[smaller_num1];
                let final_result = U256::from(result);
                helper::push_to_stack(&mut stack, final_result);
            } else {
                helper::push_to_stack(&mut stack, U256::from(0));
            }
        }
        let dup_opcodes: [u8; 16] = (128..=143)
            .collect::<Vec<u8>>()
            .try_into()
            .expect("Wrong length");
        if dup_opcodes.contains(&opcode) {
            pc += 1;
            let num2 = opcode - 128;
            let index = num2 as usize;
            let num1 = stack[index];
            helper::push_to_stack(&mut stack, num1);
        }

        let swap_opcodes: [u8; 16] = (144..=159)
            .collect::<Vec<u8>>()
            .try_into()
            .expect("Wrong length");
        if swap_opcodes.contains(&opcode) {
            prev_opcode_stack.insert(0, opcode);
            pc += 1;
            let num1 = stack.remove(0);
            let num2 = opcode - 144;
            let index = num2 as usize;
            let intermidate = stack.remove(index);
            stack.insert(0, intermidate);
            stack.insert(index + 1, num1);
        }

        if opcode == 0x58 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            //NOTE
            //Get the value of the program counter prior to the increment corresponding to this instruction
            helper::push_to_stack(&mut stack, pc.into());
            pc += 1;
        }

        if opcode == 0x5a {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            //NOTE
            //dont know what to do here
            pc += 1;
            helper::push_to_stack(&mut stack, U256::MAX);
        }
        //A byte offset refers to the position of a specific byte within a data structure, file, or memory block. It is a way to index or address bytes in a sequential manner, starting from a base address or the beginning of a data structure.
        if opcode == 0x56 {
            //let push_check = helper::find_valid_opcode()
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            //let num1 = stack.remove(0).low_u64() as usize;
            //if num1 < code.len() && code[num1] == 0x5b  {
            //  pc = num1;
            //}
            //else {
            //  status = false;
            //stack.clear();
            //pc = pc + code.len();
            //}
            let invalid_indexes = helper::invalid_jumpdest(&code, &mut pc);
            let num1 = stack.remove(0).low_u64() as usize;
            if num1 < code.len() && code[num1] == 0x5b {
                if invalid_indexes.contains(&num1) {
                    status = false;
                    stack.clear();
                    pc = pc + code.len();
                } else {
                    pc = num1;
                }
            } else {
                status = false;
                stack.clear();
                pc = pc + code.len();
            }
        }
        if opcode == 0x5b {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);
            pc += 1;
        }
        if opcode == 0x57 {
            prev_opcode_stack.insert(0, opcode);
            prev_opcode_pcvalue_stack.insert(0, opcode);

            let invalid_indexes = helper::invalid_jumpdest(&code, &mut pc);
            let num1 = stack.remove(0).low_u64() as usize;
            let num2 = stack.remove(0);

            if num1 < code.len() && code[num1] == 0x5b {
                if invalid_indexes.contains(&num1) {
                    status = false;
                    stack.clear();
                    pc = pc + code.len();
                } else {
                    if num2 == U256::from(0) {
                        pc += 1;
                    } else {
                        pc = num1;
                    }
                }
            } else {
                status = false;
                stack.clear();
                pc = pc + code.len();
            }
        }
        if opcode == 0x52 {
            pc += 1;
            //here the size of the array is always a multiple of 32
            let (num1, num2) = helper::pop2(&mut stack);
            //num1 is the index at which we have to store the value
            //memory_array is the memory vector
            let mut byte_form = [0u8; 32];
            num2.to_big_endian(&mut byte_form);
            let mut v2 = num1.low_u64() as usize;
            if v2 >= memory_array.len() {
                let intermidate_add_0 = v2 - memory_array.len(); //these are len and index so difference of 1 is adjusted
                helper::add0(&mut memory_array, intermidate_add_0);
                let expected_final_size = memory_array.len() + byte_form.len();
                let req_0 = (expected_final_size / 32 + 1) * 32 - expected_final_size;
                for i in byte_form {
                    memory_array.push(i); //pushed as we are now changing the size of the array
                }
                helper::add0(&mut memory_array, req_0);
            } else {
                //the max insert of of 32
                if v2 + byte_form.len() < memory_array.len() {
                    for i in 0..byte_form.len() {
                        memory_array[v2] = byte_form[i];
                        v2 += 1;
                    }
                } else {
                    //always the lenght of memory will be a multiple of 32
                    helper::add0(&mut memory_array, 32);
                    for i in 0..byte_form.len() {
                        memory_array[v2] = byte_form[i];
                        v2 += 1;
                    }
                }
            }

            //let expected_final_size = byte_form.len() + mem_ptr;
            //let req_0 = (expected_final_size/32 + 1)*32;

            //here there is floor div so this step is required
            //if expected_final_size > memory_array.len(){
            //  helper::add0(&mut memory_array,req_0);
            //}
            //for i in 0..byte_form.len(){
            //  memory_array[i] = byte_form[v2];
            //v2 +=1;
            //mem_ptr += 1;
            //}

            //let rem_index = memory_array.len()/32 +1;
            //let add_0_num = rem_index*32 - mem_ptr;
            //mem_ptr is already 1 step ahead
            //helper::add0(&mut memory_array, add_0_num);

            //Yes, the MSTORE opcode in Ethereum's Virtual Machine (EVM) can overwrite any value in memory. The MSTORE opcode is used to write a value to a specific location in memory. If you use MSTORE to write to a memory location that already has data, it will replace the existing value at that location with the new value.
            // memory is a simple byte array, data stored in 32 byte format but can also be done as 1 byte
            //memory is also a data structure just like the stack but it is cleared once the execution of the txn is done
            //in the memory it has a specific format where it maintains the memory pointer, scratch space etc but for that the instructions are given in the bytcode by organinsing the bytecode in the required seqence and we dont have to implement it here from the opcode
            //when the txn execution starts it starts with memory initialsed to 0 and When a transaction ends, the EVM automatically resets the memory state. This means that all the data stored in memory during the transaction is discarded, and the memory is prepared to be zeroed out for the next transaction.
            // here we also have to implemment memory expansion
            //for inserting the value we need the value of the memory conuter and it is strored int the memory and all these values can be accessed by writing opcodes in the bytecode and we dont have to change anything in the opcode defn
        }
        if opcode == 0x51 {
            pc += 1;
            //the size of memory must change even if we just try to access an index
            let num1 = stack.remove(0).low_u64() as usize;
            helper::memory_access(num1, 32, &mut memory_array, &mut stack);
            
            let ans = &memory_array[num1..num1 + 32];
            let mut ans_vec = ans.to_vec();
            let result = helper::bytes_to_u256(ans_vec);
            helper::push_to_stack(&mut stack, result);
        }
        if opcode == 0x53 {
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            let mut byte_form = [0u8; 32];
            num2.to_big_endian(&mut byte_form);
            println!("{:?}", byte_form);
            let mut v1 = num1.low_u64() as usize;
            if v1 >= memory_array.len() {
                let intermidate_add_0 = v1 - memory_array.len(); //these are len and index so difference of 1 is adjusted
                helper::add0(&mut memory_array, intermidate_add_0);
                let expected_final_size = memory_array.len() + byte_form.len();
                let req_0 = (expected_final_size / 32 + 1) * 32 - expected_final_size;

                helper::add0(&mut memory_array, req_0);
                memory_array[v1] = byte_form[31];
                println!("{:?}", memory_array);
            } else {
                memory_array[v1] = byte_form[31];
            }
        }
        if opcode == 0x59 {
            pc += 1;
            let result = memory_array.len();
            helper::push_to_stack(&mut stack, U256::from(result));
            
        }
        if opcode == 0x20 {
            pc += 1;
            let (num1, num2) = helper::pop2(&mut stack);
            //num1 is the position of reading or the index and num2 is the offset
            let v1 = num1.low_u64() as usize; 
            let v2 = num2.low_u64() as usize; 
            helper::memory_access(v1, v2, &mut memory_array, &mut stack);
            let mut hasher = Keccak::v256();
            let ans = &memory_array[v1..v1 + v2];
            hasher.update(ans);
            let mut output = [0u8; 32];
            hasher.finalize(&mut output);
            let result = helper::bytes_to_u256(output.to_vec());
            helper::push_to_stack(&mut stack, result);
            //the hash func takes in a bytes string reference or slice of bytes 
            //this opcode accesses the memory so we have to take care of memory expansion

        }
        if opcode == 0x30 {
            pc +=1;
            let to = _tx_to;
            helper::hex_str_to_u256_push(&to,&mut stack);
        }
        if opcode == 0x33 {
            pc +=1;
            let to = _tx_from;
            helper::hex_str_to_u256_push(&to,&mut stack);
        }
        if opcode == 0x32 {
            pc +=1;
            let to = _tx_origin;
            helper::hex_str_to_u256_push(&to,&mut stack);
        }
        if opcode == 0x3a {
            pc +=1;
            let to = _tx_gasprice;
            helper::hex_str_to_u256_push(&to,&mut stack);
        }
        if opcode == 0x48 {
            pc +=1;
            let to = _block_basefee;
            helper::hex_str_to_u256_push(&to,&mut stack);
        }
        if opcode == 0x41 {
            pc +=1;
            let to = _block_coinbase;
            helper::hex_str_to_u256_push(&to,&mut stack);
        }
        if opcode == 0x42 {
            pc +=1;
            let to = _block_timestamp;
            helper::hex_str_to_u256_push(&to,&mut stack);
        }
        if opcode == 0x43 {
            pc +=1;
            let to = _block_number;
            helper::hex_str_to_u256_push(&to,&mut stack);
        }
        if opcode == 0x44 {
            pc +=1;
            let to = _block_difficulty;
            helper::hex_str_to_u256_push(&to,&mut stack);
        }
        if opcode == 0x45 {
            pc +=1;
            let to = _block_gaslimit;
            helper::hex_str_to_u256_push(&to,&mut stack);
        }
        if opcode == 0x46 {
            pc +=1;
            let to = _block_chainid;
            helper::hex_str_to_u256_push(&to,&mut stack);
        }
        if opcode == 0x40 {
            pc += 1;
            let num1 = stack.remove(0);
            let mut input  = [0u8; 32];
            num1.to_big_endian(&mut input);
            let input_str = &input[0..32];
            let trimmed_hex = _block_number.trim_start_matches("0x");
            // Remove the "0x" prefix if it's there for this function 
            let number_u256 = U256::from_str_radix(trimmed_hex, 16).unwrap();
            if (number_u256 > U256::from(256) && num1 < number_u256 && num1 > number_u256 - U256::from(256)) || (number_u256 < U256::from(256) && num1 < number_u256){    
                let mut hasher = Keccak::v256();
                hasher.update(input_str);
                let mut output = [0u8; 32];
                hasher.finalize(&mut output);
                let result = helper::bytes_to_u256(output.to_vec());
                helper::push_to_stack(&mut stack, result);
            }
            else {
                helper::push_to_stack(&mut stack, U256::from(0));
            }
            
        }
        if opcode == 0x31 {
            pc +=1;
            let num1 = stack.remove(0);
            let address = helper::get_addr(num1);
            let result = match _account_state{
                Some(value) => match value.get(&address) {
                    Some(val) => &val.balance,
                    None => "0x0",
                },
                None => "0x0",
            };
            //let result = match _account_state.get(&extension) {
              //  Some(value) => &value.balance,
               // None => "0x0",//if the hash map does not have the value for this key  
            //};
            
            let ans = U256::from_str_radix(result, 16).unwrap();
            println!("{ans}");

            helper::push_to_stack(&mut stack, ans);

        }
        if opcode == 0x34 {
            pc +=1;
            let to = _tx_value;
            helper::hex_str_to_u256_push(&to,&mut stack);
            
        }
        if opcode == 0x35 {
            pc += 1;
            let num1 = stack.remove(0);
            let offset = num1.low_u64() as usize;
            //let trimmed_data = _tx_data.trim_start_matches('0');
            println!("{_tx_data}");
            let mut bytes_form : Vec<u8> = hex::decode(_tx_data).unwrap();
            //not use the as_bytes method on string as that will convert each character of the tsring to its ascii value but 1 bytes is of 2characters so use the hex::decode() function when trying to convert a hexadecimal string to bytes 

            if offset + 32 >= bytes_form.len(){
                let req_0 = offset+32-bytes_form.len();
                for i in 0..req_0{
                    bytes_form.push(00);
                }
            }
            println!("{:?}",bytes_form);
            let resultant = &bytes_form[offset..offset+32].to_vec();
            println!("{:?}",resultant);
            let result = helper::bytes_to_u256(resultant.to_vec());
            helper::push_to_stack(&mut stack, result);
        }
        if opcode == 0x36 {
            pc +=1;
            let mut bytes_form : Vec<u8> = hex::decode(_tx_data).unwrap();
            let size = bytes_form.len();
            println!("{size}");
            helper::push_to_stack(&mut stack, U256::from(size));
        }
        if opcode == 0x37 {
            pc +=1;
            let mut bytes_form : Vec<u8> = hex::decode(_tx_data).unwrap();
            let (_destoffset , _offset, _size) = helper::pop3(&mut stack);
            let mut destoffset = _destoffset.low_u64() as usize;
            let mut offset = _offset.low_u64() as usize;
            let mut size = _size.low_u64() as usize;
            helper::memory_access(destoffset, 32, &mut memory_array, &mut stack);
            if offset + 32 >= bytes_form.len(){
                let req_0 = offset+32-bytes_form.len();
                for i in 0..req_0{
                    bytes_form.push(00);
                }
            }
            let resultant = &bytes_form[offset..offset+size ].to_vec();
            for i in 0..size {
                memory_array[destoffset] = resultant[i];
                destoffset +=1;
            }
        }
        if opcode == 0x38 {
            pc +=1;
            helper::push_to_stack(&mut stack, U256::from(code.len()));
        }
        if opcode == 0x39 {
            pc +=1;
            let (_destoffset , _offset, _size) = helper::pop3(&mut stack);
            let mut destoffset = _destoffset.low_u64() as usize;
            let mut offset = _offset.low_u64() as usize;
            let mut size = _size.low_u64() as usize;
            let by = code.clone();
            let mut bytes_form = by.to_vec();
            helper::memory_access(destoffset, 32, &mut memory_array, &mut stack);
            if offset + 32 >= bytes_form.len(){
                let req_0 = offset+32-bytes_form.len();
                for i in 0..req_0{
                    bytes_form.push(00);
                }
            }
            let resultant = &bytes_form[offset..offset+size].to_vec();
            for i in 0..size {
                memory_array[destoffset] = resultant[i];
                destoffset +=1;
            }
        }
        if opcode == 0x3b {
            pc +=1;
            let num1 = stack.remove(0);
            let address = helper::get_addr(num1);
            let result = match _account_state{
                Some(value) => match value.get(&address) {
                    Some(val) => &val.code.bin,
                    None => "0x0",
                },
                None => "",
            };
            //let result = match _account_state.get(&extension) {
              //  Some(value) => &value.balance,
               // None => "0x0",//if the hash map does not have the value for this key  
            //};
            let ans = hex::decode(result).unwrap();
            //let ans = U256::from_str_radix(result, 16).unwrap();
            println!("{:?}",ans);

            helper::push_to_stack(&mut stack, U256::from(ans.len()));

        }
        if opcode == 0x3c {
            pc +=1;
            let (addr, _destoffset, _offset, _size) = helper::pop4(&mut stack);
            let address = helper::get_addr(addr);
            let mut destoffset = _destoffset.low_u64() as usize;
            let offset = _offset.low_u64() as usize;
            let size = _size.low_u64() as usize;
            helper::memory_access(destoffset, size, &mut memory_array, &mut stack);
            let result = match _account_state{
                Some(value) => match value.get(&address) {
                    Some(val) => &val.code.bin,
                    None => "0x0",
                },
                None => "",
            };
            let mut ans = hex::decode(result).unwrap();
            println!("{:?}",ans);
            if offset + size > ans.len(){
                for i in 0..(offset + size - ans.len()){
                    ans.push(00);
                }
            }
            for i in offset..offset+size{
                memory_array[destoffset] = ans[i];
                destoffset += 1;
            }

        }
        if opcode == 0x3f {
            pc +=1;
            
            let num1 = stack.remove(0);
            let address = helper::get_addr(num1);
            match _account_state{
                Some(value) => match value.get(&address) {
                    Some(val) => {
                        let result = String::from(&val.code.bin);
                        let mut hasher = Keccak::v256();
                        //let ans = &memory_array[v1..v1 + v2];
                        hasher.update(&hex::decode(result).unwrap());
                        let mut output = [0u8; 32];
                        hasher.finalize(&mut output);
                        let ans = helper::bytes_to_u256(output.to_vec());
                        helper::push_to_stack(&mut stack, ans);
                    },
                    None => helper::push_to_stack(&mut stack, U256::from(0)),
                },
                None => helper::push_to_stack(&mut stack, U256::from(0)),//if want to do noting then return this 
            };
           
        }
        if opcode == 0x47 {
            pc +=1;
            
            match _account_state {
                Some(value) => match value.get(_tx_to){
                    Some(val) => {
                        let result = &val.balance;
                        let bal= U256::from_str_radix(result, 16).unwrap();
                        helper::push_to_stack(&mut stack, bal);
                    }
                    None => (),
                }
                None => (),
            }
        }
        if opcode == 0x55 {
            //NOTE
            //storage is never deleted wwith the txn hence we define it out of the txn while loop 
            //for storage make an array of U256 type as each value is 256 byte 
            //the size of the array is 2^256
            //the evm does not do slot packing it is done by the solidity bytecode with the help of other opcodes 
            //If you take our example above when we run SLOAD on slot 0 we’re going to get the full 32-byte value stored at that location.
            //in sstore also it will overwrite the value at that place 
            pc +=1;
            let (slot, num) = helper::pop2(&mut stack);
            storage_changed_slot_list.push(slot);
            strorage_changed_initial_value_list.push(match storage.get(&slot){
                Some(value) => *value,
                None => U256::from(0),
            });
            storage.insert(slot,num);
        }
        if opcode == 0x54 {
            pc +=1;
            let slot = stack.remove(0);
            match storage.get(&slot) {
                Some(value) => helper::push_to_stack(&mut stack, *value),
                None => helper::push_to_stack(&mut stack, U256::from(0)),
            }
        }
        let log_opcodes: [u8; 4] = (161..=164)
            .collect::<Vec<u8>>()
            .try_into()
            .expect("Wrong length");
        
        if opcode == 0xa0 {
            pc +=1;
            let (_offset,_size) = helper::pop2(&mut stack);
            let offset = _offset.low_u64() as usize;
            let size = _size.low_u64() as usize;
            _address = String::from(_tx_to);
            helper::memory_access(offset,size, &mut memory_array, &mut stack);
            let data_str = &memory_array[offset..offset+size].to_vec();
            
            _data = helper::bytes_to_u256_ref(data_str);
            println!("{status}");
        }
        if log_opcodes.contains(&opcode){
            pc +=1;
            let (_offset,_size) = helper::pop2(&mut stack);
            let offset = _offset.low_u64() as usize;
            let size = _size.low_u64() as usize;
            _address = String::from(_tx_to);
            helper::memory_access(offset,size, &mut memory_array, &mut stack);
            let data_str = &memory_array[offset..offset+size].to_vec();
            _data = helper::bytes_to_u256_ref(data_str);
            let topic_number = (opcode - 160) as usize;
            
            for i in 0..topic_number{
                let topic = stack.remove(0);
                logs_stack.push(topic);
            }
        }
        if opcode == 0xf3 {
            pc +=1;
            let (_offset,_size) = helper::pop2(&mut stack);
            let offset = _offset.low_u64() as usize;
            let size = _size.low_u64() as usize;
            helper::memory_access(offset,size, &mut memory_array, &mut stack);
            //let return_str = &memory_array[offset..offset+size].to_vec();
            //return_val = helper::bytes_to_u256_ref(return_str);
            let return_str = &memory_array[offset..offset+size];
            println!("the return_str is {:?}",return_str);
            return_val = return_str.to_vec();
            println!("the return_val is {:?}",return_val);
        }
        if opcode == 0xfd {
            //this will stop the txn execution i.e. break the while loop 
            //this will also revert all the state changes made to the storage 
            let mut index = 0;
            
            for i in storage_changed_slot_list{
                storage.insert(i,strorage_changed_initial_value_list[index]);
                index +=1;
            }
            status = false;
            let (_offset,_size) = helper::pop2(&mut stack);
            let offset = _offset.low_u64() as usize;
            let size = _size.low_u64() as usize;
            helper::memory_access(offset,size, &mut memory_array, &mut stack);
            //let return_str = &memory_array[offset..offset+size].to_vec();
            //return_val = helper::bytes_to_u256_ref(return_str);
            let return_str = &memory_array[offset..offset+size];
            return_val = return_str.to_vec();
            break;

        }
        if opcode == 0xf1 {
            pc +=1;
            let (gas, addr,value,_argsoffset,_argssize, _retoffset,_retsize) = helper::pop7(&mut stack);
            let argsoffset = _argsoffset.low_u64() as usize;
            let argssize = _argssize.low_u64() as usize;
            let retoffset = _retoffset.low_u64() as usize;
            let retsize = _retsize.low_u64() as usize;
            let address = helper::get_addr(addr);
            let defulatstateacc = StateAccountData {
                balance: String::from(""),
                code: Statecode {
                    asm: Some(String::from("")),
                    bin: String::from(""),
                },
            };
            println!("the address is: {address}");
            let callobj = match _account_state {
                Some(ref value) => value.get(&address).unwrap_or(&defulatstateacc),
                None => &defulatstateacc,
            };
            
            let callcode : Vec<u8> = hex::decode(&callobj.code.bin).unwrap();
            println!("the value if callcode is {:?}",callcode);
            
            //let callobj = match _account_state{
              //  Some(value) => match value.get(&address) {
                //    Some(val) => val,
                  //  None => &StateAccountData{
                    //    balance : String::from(""),
                      //  code : Statecode{
                        //    asm : Some(String::from("")),
                          //  bin : String::from(""),
                        //},
                    //},
                //},
                //None => &StateAccountData{
                  //  balance : String::from(""),
                    //code : Statecode{
                      //  asm : Some(String::from("")),
                        //bin : String::from(""),
                    //},
                
            //},
        //};
            //let callcode = &callobj.code.bin;
            
            let val = helper::get_addr(value);
            helper::memory_access(argsoffset,argssize,&mut memory_array, &mut stack);
            println!("the memory is {:?}",memory_array);
            let call_calldata = &memory_array[argsoffset..argsoffset+argssize];
            println!("the calldata_array_slice is {:?}",call_calldata);
            let call_calldata_vec = call_calldata.to_vec();
            println!("the calldata_vec is {:?}",call_calldata_vec);
            let calldata = helper::bytes_to_str(call_calldata_vec);
            println!("the calldata is {:?}",calldata);
            
            call_result = evm(callcode,&address, _tx_to, _tx_from, _tx_gasprice,&val, &calldata, _block_basefee,_block_coinbase, _block_timestamp, _block_number, _block_difficulty, _block_gaslimit, _block_chainid, _account_state, storage);
            //pub fn evm(_code: impl AsRef<[u8]>, _tx_to : &String, _tx_from : &String, _tx_origin : &String, _tx_gasprice  : &String,_tx_value  : &String, _tx_data : &String,_block_basefee : &String, _block_coinbase : &String,_block_timestamp : &String,_block_number : &String,_block_difficulty : &String,_block_gaslimit : &String,_block_chainid : &String, _account_state : &Option<HashMap<String, StateAccountData>>,storage : &mut HashMap<U256,U256>) -> EvmResult
            helper::memory_access(retoffset,retsize ,&mut memory_array, &mut stack);
            //let mut byte_form = [0u8; 32];
            //call_result.ret.to_big_endian(&mut byte_form);
            
            println!("the returned value is : {:?}",call_result.ret);
            let mut index = retoffset; 
            for i in 0..retsize{
                
                memory_array[index] = call_result.ret[i];
                index += 1;
                //here write to specific bytes as mload will load all the 32 bytes together 
                //if call the caller in the sub context of the other contract then it will be main contract tx_to and the other contract tx_from
            }
            if call_result.success {
                helper::push_to_stack(&mut stack, U256::from(1));
            }
            else {
                helper::push_to_stack(&mut stack, U256::from(0));
            }
            
        }
        if opcode == 0x3d{
            pc = pc + code.len();
            helper::push_to_stack(&mut stack, U256::from(0));
        }

        

        //invalid opcode
        if opcode == 0xfe {
            status = false;
            break;
        }
        if !valid_opcodes.contains(&opcode) {
            status = false;
            println!("status switched to false as not a vaid opcode");
            break;
        }
    }

    // TODO: Implement me
    let logs_struct = Log{
        address: _address,
        data : _data,
        topics : logs_stack,
    };
    println!("status of success is : {status}");
    println!("status of return_val is : {:?}",return_val);
    return EvmResult {
        stack: stack,
        success: status,
        logs: logs_struct,
        ret : return_val,
    };
}
