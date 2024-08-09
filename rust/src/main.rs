/**
 * EVM From Scratch
 * Rust template
 *
 * To work on EVM From Scratch in Rust:
 *
 * - Install Rust: https://www.rust-lang.org/tools/install
 * - Edit `rust/lib.rs`
 * - Run `cd rust && cargo run` to run the tests
 *
 * Hint: most people who were trying to learn Rust and EVM at the same
 * gave up and switched to JavaScript, Python, or Go. If you are new
 * to Rust, implement EVM in another programming language first.
 */
// if we initialise a project in rust using cargo new projectname but then chnage the name of the project from outside without chnaging the name in the package section of cargo.toml then it will not cause  problem, here the cargo was initialsed using evm package name but then it was changed to rust
//here evm is refering to ur package name only
//when we create a new package here evm then the files lib.rs and main.rs are named actually after the package name, so to use some function of library crate we have to write this statement of use evm::evm, as evm is the package name so lib.rs name is also evm, the func name is also evm
use evm::{evm, Block, default_block_data_internal_data, default_block_data, Tx, default_tx_data, default_tx_data_internal_data, default_state,default_state_data,Code,default_statecode,Statecode, StateAccountData, Log, default_log_str, default_log_vec,default_logs,LogTest,default_stack};
use primitive_types::U256; //imported crate as dependency
use serde::{Serialize, Deserialize}; //imported crate as dependency
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Evmtest {
    name: String,
    hint: String,
    code: Code,
    #[serde(default = "default_tx_data")]
    tx: Tx,//the filed name should be same to that in the json object as here string matching is done 
    #[serde(default = "default_block_data")]
    block : Block,
    
    state : Option<HashMap<String, StateAccountData>>,
    expect: Expect,
}







#[derive(Debug, Serialize, Deserialize)]
struct Expect {
    #[serde(default = "default_stack")]
    stack: Option<Vec<String>>,
    
    #[serde(default = "default_logs")]
    logs : Option<Vec<LogTest>>,
    //made it a vector as in the serde match the [] brackets were creating a problem and considereing it as a mapping 
    success: bool,
    
    #[serde(rename = "return")]
    ret: Option<String>,
    //This attribute changes the name of the field in the serialized or deserialized data, without changing the name of the field in your Rust struct.
    //using option as this field might not be available everywhere
}

