<p align="center">
    <img src="assets/ezpie.png" width="220"/>
</p>

<h1 align="center">Ezpie</h1>
<p align="center"><strong>Create python projects <em>blazingly fast</em></strong></p>

### What Ezpie can do?
It can create a python project directory

### What kind of directory can Ezpie create?
eg.

```
my_project/
├── README.md
├── requirements.txt
├── .gitignore
├── src/
│   ├── __init__.py
│   ├── main.py
```

## Installation

### Windows

* Build Ezpie from source or download the latest build
    * https://github.com/Jupiee/ezpie/releases
    
* put the ezpie.exe from the build directory in your PATH
```C:\Users\<YourUsername>\AppData\Local\Microsoft\WindowsApps\```

### Linux

* Build Ezpie from source or download the latest build
    * https://github.com/Jupiee/ezpie/releases

* put the ezpie from the build directory in your PATH
```usr/local/bin```

## Usage

1. Open the terminal anywhere and type `ezpie`
2. Enter the name of the directory you want to create

## Building from Source

* Use cargo build to compile the project
```rust
cargo build --release
```

## License

[MIT License](LICENSE)