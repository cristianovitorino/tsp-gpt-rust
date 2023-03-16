<p align="center">
    <img src="" alt="icon" width="150"/>
</p>

<h1 align="center">
    TSP GPT-3 Rust
</h1>

## About
Attacking the [**Traveling Salesman Problem**](https://en.wikipedia.org/wiki/Travelling_salesman_problem) in [**NP-hard**](https://en.wikipedia.org/wiki/NP-hardness) using [**GPT-3**](https://openai.com/product) and [**Rust**](https://www.rust-lang.org/)

## Features
- Accepts addresses input
- Accepts coordinates input
- TSP route handling using GPT-3
- Returns the best routing solution possible in [**Polynomial Time**](https://mathworld.wolfram.com/PolynomialTime.html)

## How to use
```
# API
# An environmental variable needs to be created. As long as the `.env` is still present at `.gitignore` your secret API token is never exposed publicly, which is the recommended and expected behaviour
touch .env && echo 'OAI_TOKEN=YOUR_OPENAI_API' >> .env

# CLI
# Press 'Ctrl + C' to exit the CLI application
cargo run
```

## Roadmap
- [x] OpenAI Auth Tokenization
- [x] OpenAI API Consumption
- [x] Async call
- [x] User input
- [x] Prompt template for testing
- [x] CLI Interface
- [ ] Custom prompt
    - [ ] Addresses input handling
    - [ ] Coordinates input handling
- [ ] TSP route handling through GPT-3
- [ ] Polynomial Time response
- [ ] Simple logo for the project
- [ ] Upgrade the interface from a CLI to a TUI
- [ ] Improve the TSP handling efficiency and accuracy
## Copyright

TSP GPT-3 Rust © Cristiano Vitorino, [BSD-3-Clause license](https://opensource.org/licenses/BSD-3-Clause)
