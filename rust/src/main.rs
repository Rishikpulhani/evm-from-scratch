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
use evm::evm;
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
    #[serde(default = "default_state")]
    state : HashMap<String, State_Account_Data>,
    expect: Expect,
}
fn default_tx_data() -> Tx{
    Tx{
        to : String::from("0x00".to_string()),
        from : String::from("0x00".to_string()),
        origin : String::from("0x00".to_string()),
        gasprice : String::from("0x00".to_string()),
    }
}
fn default_block_data() -> Block{
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
fn default_tx_data_internal_data() -> String{
    String::from("0x00".to_string())
}
fn default_block_data_internal_data() -> String{
    String::from("0x00".to_string())
}
fn default_state_data() -> String{
    String::from("0x00".to_string())
}
fn default_state() -> HashMap<String, State_Account_Data>{
    HashMap::new()
}

#[derive(Debug,Serialize, Deserialize)]
struct Code {
    asm: String,
    bin: String,
}
#[derive(Debug,Serialize, Deserialize)]
struct State_Account_Data{
    #[serde(default = "default_state_data")]
    balance : String,//using hashmap as the key value i.e the address of the account will be given in the push statement 
}
#[derive(Debug,Serialize, Deserialize)]
struct Tx {
    #[serde(default = "default_tx_data_internal_data")]//this attribute is used to assign default values to fileds of a struct if their value is not given 
    to: String,
    //as serde will also go and compare the parameters inside and see if their value is present and if not then assign a default value if specified 
    #[serde(default = "default_tx_data_internal_data")]
    from: String,
    #[serde(default = "default_tx_data_internal_data")]
    origin : String,
    #[serde(default = "default_tx_data_internal_data")]
    gasprice : String,
}
#[derive(Debug,Serialize, Deserialize)]
struct Block {
    #[serde(default = "default_block_data_internal_data")]//this attribute is used to assign default values to fileds of a struct if their value is not given 
    basefee: String,
    //as serde will also go and compare the parameters inside and see if their value is present and if not then assign a default value if specified 
    #[serde(default = "default_block_data_internal_data")]
    coinbase : String,
    #[serde(default = "default_block_data_internal_data")]
    timestamp : String,
    #[serde(default = "default_block_data_internal_data")]
    number : String,
    #[serde(default = "default_block_data_internal_data")]
    difficulty : String,
    #[serde(default = "default_block_data_internal_data")]
    gaslimit : String,
    #[serde(default = "default_block_data_internal_data")]
    chainid : String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Expect {
    stack: Option<Vec<String>>,
    success: bool,
    // #[serde(rename = "return")]
    // ret: Option<String>,
}

fn main() {
    let text = std::fs::read_to_string("../evm.json").unwrap();
    let data: Vec<Evmtest> = serde_json::from_str(&text).unwrap();

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
        let tx_gasprice = &test.tx.gasprice;
        let block_basefee = &test.block.basefee;
        let block_coinbase  = &test.block.coinbase;
        let block_timestamp = &test.block.timestamp;
        let block_number = &test.block.number;
        let block_difficulty = &test.block.difficulty;
        let block_gaslimit = &test.block.gaslimit;
        let block_chainid  = &test.block.chainid;
        let account_state = &test.state;


        let result = evm(&code,tx_to, to_from, tx_origin, tx_gasprice, block_basefee,block_coinbase, block_timestamp, block_number, block_difficulty, block_gaslimit, block_chainid, account_state);
        //let result = evm(&code, &tx_to,&tx_from);

        let mut expected_stack: Vec<U256> = Vec::new();
        if let Some(ref stacks) = test.expect.stack {
            for value in stacks {
                expected_stack.push(U256::from_str_radix(value, 16).unwrap());
            }
        }

        let mut matching = result.stack.len() == expected_stack.len();
        if matching {
            for i in 0..result.stack.len() {
                if result.stack[i] != expected_stack[i] {
                    matching = false;
                    break;
                }
            }
        }

        matching = matching && result.success == test.expect.success;

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm);

            println!("Expected success: {:?}", test.expect.success);
            println!("Expected stack: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("Actual success: {:?}", result.success);
            println!("Actual stack: [");
            for v in result.stack {
                println!("  {:#X},", v);
            } //to print each element in hexadecimal format
            println!("]\n");

            println!("\nHint: {}\n", test.hint);
            println!("Progress: {}/{}\n\n", index, total);
            panic!("Test failed");
        }
        println!("PASS");
    }
    println!("Congratulations!");
}
