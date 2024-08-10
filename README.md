# EVM From Scratch

![EVM From Scratch](.github/logo.png)

Welcome to **EVM From Scratch**! It's a 100% practical course that will help you better understand the inner workings of the Ethereum Virtual Machine. During this course, we'll implement EVM in your favorite programming language.

## Getting Started

Clone the repo:

```sh
git clone https://github.com/w1nt3r-eth/evm-from-scratch
```

This repository contains [`evm.json`](./evm.json) file with more than 100 test cases. Your goal is to create an implementation in any programming language of your choice that passes all tests.

The test cases are organized by complexity: they start with the simplest opcodes and gradually progress to advanced. Each test case has a name, code and expectation. The code is provided as a human-readable instructions list (`asm`) and machine-readable bytecode encoded in hex (`bin`). Your implementation should only look at `bin`, the `asm` is provided to make unit tests easier to debug.

The repository contains templates for JavaScript, TypeScript, Python, Go and Rust. However, you don't have to use the existing templates, you can build your own test suite based on [`evm.json`](./evm.json).

## Credits

All the materials in the repository are made by [w1nt3r.eth](https://twitter.com/w1nt3r_eth). The repository is part of the "EVM From Scratch" course (release date TBD).


## Limitations in the project

1) The actual Theorotical size of the memory iin the EVM is 2^256 - 1 bits long but in in the project the data structure used a vector and a vector cannot have a size more that usize::MAX i.e. 2^64 - 1 bits.
2) The feature of GAS is not yet implemented.
3) The feature of Nonce of a contract is not yet implemented and because of this in the CREATE opcode the address creation algorithm is using the hash of the sender address only instead of the sender address + nonce. the feature of Nonce will be added to the StateAccountData struct.
4) The storage is also to be integrated with the StateAccountData struct as it is seperate for every contract.
5) In the CALL opcode the Storage passed in the evm function should be of the contract being called but it is a new storage which is being passed whose scope is also limited to that opcode only beacuse of the way in which it is defined in the if code block (same of for nonce). Pt4 and 5 are not yet implemented because the tests given in the evm.json don't match the required format as required to add these.
