# Nuxt3 + Rust Example

---

This repository is a test of using Nuxt3 with Rust.

As it stands, there are three main parts:

- `app/` - A Rust library to be built in WebAssembly
- `ui/` - A Nuxt3 app which uses the above Rust library
- `server/` - A Rust application which embeds a static version of the Nuxt3 app and hosts it on port 3000.

For ease of use, building the `server` app will automatically build/bundle the others.
