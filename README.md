<br/>
<p align="center">
  <a href="https://github.com/Tanzkalmar35/ScheduleFlow">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">ScheduleFlow automated calendar</h3>

  <p align="center">
    Schedule with ease, powered by AI grace!
    <br/>
    <br/>
    <a href="https://github.com/Tanzkalmar35/ScheduleFlow"><strong>Explore the docs »</strong></a>
    <br/>
    <br/>
    <a href="https://github.com/Tanzkalmar35/ScheduleFlow/issues">Report Bug</a>
    .
    <a href="https://github.com/Tanzkalmar35/ScheduleFlow/issues">Request Feature</a>
  </p>
</p>

![Contributors](https://img.shields.io/github/contributors/Tanzkalmar35/ScheduleFlow?color=dark-green) ![Forks](https://img.shields.io/github/forks/Tanzkalmar35/ScheduleFlow?style=social) ![Issues](https://img.shields.io/github/issues/Tanzkalmar35/ScheduleFlow) ![License](https://img.shields.io/github/license/Tanzkalmar35/ScheduleFlow) 

## Table Of Contents

* [About the Project](#about-the-project)
* [Built With](#built-with)
* [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
* [Usage](#usage)
* [Contributing](#contributing)
* [License](#license)

## About The Project

![Screen Shot](images/screenshot.png)

There are many great calendar apps and tools, but I didn't find one that could fits all of my needs, so because I have no hobbies anyways I am now on my journey to create my own calendar with smart features such as:

* Automated event scheduling (coming soon) 
* Natural language processing (NLP) (also coming soon)
* cli calendar management including creating, deleting and editing tasks

## Built With

This project is built in rust using the clap crate for the commands.

## Getting Started

Setting up this project locally:
To get a local copy up and running follow these simple example steps.

### Prerequisites

You should already have rust and cargo installed on you machine, if not head over to the rust website to download rust for your os.


### Installation

1. Clone the repo

```sh
git clone https://github.com/Tanzkalmar35/ScheduleFlow
```

2. Install packages

```sh
cargo install
```

3. Run the program

```sh
cargo run -- [command]
```

## Usage

Simple example of adding a basic calendar entry (without nlp):

```sh
scheduleflow add -n <NAME> -d <DATE> -f <FROM_TIME> -t <TO_TIME> -c <COLOR>
```

Show the calendar for a specific date 

```sh
scheduleflow show -d <DATE>
```

## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.
* If you have suggestions for adding or removing projects, feel free to [open an issue](https://github.com/Tanzkalmar35/ScheduleFlow/issues/new) to discuss it, or directly create a pull request after you edit the *README.md* file with necessary changes.
* Please make sure you check your spelling and grammar.
* Create individual PR for each suggestion.
* Please also read through the [Code Of Conduct](https://github.com/Tanzkalmar35/ScheduleFlow/blob/main/CODE_OF_CONDUCT.md) before posting your first idea as well.

### Creating A Pull Request

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT license.
