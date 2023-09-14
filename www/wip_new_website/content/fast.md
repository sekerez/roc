# Fast

Roc code is designed to build fast and run fast...but what does "fast" mean here? And how close is Roc's curent implementation to realizing that design goal?

## Fast Programs

What "fast" means in embedded systems is different from what it means in games, which in turn is different from what it means on the Web. To better understand Roc’s performance capabilities, let's look at the upper bound of how fast optimized Roc programs are capable of running, and the lower bound of what languages Roc should generally outperform.

**Limiting factors: memory management and async I/O.** Part of Roc's design is that it is a [memory-safe](https://en.wikipedia.org/wiki/Memory_safety) language with [automatic memory management](https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)#Reference_counting). Automatic memory management has some unavoidable runtime overhead, and memory safety rules out certain performance optimizations—which is why [unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) can outperform safe Rust. This gives Roc a lower performance ceiling than languages which support memory unsafety and manual memory management, such as C, C++, Zig, and Rust. Another part of Roc's design is that I/O operations are done using a lightweight state machine so that they can be asynchronous. This has potential performance benefits compared to synchronous I/O, but it also has some unavoidable overhead.

**Faster than dynamic or gradual languages.** As a general rule, Roc programs should have almost strictly less runtime overhead than equivalent programs written in languages with dynamic types and automatic memory management. This doesn't mean all Roc programs will outperform all programs in these languages, just that Roc should have a higher ceiling on what performance is achievable. This is because dynamic typing (and gradual typing) requires tracking types at runtime, which has unavoidable overhead. Roc tracks types only at compile time, and tends to have [minimal (often zero) runtime overhead](https://guide.handmade-seattle.com/c/2021/roc-lang-qa/) for language constructs compared to the top performers in industry. For example, Roc's generics, records, functions, numbers, and tag unions have no more runtime overhead than they would in their Rust or C++ equivalents.

When [benchmarking compiled Roc programs](https://www.youtube.com/watch?v=vzfy4EKwG_Y), the normal goal is to have them outperform the fastest mainstream garbage-collected languages (for example, Go, C#, Java, and JavaScript), but it's a non-goal to outperform languages that support memory unsafety or manual memory management. There will always be some individual benchmarks where mainstream garbage-collected languages outperform Roc, but the goal is for these to be uncommon cases rather than the norm.

### Current Progress

Most of Roc's data structures are already close to their theoretical limit in terms of performance, at least without changing their behavior or introducing memory unsafety. However, there is plenty of room for further compiler optimizations; current optimizations include only [LLVM](https://llvm.org/), [Morphic](https://www.youtube.com/watch?v=F3z39M0gdJU&t=3547s), and [Perceus](https://www.microsoft.com/en-us/research/uploads/prod/2020/11/perceus-tr-v1.pdf). Promising examples of potential future optimizations include closure-aware [inlining](https://en.wikipedia.org/wiki/Inline_expansion), [automatic deforestation](https://www.cs.tufts.edu/~nr/cs257/archive/duncan-coutts/stream-fusion.pdf), and full [compile-time evaluation](https://en.wikipedia.org/wiki/Constant_folding) of top-level declarations.

An open design question is how atomic reference counting should work in the context of platforms. Roc uses automatic reference counting (with some compile-time optimizations) to manage memory, and updating reference counts runs a lot faster if you don't need to guard against [data races](https://en.wikipedia.org/wiki/Race_condition#Data_race). If Roc data structures are shared across threads (which might or might not come up, depending on the platform), then in some cases [atomic](https://en.wikipedia.org/wiki/Linearizability#Primitive_atomic_instructions) reference counting might be necessary, which is slower.

A design of "always use atomic reference counts" would be unnecessarily slow for platforms where atomicity isn't needed. A design of "never use atomic reference counts" would mean sharing Roc data structures across threads would always require deeply copying the entire data structure (which has worked well for Erlang, but which might not work as well for Roc). It seems likely that the best design is for the compiler to sometimes choose one and sometimes choose the other, but the exact design to go with is currently an open question.

## Fast Feedback Loops

The goal is for Roc to provide fast feedback loops by making builds normally feel "instant" except on truly massive projects. It's a concrete goal to have them almost always complete in under 1 second on the median computer being used to write Roc (assuming that system is not bogged down with other programs using up its resources), and ideally under the threshold at which humans typically find latency perceptible (around 100 milliseconds). Hot code loading can make the feedback loop even faster, by letting you see changes without having to restart your program.

Note that although having fast "clean" builds (without the benefit of caching) is a goal, the "normally feels instant" goal refers to builds where caching was involved. After all, the main downside of build latency is that it comes up over and over in a feedback loop; the initial "clean" build comes up rarely by comparison.

### Current Progress

`roc check` type-checks your code and reports problems it finds. `roc build` also does this, but it additionally
builds a runnable binary of your program. You may notice that `roc build` takes much longer to complete! This is because
of two projects that are underway but not completed yet:
- *Development backend* refers to generating machine code directly instead of asking [LLVM](https://llvm.org/) to generate it. LLVM is great at generating optimized machine code, but it takes a long time to generate it—even if you turn off all the optimizations (and `roc` only has LLVM perform optimizations when the `--optimize` flag is set). The dev backend is currently implemented for WebAssembly, which you can see in the [Web REPL](https://www.roc-lang.org/repl), and in `roc repl` on x64 Linux. Work is underway to implement it for `roc build` and `roc run`, as well as macOS, Windows, and the ARM versions of all of these.
- *Surgical linking* refers to a fast way of combining the platform and application into one binary. Today, this works on Linux, Windows, and WebAssembly. `roc build` on macOS is noticeably slower because it falls back on non-surgical linking.

Here's a table summarizing the current progress:

Target                     | Dev backend | Surgical linking  |
---------------------------|-------------|-------------------|
WebAssembly                |     yes     |        yes        |
Linux x64                  |  repl only  |        yes        |
Windows x64                |             |        yes        |
macOS Intel (x64)          |             |                   |
Linux ARM                  |             |                   |
Windows ARM                |             |                   |
macOS Apple Silicon (ARM)  |             |                   |

Once we have full coverage, `roc build` (and `roc run` and `roc test`, which also perform builds) should take only a bit longer than `roc check`.

The next major performance improvement will be caching. Currently, `roc` always builds everything from scratch. Most of the time, it could benefit from caching some of the work it had done in a previous build, but today it doesn't do that. There's a design for the caching system, but essentially none of the implementation has started yet. Hot code loading will be the next major improvement after caching, but it requires full dev backend coverage, and does not have a concrete design yet.
