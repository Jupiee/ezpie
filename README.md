<p align="center">
    <img src="assets/ezpie.png" width="220"/>
</p>

<h1 align="center">Ezpie</h1>
<p align="center"><strong>Create python projects <em>blazingly fast</em></strong></p>

### What Ezpie can do?
It can create a python project directory

### What kind of directory can Ezpie create?
For custom projects:

```
my_project/
├── README.md
├── requirements.txt
├── .gitignore
├── src/
│   ├── __init__.py
│   ├── main.py
```

For discord.py projects:

```
my_project/
├── bot.py
├── requirements.txt
├── .gitignore
├── cogs/
```

## Installation

### Windows

* Build Ezpie from source or download the ```windows.bat``` from releases
    * https://github.com/Jupiee/ezpie/releases

### Linux

* Build Ezpie from source or download the ```linux.sh``` from releases
    * https://github.com/Jupiee/ezpie/releases

## Usage

For Custom Projects:
1. Open the terminal anywhere and type 
`ezpie project-name`

For Discord.py Projects:
1. Open the terminal anywhere and type 
`ezpie project-name -d` or `ezpie project-name --discord`

## Building from Source

* Use cargo build to compile the project
`
cargo build --release
`

## Goals

* [ ] Use tokio for faster performance
* [ ] Flag to create virtual environment
* [x] Support for Discord.py projects

## License

[MIT License](LICENSE)