# rust-del-dups

Rust remove-duplicate API

Simply removes ngram duplicates in the input string.

- Allows up to 5 unigram repetitions
- Allows up to 3 bigram repetitions
- Allows up to 1 N-gram repetitions (N > 2)


## How to use

Simply build using cargo, then run the API via `cargo run`

```
cargo run
...
Running `target/debug/src`
🔧 Configured for debug.
   >> address: 127.0.0.1
   >> port: 8000
   >> workers: 10
   >> ident: Rocket
   >> limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
   >> temp dir: /var/folders/6k/tjrzsxzd6_13ngrfyc6t13_w0000gp/T/
   >> http/2: true
   >> keep-alive: 5s
   >> tls: disabled
   >> shutdown: ctrlc = true, force = true, signals = [SIGTERM], grace = 2s, mercy = 3s
   >> log level: normal
   >> cli colors: true
📬 Routes:
   >> (remove_dups_handler) GET /remove-dups?<q>
📡 Fairings:
   >> Shield (liftoff, response, singleton)
🛡️ Shield:
   >> X-Frame-Options: SAMEORIGIN
   >> Permissions-Policy: interest-cohort=()
   >> X-Content-Type-Options: nosniff
🚀 Rocket has launched from http://127.0.0.1:8000

```

* Sample run
  ```
  // http://localhost:8000/remove-dups?q=leetleetleetcode%20is%20not%20fun
  
  {
    "result": "leetcode is not fun "
  }
  ```

