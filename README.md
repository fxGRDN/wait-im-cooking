# Wait, I'm Cooking!
Recipe managment app with cooking log and ingredient tracking.

## Stack:
- SvelteKit
- Tauri
- Rust
- SQLite


## Setup
1. Install rustup, android-ndk sdkmanager android-sdk-cmdline-tools-latest (or their distro equivalent)
2. Run `rustup default stable`
3. Run 
```bash
rustup target add \
aarch64-linux-android \
armv7-linux-androideabi \
i686-linux-android \
x86_64-linux-android \
aarch64-apple-ios \
x86_64-apple-ios
```
4. Run `yes | sdkmanager --licenses`
5. Run `sdkmanager ndk;26.3.11579264`
6. Set `ANDROID_HOME` to Android SDK location
7. Set `NDK_HOME` to Android NDK location

## Development

To generate sql types for rust install `cargo install sqlx-cli`.
Set the `DATABASE_URL` environment variable to your database connection string in src-tauri/.env.
Then run `cargo sqlx migrate run` to apply the database schema migrations.
Lastly, run `cargo sqlx prepare` to generate the types from the database schema.


For local development, run `(npm/deno/bun) run tauri dev`. 
For Android development, run `(npm/deno/bun) run tauri android dev`.

## Testing
To run tests, run `bun run test` for unit tests or `bun run test:e2e` for end-to-end tests.


## License
Idk MIT? It's just a project for my masters.
