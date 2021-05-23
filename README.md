# lsp
lsp is a little script I made to make viewing the permissions of a directory/file in Linux a little bit better/clearer and also to get my first contact with the Rust lang.

## Dependencies
```toml
[dependencies]
structopt = { version = "0.3", default-features = false }
users = "0.11.0"
colored = "2"
```

## Example output
![ss2](https://user-images.githubusercontent.com/49255836/119244741-daa22c00-bb49-11eb-9712-93d449e593ba.png)


## Installation
In order to use this you have to clone the repository to your machine with:

``` git clone https://github.com/c0nradLC/lsp.git ```

Change into lsp's directory

``` cd lsp ```

And run the package

``` cargo run ```, you can also pass the package's arguments and flags to ``` cargo run ```. Example:  ``` cargo run -- -f ~/ ```

## Usage
Since this is a small tool made only with the purpose of getting into Rust and learning a bit more about Linux permissions, its usage is pretty straightforward and some errors can be expected to happen.

### Arguments

``` path ```: The path to the directory of which you want to list the files/directories permissions.Default value is ``` ./ ```.

### Flags

``` -f ``` or ``` --list-files-only ```: Will list the permissions only for the entries that are files;

``` -d ``` or ``` --list-directories-only ```: Will list the permissions only for the entries that are directories.

![ss1](https://user-images.githubusercontent.com/49255836/119244365-539f8480-bb46-11eb-9fc9-eee4d51cb2cf.jpg)

# Warning
As of now, there seems to have a problem with the rust-users library/crate used to call the function to retrieve the groups that the current user is in, the function in question is adding the "root" group to a users group regardless of him belonging to the group or not, to avoid that I decided to not consider the user as part of the "root" group. I don't know if I should make a parameter to make it optional to exclude the user from the root group, but if I do so I'll edit this README.
