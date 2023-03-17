<p align="center">
    <img src="" alt="icon" width="150"/>
</p>

<h1 align="center">
    TSP GPT-3 Rust
</h1>

## About
Attacking the [**Traveling Salesman Problem**](https://en.wikipedia.org/wiki/Travelling_salesman_problem) in [**NP-hard**](https://en.wikipedia.org/wiki/NP-hardness) using [**GPT-3**](https://openai.com/product) and [**Rust**](https://www.rust-lang.org/)

## Features
- Accepts cities as input
- TSP route handling using GPT-3
- Returns the best routing solution possible in [**Polynomial Time**](https://mathworld.wolfram.com/PolynomialTime.html)

## How to use
#### API
An environmental variable needs to be created. As long as the `.env` is still present at `.gitignore` your secret OpenAI API key is never exposed publicly, which is the recommended and expected behaviour
```
touch .env && echo 'OAI_TOKEN=YOUR_OPENAI_API' >> .env
```

#### CLI
Press `Ctrl + C` when you need to exit the CLI application
```
cargo run
```

#### Prompt
Type the city names like in the below example, separated by commas
```
sacramento, santa maria, bakersfield, santa rosa, monterey
```
#### Max Tokens
the `oai_request` variable can be increased for more accuracy, but it can get exponentially more expensive

## Roadmap
- [x] OpenAI Auth Tokenization
- [x] OpenAI API Consumption
- [x] Async call
- [x] User input
- [x] Preamble prompt
- [x] CLI Interface
- [x] TSP route handling through GPT-3
- [x] Polynomial Time response
- [ ] Simple logo for the project
- [ ] Custom preamble prompt tuning
    - [ ] Addresses input handling
    - [ ] Geolocation input handling
- [ ] Upgrade the interface from a CLI to a TUI
- [ ] Improve the TSP handling efficiency and accuracy
- [ ] Steadly increase the application complexity and scaling by simulating a TSP Modeling by integrating [WolframAlpha](https://www.wolframalpha.com/)
    - [ ] Build a local, personalized database, incremented with every prompt response
    - [ ] Simulate GPT-3 Modeling considering the database as input
    - [ ] Integrate WolframAlpha algorithms for better accuracy

## Copyright

TSP GPT-3 Rust © Cristiano Vitorino, [BSD-3-Clause license](https://opensource.org/licenses/BSD-3-Clause)
