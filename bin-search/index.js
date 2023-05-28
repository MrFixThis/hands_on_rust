const rust = import("./pkg")
rust
    .then(m => m.search_report())
    .catch(console.error)
