  <h1><code>My conway's  game of life</code></h1>

  <strong>Kickstarted from `wasm-pack-template`</strong>

## 🚴 Usage

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### Execute locally

After building, go to `./www` and run `npm run start`.

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
