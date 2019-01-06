# ckb-binary-to-script

A simple tool to wrap a RISC-V binary in CKB's [script](https://github.com/nervosnetwork/ckb/blob/1da17f000cfc5bd98acb454c278c0c68900d8c4a/core/src/script.rs#L10) structure. This could serve as a temporary solution for languages without FlatBuffers implementation, such as Ruby.

## Usage

```bash
$ git clone https://github.com/nervosnetwork/ckb-binary-to-script
$ cd ckb-binary-to-script
$ cargo build
$ ./target/debug/ckb-binary-to-script < /path/to/RISC-V/binary > path/to/generated/Script/data
```
