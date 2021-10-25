# Firecrypt Decrypter
Command line tool for decrypting profiles encrypted with [Firecrypt](https://github.com/gltchitm/firecrypt).

## About
Firecrypt currently only supports macOS. This means that the official implementation cannot decrypt profiles on non-macOS devices. Firecrypt Decrypter allows for this to be done. It also serves as a short and simple reference to see how Firecrypt implements cryptography.

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
