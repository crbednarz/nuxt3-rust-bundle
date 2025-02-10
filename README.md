# Nuxt3 + Rust + Bevy Example

---

This repository is a test of using Nuxt3 with Rust + Bevy.

As it stands, there are four main parts:

- `app/` - A Rust library to be built in WebAssembly
- `ui/` - A Nuxt3 app which uses the above Rust library
- `server/` - A Rust application which embeds a static version of the Nuxt3 app and hosts it on port 3000.
- `viewer` - A Rust library which uses Bevy

For ease of use, building the `server` app will automatically build/bundle the others.
