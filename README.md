# Conv
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](./LICENSE)
![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange.svg)
![Version](https://img.shields.io/badge/version-1.1.1-yellow.svg)

A simple and useful value converter between number systems.

## ▶ What does it do?
`Conv` lets you easily convert between **hex**, **dec**, **bin**, and **oct** number formats.

## ▶ How to use?
It's super simple! Just run:

```bash
conv -<type> <value> -to_<type>
```

You can also use it as library!

```rs
fn main() {
    let value = conv::convert("A", "hex", "dec").unwrap();
    println!("Converted value: {}", value); // Output: Converted value: 10
}
```

## ▶ For example:
```bash
# I want to convert hex A to decimal
conv -hex A -to_dec
# Output: 10
# You can also use -to_all to get all translations.
```

## Installation 📩
Clone repo and build with cargo
```bash
git clone https://github.com/manguseqq/conv.git
cd conv
cargo build --release
```

## Contributing 🤝
Im happy to receive pull requests bug reports and new feature suggestions!  
Feel free to fork the repo and send your improvements all contributions are welcome!

## License 📚
Licensed under MIT License.
See the ```LICENSE``` file for details.

#### That's all :3 ❤