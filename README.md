# `hns` — *H*uman *N*umeric *S*ort v0.3.0 (⏫︎2023-05-21)

* © 2022–2023 Fredrick R. Brennan and `hns` Authors
    * Apache 2.0 licensed, see LICENSE.  
* [**`man` page**](http://copypaste.wtf/humnumsort/doc/hns.1.html)

## Packages

* [`hns_0.3.0_amd64.deb` (for Debian, Ubuntu, Pop! OS, etc.)](https://github.com/ctrlcctrlv/humnumsort/releases/download/v0.3.0/hns_0.3.0_amd64.deb)
* [`hns-0.3.0-1.x86_64.rpm` (for Fedora, CentOS _n_, Red Hat Linux, etc.](https://github.com/ctrlcctrlv/humnumsort/releases/download/v0.3.0/hns-0.3.0-1.x86_64.rpm)
* [`humnumsort.PKGBUILD` (for Arch Linux)](dist/aur/PKGBUILD)

### Or if you have a Rust toolchain, install via Rust's package manager, `cargo`:

* `cargo install hns`

<div align="center">
  <a href="https://docs.rs/humnum">
    <img src="https://docs.rs/humnum/badge.svg" alt="Documentation">
  </a>
  <a href="https://crates.io/crates/hns">
    <img src="https://img.shields.io/crates/v/hns.svg" alt="Version">
  </a>
  <a href="https://github.com/ctrlcctrlv/humnumsort/blob/main/LICENSE">
    <img src="https://img.shields.io/crates/l/hns.svg" alt="License">
  </a>
</div>

## Commercial

**Has this ever happened to you?**

```bash
$ seq 1 30 | awk '{printf "data_%s.txt\n", $1}' | sort -h > important_filenames.txt
$ sort -h < important_filenames.txt
```

```
data_10.txt
data_11.txt
data_12.txt
data_13.txt
data_14.txt
data_15.txt
data_16.txt
data_17.txt
data_18.txt
data_19.txt
data_1.txt
data_20.txt
data_21.txt
data_22.txt
data_23.txt
data_24.txt
data_25.txt
data_26.txt
data_27.txt
data_28.txt
data_29.txt
data_2.txt
data_30.txt
data_3.txt
data_4.txt
data_5.txt
data_6.txt
data_7.txt
data_8.txt
data_9.txt
```

Oh no! You forgot that the `-h` flag of the GNU coreutils `sort` package doesn't actually do what it claims to, and can't be fixed for various historical reasons (stay with me now don't fall asleep)!

```
       -h, --human-numeric-sort
              compare human readable numbers (e.g., 2K 1G)
```

If only there was a better way!

Hi, FREDDY MAYS here with another FUR-tastic invention. All your numbers, sorted just for you!

```bash
$ mv important_filenames.txt tests/data/README_example.txt
$ hns < tests/data/README_example.shuf.txt
```
```
data_1.txt
data_2.txt
data_3.txt
data_4.txt
data_5.txt
data_6.txt
data_7.txt
data_8.txt
data_9.txt
data_10.txt
data_11.txt
data_12.txt
data_13.txt
data_14.txt
data_15.txt
data_16.txt
data_17.txt
data_18.txt
data_19.txt
data_20.txt
data_21.txt
data_22.txt
data_23.txt
data_24.txt
data_25.txt
data_26.txt
data_27.txt
data_28.txt
data_29.txt
data_30.txt
```

Wow!

But if you `git pull` in the next Unix epoch, you'll also get my super duper negative number-understanding version!

```bash
$ seq -10 10 | awk '{printf "data_%s.txt\n", $1}' | sort -h > tests/data/README_example2.shuf.txt
```

Before your numbers were sad and drab…

```
data_0.txt
data_-10.txt
data_10.txt
data_-1.txt
data_1.txt
data_-2.txt
data_2.txt
data_-3.txt
data_3.txt
data_-4.txt
data_4.txt
data_-5.txt
data_5.txt
data_-6.txt
data_6.txt
data_-7.txt
data_7.txt
data_-8.txt
data_8.txt
data_-9.txt
data_9.txt
```

But now they can be radically sequential! (Woah!)

```bash
$ hns < tests/data/README_example2.shuf.txt
```
```
data_-10.txt
data_-9.txt
data_-8.txt
data_-7.txt
data_-6.txt
data_-5.txt
data_-4.txt
data_-3.txt
data_-2.txt
data_-1.txt
data_0.txt
data_1.txt
data_2.txt
data_3.txt
data_4.txt
data_5.txt
data_6.txt
data_7.txt
data_8.txt
data_9.txt
data_10.txt
```

So simple to use! No command line options! Just standard in and standard out, one size fits all! (Really, Freddy?) (Yes!)

And it's not only for short files, oh no no no! It's written in Rust so you know it can handle even the largest data overflows, such as an entire randomly sorted Class A network!

```bash
$ time RUST_LOG=INFO target/release/hns < /tmp/0.0.0.0／8.shuf.txt > /dev/null
```
```ini
[2022-09-20T14:58:00Z INFO  hns] Reading done; got 16777216 lines in 348513µs
[2022-09-20T14:58:28Z INFO  hns] Sorting done; sorted 16777216 lines in 27122184µs
[2022-09-20T14:58:32Z INFO  hns] Writing done; wrote in 4370570µs
```
```
real    0m31.855s
user    0m30.306s
sys     0m1.548s
```

Sixteen million lines with four comparison points each sorted in under thirty seconds! And that's a Freddy Mays guarantee.

## Changelog

### v0.3.0 (2023-05-21)

- +`-C` flag (case insensitive sort)

### v0.2.0 (2023-05-11)

- 30x faster

### v0.1.1 (2022-09-22)

- hexadecimal mode

### v0.1.0 (2022-09-21)

- initial release

## Benchmarking data

* See [`humnumsort-test-data` repository](https://github.com/ctrlcctrlv/humnumsort-test-data/).

    You may want to clone it as:

    ```bash
    $ git clone https://github.com/ctrlcctrlv/humnumsort-test-data/ tests/data/expensive
    ```

## TODO

* Zero-padding via another binary?

## License

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   <http://www.apache.org/licenses/LICENSE-2.0>

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an _“as is” basis,
without warranties or conditions of any kind_, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
