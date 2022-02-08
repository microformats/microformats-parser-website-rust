# Microformats Rust Parser Website

Website for Microformats Rust parser (based on pin13.net by [Aaron Parecki](https://aaronpk.com)).

https://rust.microformats.io

## Deployment

All commits to the `main` branch get auto-deployed to the live website (running on [Heroku](https://rust.microformats.io))

## Development Status

Implemented:

- Parse a website from a pasted URL
- Parse a blob of `HTML` with optional `Base URL`
- Optionally, save `HTML` blob, creating a permalink (a la Gist)
- Optionally, display rendered `HTML` blob on the page

## Requirements

- Cargo (to build)

## Installation

Clone the repo:

```
git clone https://github.com/microformats/microformats-rust-parser-website.git
cd microformats-rust-parser-website
```

Start the server:

```
cargo run
```

Open the site in your browser:

```
open http://localhost:8000

# For XDG-compliant systems
xdg-open http://localhost:8000
```


## Authors

- [Jacky Alciné](https://jacky.wtf)

## Contributions

1. Fork it
2. Get it running (see Installation above)
3. Create your feature branch (`git checkout -b my-new-feature`)
4. Write your code and **specs**
5. Commit your changes (`git commit -am 'Add some feature'`)
6. Push to the branch (`git push origin my-new-feature`)
7. Create new Pull Request

If you find bugs, have feature requests or questions, please
[file an issue](https://github.com/indieweb/microformats-parser-website-rust/issues).


## License

Microformats Rust Parser Website is dedicated to the public domain using Creative Commons -- CC0 1.0 Universal.

http://creativecommons.org/publicdomain/zero/1.0
