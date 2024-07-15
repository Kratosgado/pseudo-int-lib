<div align="center">

  <h1><code>PseudoCode Interpreter Library</code></h1>

<strong>A Pseudocode interpreter library to be used in web apps. Written in Rust and exported to npm registry<a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <h3>Profile<h3>
  <h3>
    <a href="https://github.com/Kratosgado">Github Profile</a>
    <span> | </span>
    <a href="https://Kratosgado.github.io">Portfolio site</a>
  </h3>

<sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>

</div>
<div>
  <h3>Installation</h3>
  <ul>
    <li> npm <code> npm install pseudo-int-lib</code>
    <li> pnpm <code> pnpm add pseudo-int-lib</code>
  </ul>
</div>

# Pseudo Interpreter

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A pseudocode interpreter library written in rust and exported to npm registry.

## Implemented Features

- print function
- input function
- variable assignment
- expressions
- if statements
- multi conditional statements
- arrays
- for loops
- while loops
- functions

## Usage
```
export async function interpret(input: string): Promise<string> {
    let output = await import("pseudo-int-lib").then((module) => {
        try {
            return module.interpret(input);
        } catch (error) {
            throw error
        }
    });

    return output;
}
```
- Create a export a function that will load the wasm module and call its execute function. This is the function you will call in your frontend.

- the function can be used like this
- the module.interprete function returns a string when no error occurs or throws a string error when Error occurs.

```
const [input, setInput] = useState("");
const [output, setOutput] = useState("");
const [error, setError] = useState("");

const handleExecute = async () => {
  if (input.length < 1) {
    setError("No inputs");
    return;
  }
  try {
    const result = await interpret(input);
    console.info(result);
    setOutput(result);
    setError("");
  } catch (error) {
    console.error(error);
    setError(`${error}`);
  }
};

```

## Contact

- Email: [mbeahessilfieprince@gmail.com](mailto:mbeahessilfieprince@gmail.com)
- GitHub: [Kratosgado](https://github.com/Kratosgado)
- LinkedIn: [Prince Mbeah Essilfie](https://www.linkedin.com/in/prince-mbeah-essilfie-6bb0b5231)
- Twitter: [MbeahEssilfie](https://twitter.com/MbeahEssilfie)

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
