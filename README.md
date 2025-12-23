# Compiler rewrite
There's still no name, but I'll find one eventually.

## Installation
This project will require LLVM 14 to be installed and available in your PATH by the time it is finished.

To install LLVM, you can use:
```bash
sudo apt-get update && sudo apt-get install -y llvm-14 llvm-14-dev libpolly-14-dev
```

You may need to export the LLVM path for llvm-sys:
```bash
export LLVM_SYS_140_PREFIX=/usr/lib/llvm-14
```

## Compilation
N/A

## Future Plans
The lexer is pretty much in its final form and won't be changed too much in future. This is what will be changed in future:
- Add parser
- Add typechecker
- Add compiler
- Polish error handling
