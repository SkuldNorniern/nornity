---
title: "Releasing Lamina: Building my own Compiler Backend"
excerpt: "The journey of creating and releasing Lamina, a modern compiler backend that competes with established systems languages in performance benchmarks."
author: "Eira"
tags: ["lamina", "compiler", "rust", "performance"]
published_at: "2025-09-09 14:33:57"
draft: false
---

# Releasing Lamina: Building my own Compiler Backend

After months of development, I'm excited to announce the release of [Lamina](https://github.com/SkuldNorniern/lamina), [Lamina-Crate](https://crates.io/crates/lamina), a modern compiler backend for the Lamina Intermediate Representation. This project represents a significant milestone in my journey to understand compiler construction and create a modern alternative to established compiler backends like LLVM.

## The Vision

The idea for Lamina started from a simple question:
>  what would a modern compiler backend look like if designed from scratch today? While LLVM has been revolutionary, I wanted to explore how we could build something that leverages modern Rust practices, provides a cleaner API, and delivers competitive performance.

Lamina isn't meant to replace LLVM, it's an exploration of compiler backend design principles and a learning project to understand how compilers work.
while an attempt to make my pre-existing Language projects to use Lamina as their backend.

## What Makes Lamina Different

### Clean IRBuilder API

One of the most satisfying aspects of Lamina is its IRBuilder API. Creating compiler frontends becomes straightforward:

```rust
use lamina::ir::*;

fn create_add_function() -> Result<Module, Error> {
    let mut builder = IRBuilder::new();
    let module = builder.create_module("math_example");
    
    let params = vec![
        FunctionParameter { ty: Type::I32, name: "a".to_string() },
        FunctionParameter { ty: Type::I32, name: "b".to_string() },
    ];
    
    let function = builder.create_function("add", params, Type::I32);
    let entry_block = builder.create_block("entry");
    builder.set_current_block(entry_block);
    
    let param_a = builder.get_parameter(0);
    let param_b = builder.get_parameter(1);
    
    let result = builder.add_instruction(
        Instruction::BinaryOp {
            op: BinaryOperator::Add,
            ty: Type::I32,
            left: param_a,
            right: param_b,
        }
    );
    
    builder.add_return(result);
    Ok(module)
}
```

This API makes it easy to build compilers for other languages by providing a clean abstraction over the intermediate representation.

### Performance That Matters

The most surprising aspect of this project was the performance results. In our comprehensive 256×256 2D matrix multiplication benchmark (500 runs with statistical analysis), Lamina achieved competitive performance:

| Language   | Time (s) | Performance Ratio | Memory (MB) | Memory Ratio |
| ---------- | -------- | ----------------- | ----------- | ------------ |
| Lamina     | 0.0372   | 1.00x (baseline)  | 1.38        | 1.00x        |
| Zig        | 0.0021   | 0.06x             | 0.50        | 0.36x        |
| C          | 0.0098   | 0.26x             | 1.50        | 1.09x        |
| C++        | 0.0101   | 0.27x             | 3.49        | 2.54x        |
| Go         | 0.0134   | 0.36x             | 1.60        | 1.16x        |
| Rust       | 0.0176   | 0.47x             | 1.91        | 1.39x        |
| C#         | 0.0333   | 0.90x             | 30.39       | 22.10x       |
| Java       | 0.0431   | 1.16x             | 42.93       | 31.22x       |
| Python     | 2.2585   | 60.70x            | 12.38       | 9.00x        |
| JavaScript | 2.7995   | 75.24x            | 53.20       | 38.69x       |

While Lamina is not beating Zig or C anytime soon, it's currently competitive with C# or Java and significantly outperform interpreted languages like Python and JavaScript.

## The Development Journey

### Starting Simple

The project began with basic arithmetic operations and simple control flow, and adding things like Functions, Global Variables, Pointer Operations, Memory Operations, and more.

- **Basic Arithmetic**: All arithmetic operations with proper type checking
- **Control Flow**: Conditionals, loops, and branching
- **Function Calls**: Non-recursive function calls and returns
- **Memory Operations**: Stack and heap allocations
- **Extern Functions**: Integration with extern functions for both x86_64 and AArch64

### The IR Design

One of the most challenging aspects was designing the intermediate representation. I wanted something that was:

1. **Human-readable**: Easy to debug and understand
2. **Efficient**: Fast to process and optimize
3. **Extensible**: Easy to add new features

The resulting IR syntax is clean and intuitive:

```lamina
# Type declaration
type @Vec2 = struct { x: f32, y: f32 }

# Global value
global @message: [5 x i8] = "hello"

# Function with annotations
@export
fn @add(i32 %a, i32 %b) -> i32 {
  entry:
    %sum = add.i32 %a, %b
    ret.i32 %sum
}
```

### Cross-Platform Support

Supporting multiple architectures was crucial for a modern compiler backend. Lamina currently supports:

- `x86_64_linux`: Linux x86_64 - Tested
- `x86_64_macos`: macOS x86_64 - Maybe Work - Not Tested    
- `aarch64_macos`: macOS ARM64 (Apple Silicon) - Tested
- `aarch64_linux`: Linux ARM64 - Maybe Work - Not Tested


## The Release Process

### Documentation

Creating comprehensive documentation was crucial.
I have put the docstrings in the codebase for the parts that will be used by the user.
[Documentation](https://docs.rs/lamina)

- API documentation with examples
- Multiple target platform support

### Testing

The project includes extensive test cases covering:

- Basic arithmetic operations
- Control flow constructs
- Memory operations
- Cross-platform compatibility
- Performance regression testing

## What's Next

The current release (v0.0.3) represents a solid foundation, but there's still much work to be done:

### Enhanced Optimizations
- Loop analysis and optimization
- Auto-vectorization for SIMD instructions
- Advanced register allocation algorithms
- Dead code elimination

### Additional Architectures and Platforms
- Windows support
- BSD variants support
- RISC-V support
- Additional ARM variants
- WebAssembly target

### Language Integration
- C frontend for easier integration
- Go frontend for easier integration
- Custom language frontend examples
  - Brainfuck frontend coming soon

### JIT Compilation
- Dynamic code execution support
- Runtime optimization
- Hot code path optimization

### GPU Acceleration - Maybe via Zen Accelerator(Planned for Cetana ML Framework to use)
- CUDA code generation
- Vulkan compute shader support
- Parallel execution optimization

## Reflections

Building Lamina has been an incredible learning experience. It's given me deep insights into:

- How compilers work at a low level
- The challenges of cross-platform development
- Performance optimization techniques
- The complexity of modern compiler backends

## Getting Started

If you're interested in exploring Lamina or building your own compiler frontend, check out the [Documentation](https://docs.rs/lamina). The codebase is well-documented and includes examples for common use cases.

The IRBuilder API makes it straightforward to build compilers for other languages, and the performance characteristics make it suitable for systems programming tasks.

## Conclusion

The journey from a simple arithmetic evaluator to a multi-target compiler backend has been challenging but incredibly fun to do. 
I'm excited to see where this project goes next and how others might use it to build their own language implementations.
> Well also expecting a lot of bugs too, but that's part of the fun of building your own compiler backend.
> While building the brainfuck compuler I'm also experiecing a lot of bugs and fixing them.

If you're interested in compiler construction or just want to see what's possible with Lamina, I encourage you to explore the codebase and perhaps even contribute to its development. 