fn main() {
//STORAGE
//GLOBAL VARIABLE 
//let can only be used to declare variables inside a function 
let mut storage : HashMap<U256,U256> = HashMap::new();
//here we are using hashmaps and nit vector as in hashmaps we are not restricted to limited storage slots which is the max value of usize and also we are able to implement the key value pairs and the empty storage slots that is no defined key values are empty and using match can be set to 0 and wont require storage place like in the actual evm
//this is seperate for each contract so define this in the main.rs which interacts with the bytecode of the entire contract not only a transaction of that contract as in case of memory 
//U256 is 2 bytes
    let text = std::fs::read_to_string("../evm.json").unwrap();
    
    let data: Vec<Evmtest> = serde_json::from_str(&text).unwrap();
    //the serde json is smart enough to automatically put the given data in the form of a string in json format to the types specified by us 
    //so here when some alue is null we have to assign it to an option enum and then handle it 

    let total = data.len();

    for (index, test) in data.iter().enumerate() {
        println!("Test {} of {}: {}", index + 1, total, test.name);
        println!("{:?}", test);

        let code: Vec<u8> = hex::decode(&test.code.bin).unwrap();
        let tx_to = &test.tx.to;
        let to_from = &test.tx.from;//these are already references so in evm func no need of &
        //println!("{:?}", tx_to);
        //println!("{:?}", tx_from);
        let tx_origin = &test.tx.origin;
        let tx_value = &test.tx.value;
        let tx_data = &test.tx.data;
        let tx_gasprice = &test.tx.gasprice;
        let block_basefee = &test.block.basefee;
        let block_coinbase  = &test.block.coinbase;
        let block_timestamp = &test.block.timestamp;
        let block_number = &test.block.number;
        let block_difficulty = &test.block.difficulty;
        let block_gaslimit = &test.block.gaslimit;
        let block_chainid  = &test.block.chainid;
        let account_state = &test.state;


        let result = evm(&code,tx_to, to_from, tx_origin, tx_gasprice,tx_value, tx_data, block_basefee,block_coinbase, block_timestamp, block_number, block_difficulty, block_gaslimit, block_chainid, account_state, &mut storage);
        println!("{:?}",&test.expect);
        
        //testing 
        //in testing everythinh is same for logs as in stack except where we are accessing test first 
        //all this for 1 test only 
        let mut expected_stack: Vec<U256> = Vec::new();
        let mut expected_log_stack: Vec<U256> = Vec::new();
        if let Some(ref stacks) = test.expect.stack {
            for value in stacks {
                expected_stack.push(U256::from_str_radix(value, 16).unwrap());
            }
        }
        let mut expected_data = String::new();
        let mut expected_address = String::new();
        let mut actual_data = U256::from(0);
        let mut actual_address = String::new();
        let mut expected_data_u256 = U256::from(0);
        let mut expected_return_vec = Vec::new();
        if let Some(ref log_stacks) = test.expect.logs {
            //this is a stack/vector of LogTest
            if log_stacks.len() > 0 {
                let index = log_stacks[0].topics.len();
                if index > 0 {
                    for i in 0..index {
                        //as logs is a option which contains a vector due to the sytax in evm.json 
                        let value = &log_stacks[0].topics[i];
                        expected_log_stack.push(U256::from_str_radix(value, 16).unwrap());
                    }
                }
                
                //expected_data_str = hex::decode(log_stacks[0].data.clone()).unwrap();
                expected_data = log_stacks[0].data.clone();
                let mut extension_data = String::from("0x");
                extension_data.push_str(&expected_data);
                expected_data_u256 = U256::from_str_radix(&extension_data, 16).unwrap();
                expected_address = log_stacks[0].address.clone();
                
            }
            
        }
        match &test.expect.ret {
            Some(val) => expected_return_vec = hex::decode(val).unwrap(),
            None => (),
        }
        
        let mut matching_address = result.logs.address == expected_address;
        let mut matching_data = result.logs.data == expected_data_u256;
        
        //actual_data = result.logs.data.clone();
        //actual_address = result.logs.address.clone();
        
        
        let mut matching = result.stack.len() == expected_stack.len();
        let mut matching_return = result.ret.len() == expected_return_vec.len();
        let mut log_matching = result.logs.topics.len() == expected_log_stack.len() ;//result object is not an option type 
        //LogTest is option type 
        if matching {
            for i in 0..result.stack.len() {
                if result.stack[i] != expected_stack[i] {
                    matching = false;
                    break;
                }
            }
        }
        if matching_return {
            for i in 0..result.ret.len() {
                if result.ret[i] != expected_return_vec[i] {
                    matching_return = false;
                    break;
                }
            }
        }
        if log_matching {
            for i in 0..result.logs.topics.len() {
                if result.logs.topics[i] != expected_log_stack[i] {
                    //here both are of type u256
                    log_matching = false;
                    break;
                }
            }
        }
        
        println!("{}",result.success);
        println!("{log_matching}");
        println!("{matching_address}");
        println!("{matching_data}");
        println!("{matching}");
        println!("{matching_return}");
        println!("{:?}",result.ret);
        println!("{:?}",expected_return_vec);
        matching = matching && result.success == test.expect.success && log_matching && matching_address && matching_data && matching_return;//expected_return_vec == result.ret;
        //let actual_log = test.expect.logs.as_ref();
        
        //not use unwrap as it consumes the value i.e. not borrowing
        if !matching {
            println!("Instructions: \n{}\n", test.code.asm);

            println!("Expected success: {:?}", test.expect.success);
            println!("Expected return: {:?}", expected_return_vec);
            println!("Expected stack: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");
            println!("Expected log stack: [");
            for v in expected_log_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");
            println!("Expected address: {:?}", expected_address);
            println!("]\n");
            println!("Expected data: {:?}", expected_data_u256);
            println!("]\n");

            println!("Actual success: {:?}", result.success);
            println!("Actual return: {:?}", result.ret);
            println!("Actual stack: [");
            for v in result.stack {
                println!("  {:#X},", v);
            } //to print each element in hexadecimal format
            println!("]\n");
            
            println!("Actual log stack: [");
            for v in result.logs.topics {
                println!("  {:#X},", v);
            } //to print each element in hexadecimal format
            println!("]\n");
            println!("actual address: {:?}", result.logs.address);
            println!("]\n");
            println!("actual data: {:?}", result.logs.data);
            println!("]\n");

            println!("\nHint: {}\n", test.hint);
            println!("Progress: {}/{}\n\n", index, total);
            panic!("Test failed");
        }
        println!("PASS");
    }
    println!("Congratulations!");
}
