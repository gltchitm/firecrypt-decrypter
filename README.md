# Firecrypt Decrypter
Command line tool for decrypting profiles encrypted with [Firecrypt](https://github.com/gltchitm/firecrypt).

## About
Firecrypt currently only supports macOS and Linux. Firecrypt Decrypter allows for profiles to be decrypted on other platforms. It also serves as a short and simple reference to see how Firecrypt implements cryptography.

## Protocol Version
Firecrypt Decrypter only supports Firecrypt Version 2. You need to use a compatible version of Firecrypt to decrypt legacy profiles.

## Usage
```
USAGE:
    firecrypt-decrypter <profile> <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <profile>    Path to the .firecrypt file
    <output>     Location to place the decrypted profile folder
```

## License
[MIT](LICENSE)
