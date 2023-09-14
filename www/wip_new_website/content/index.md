<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=PT+Serif:ital,wght@0,400;0,700;1,400;1,700&display=swap" rel="stylesheet">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Merriweather:ital,wght@0,400;0,700;1,700&family=PT+Serif:ital,wght@0,400;0,700;1,400;1,700&display=swap" rel="stylesheet">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Merriweather+Sans:ital,wght@0,400;0,500;0,600;0,700;0,800;1,400;1,500;1,600;1,700;1,800&family=Merriweather:ital,wght@0,400;0,700;1,700&family=PT+Serif:ital,wght@0,400;0,700;1,400;1,700&display=swap" rel="stylesheet">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Merriweather+Sans:ital,wght@0,400;0,500;0,600;0,700;0,800;1,400;1,500;1,600;1,700;1,800&family=Merriweather:ital,wght@0,400;0,700;1,700&family=PT+Sans:ital,wght@0,400;0,700;1,400;1,700&family=PT+Serif:ital,wght@0,400;0,700;1,400;1,700&display=swap" rel="stylesheet">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Fira+Sans:ital,wght@0,300;0,400;1,300;1,400&family=Merriweather+Sans:ital,wght@0,400;0,500;0,600;0,700;0,800;1,400;1,500;1,600;1,700;1,800&family=Merriweather:ital,wght@0,400;0,700;1,700&family=PT+Sans:ital,wght@0,400;0,700;1,400;1,700&family=PT+Serif:ital,wght@0,400;0,700;1,400;1,700&display=swap" rel="stylesheet">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Fira+Sans:ital,wght@0,300;0,400;1,300;1,400&family=Merriweather+Sans:ital,wght@0,400;0,500;0,600;0,700;0,800;1,400;1,500;1,600;1,700;1,800&family=Merriweather:ital,wght@0,400;0,700;1,700&family=PT+Sans:ital,wght@0,400;0,700;1,400;1,700&family=PT+Serif:ital,wght@0,400;0,700;1,400;1,700&family=Ubuntu:ital,wght@0,400;0,500;0,700;1,300;1,400;1,500;1,700&display=swap" rel="stylesheet">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Source+Sans+3:ital,wght@0,400;0,500;0,600;0,700;0,800;0,900;1,400;1,500;1,600;1,700;1,800;1,900&display=swap" rel="stylesheet">

<h1 id="homepage-h1">Roc</h1>

<p id="tagline">A fast, friendly, functional language.</p>

<!-- This exact sample was chosen for several reasons:

1. It's plausible to figure out what it's doing even if you don't know the language yet.
2. It uses a higher-order function, giving a functional first impression.
3. It shows some things not found in most mainstream languages, e.g. function calls without parens, lambda syntax.
4. It shows some things not found in most FP languages, e.g. string interpolation, passing a lambda without `<|` or `$`
5. It's horizontally small enough that it can be read on mobile without a scroll bar or shrinking the font size.
-->
<pre id="first-code-sample"><samp class="code-snippet">list <span class="kw">=</span> List<span class="punctuation section">.</span>map songs <span class="kw">\</span>song <span class="kw">-></span>
    <span class="string">"Artist: </span><span class="kw">\(</span>song<span class="punctuation section">.</span>artist<span class="kw">)</span><span class="string">"</span></samp></pre>

<p><b>Fast</b> - Roc code is designed to build fast and run fast. It compiles to machine code or to <a href="https://webassembly.org/">WebAssembly</a>. Like <a href="https://rust-lang.org">Rust</a> and <a href="https://clang.llvm.org/">clang</a>, it compiles to unboxed data structures using monomorphization and LLVM for optimizations.</p>
<p><b>Friendly</b></p>
<p><b>Functional</b></p>

## REPL

> TODO only show this if you have JavaScript enabled!

You can get a quick taste of Roc in this WebAssembly Read-Eval-Print Loop (REPL). Try entering 0.1 + 0.2 here:

<input placeholder="Try writing 0.1 + 0.2 and pressing Enter" style="width:600px">

Roc code can compile to WebAssembly, so this REPL runs completely in your browser. No network connection required!

## Use cases

Roc is a new language, and its ecosystem is very small. It currently has the most mature support for these use cases:

- Web servers ([tutorial on making a webserver in Roc](...))
- Command-Line Interfaces ([tutorial on making a CLI in Roc](...))
- Embedding Roc into other languages ([tutorial on calling Roc code from other languages](...))

There are many other use cases the ecosystem can potentially expand into in the future, but these are the best-supported
ones today.

## Example

```elixir
# Hover over anything here to see an explanation
main =
    cacheUserInfo (Path.fromStr "url.txt")
    |> Task.onErr handleErr

cacheUserInfo = \filename -> # Defining a function
    url <- File.readUtf8 filename |> Task.await
    { username, email } <- Http.get url Json.codec |> Task.await

    File.writeUtf8 "\(username).txt" email

handleErr = \err ->
    when err is
        FileReadErr path _ -> Stderr.line "Error reading \(Path.display path)"
        FileWriteErr path _ -> Stderr.line "Error writing \(Path.display path)"
        HttpErr url _ -> Stderr.line "Error getting \(url)"
```

> TODO maybe make some notes about the above code, e.g. it uses basic-cli and Luke's JSON package,
> this is demonstrating how type annotations are optional; this code is 100% type-checked even though
> there are no types mentioned anywhere. That also includes the JSON decoding, which is done via
> type inference, and which is not specific to JSON (or any particular encoding).

There are [more examples](https://github.com/roc-lang/examples) of full Roc programs,
and also a [tutorial](/tutorial) which takes you through building your first Roc application.

## Platforms & Applications

TODO provide explanation of platform/application abstraction versus libraries as common in most other languages as this is one of the most unique features of Roc

## Talks and Publications

If you'd like to learn more about Roc check out one of these videos:

*   [Roc at Handmade Seattle](https://media.handmade-seattle.com/roc-lang) - November 12, 2021 (very low-level explanation of how Roc's compiler makes programs run fast)
*   [Outperforming Imperative with Pure Functional Languages](https://youtu.be/vzfy4EKwG_Y) - October 1, 2021 (about Roc's runtime performance and optimizer)
*   [A taste of Roc](https://youtu.be/6qzWm_eoUXM) - September 23, 2021 (syntax, application examples)
*   [Roc at the Philly ETE conference](https://youtu.be/cpQwtwVKAfU?t=75) - May 6, 2021 (platforms and applications)
*   [Roc on Zig Showtime](https://youtu.be/FMyyYdFSOHA) - April 24, 2021 (making a platform)
*   [Roc at the Berlin FP Meetup](https://youtu.be/ZnYa99QoznE?t=4790) - September 1, 2020 (overall vision for the language)

<script src="repl.js"></script>
