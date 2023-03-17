<p align="center">
    <img src="https://raw.githubusercontent.com/cristianovitorino/tsp-gpt-rust/main/icon.png" alt="icon" width="150"/>
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
```
cargo run
```
Press `Ctrl + C` when you need to exit the CLI application

#### Prompt
Type the city names like in the below example, separated by commas
```
sacramento, santa maria, bakersfield, santa rosa, monterey
```
#### Max Tokens
the `oai_request` variable can be increased for better accuracy, but it can get exponentially more expensive

## Roadmap
- [x] OpenAI Auth Tokenization
- [x] OpenAI API Consumption
- [x] Async call
- [x] User input
- [x] Preamble prompt
- [x] CLI Interface
- [x] TSP route handling through GPT-3
- [x] Polynomial Time response
- [x] Simple logo for the project
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


<div>
The TSP GPT-3 Rust logo is licensed under a <a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/">Creative Commons Attribution-NonCommercial-NoDerivatives 4.0 International License</a>

<br/><a rel="license" href="http://creativecommons.org/licenses/by-nc-nd/4.0/"><img alt="Creative Commons License" style="border-width:0" src="https://licensebuttons.net/l/by-nc-nd/4.0/88x31.png" /></a>
</div>
