# WebSum
WebSum is a next-gen integrity checker written in Rust.

It can be used for finding out what an installation image contains or just validating it!
## Installation
WebSum can be installed with:
```
$ cargo install websum
```
(you should add $HOME/.cargo/bin to your PATH if you haven't already)
## How to use
The usage is pretty much the same as other tools like `sha256sum`
```
$ websum ./installation.iso
```
Example (Successful):
```
$ websum ./arch.iso
Processing...
SHA256SUM: 398DCEEA2D04767FBB8B61A9E824F2C8F5EACF62B2CB5006FD63321D978D48BC
This file is a valid archlinux-2024.07.01-x86_64.iso file.
```
Example (Unsuccessful):
```
$ websum ./random.iso
Processing...
SHA256SUM: 59AFA864CE54B70CADFFE846A251CB8462E868188154CBB1BABFA92BEBF1C2A0
This file is not found in our archive! It is either invalid or not in our archive.
```
## How does it work
WebSum firstly runs a `sha256sum` on the launch for the specified file.
Then the result sha256 is used to set a url to the `archive` directory in this repository.
```
https://raw.githubusercontent.com/yilmaz08/websum/main/archive/{sha256_hash}
```
and if the hash has been saved before, it contains the name of the file inside.

Finally, the HTTP response is used to print final response.

The archive can also be used manually by getting the hash with sha256 and visiting the raw file on the browser (or with curl).

## Contribution
WebSum is open source and we welcome any contributions.

You can either improve/fix the code or add/remove/fix hashes from the archive.

- Open an issue to discuss your proposed changes if it changes the source code. (When you only change the archive, you don't need an issue beforehand.)
- Fork the repository and make needed changes on the forked repository. (If you split changes into different commits, it would be better.)
- Open a pull request to merge your changes into the main WebSum repository.

We appreciate any contributions, no matter how small!
