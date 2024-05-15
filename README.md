# cdadd
Small utility for ripping encoding CDs.

Ripping is not implemented yet. Encoding is done using `flac`. The metadata
files are expected to be produced by `cdda2wav`.

## Usage
- Use `cdda2wav` to rip the cd into folder `any/folder`
- Encode using `cdadd` into folder `encoded`:
```shell
cdadd -e any/folder -o encoded
```

## Links
- **Author**: [BonnyAD9][author]
- **GitHub repository**: [BonnyAD9/cdadd][repo]
- **My website**: [bonnyad.github.io][my-web]

[author]: https://github.com/BonnyAD9
[repo]: https://github.com/BonnyAD9/cdadd
[my-web]: https://bonnyad.github.io
[releases]: https://github.com/BonnyAD9/cdadd/releases
