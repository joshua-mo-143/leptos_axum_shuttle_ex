# Leptos Starter Template

This is a template demonstrating how to integrate [TailwindCSS](https://tailwindcss.com/) with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [trunk](https://github.com/thedodd/trunk) tool. 
[Axum](https://www.github.com/tokio-rs/axum) support and support for deployment via [Shuttle](https://www.shuttle.rs) has also been added to this.

Install Tailwind and build the CSS:

`Trunk.toml` is configured to build the CSS automatically and build the static assets to the backend folder.

While you're building the frontend, you can use trunk to client side render this bundle - you can install it like so:

`cargo install trunk`
Then the site can be served with `trunk serve --open`

The browser will automatically open [http://127.0.0.1:8080//](http://127.0.0.1:8080//)

You can begin editing your app at `src/app.rs`.

When you're ready to build your static assets, you can then build it into your backend folder:

`trunk build`

To use the backend folder, you'll need to install cargo-shuttle which you can install with the following command (also supports cargo-binstall):

`cargo install cargo-shuttle`

Then you can start running your app on the backend by using the following command (the following uses aliased cargo commands; for more information, go to .cargo/config.toml):

`cargo sh`

To deploy your app, you'll want to login to shuttle via GitHub then input your API key into the CLI:

`cargo sh-login`

Then you can start your project and deploy with the following commands:

`cargo start && cargo deploy`

If at any point you need to stop your deployed container, you can use `cargo stop`.



## Installing Tailwind

You can install Tailwind using `npm`:

```bash
npm install -D tailwindcss
```

If you'd rather not use `npm`, you can install the Tailwind binary [here](https://github.com/tailwindlabs/tailwindcss/releases).

## Setting up with VS Code and Additional Tools

If you're using VS Code, add the following to your `settings.json`

```json
  "emmet.includeLanguages": {
    "rust": "html",
    "*.rs": "html"
  },
  "tailwindCSS.includeLanguages": {
      "rust": "html",
      "*.rs": "html"
  },
  "files.associations": {
      "*.rs": "rust"
  },
  "editor.quickSuggestions": {
    "other": "on",
    "comments": "on",
    "strings": true
  },
  "css.validate": false,
```

Install [Tailwind CSS Intellisense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss).

    Install "VS Browser" extension, a browser at the right window.
    Allow vscode Ports forward: 3000, 3001.

## Notes about Tooling

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup default nightly` - setup nightly as default, or you can use rust-toolchain file later on
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
5. `npm install -g sass` - install `dart-sass` (should be optional in future


## Attribution
This is based on the original Tailwind example (../examples/tailwind)
