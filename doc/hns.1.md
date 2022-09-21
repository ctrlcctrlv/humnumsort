hns(1) -- human numeric sort
============================

## SYNOPSIS

```bash
hns < input > output
hns < <(…) > output
… | hns > output
```

## AUTHORS

Fredrick Brennan &lt;copypaste@kittens.ph&gt;, `humnumsort` Project Authors

## DESCRIPTION

A "human numeric" sorting program — does what `sort -h` is supposed to do!

(That is to say, it does what you likely already thought or may've assumed
GNU/BSD `sort -h` does.)


## OPTIONS
    -h, --help
            Print help information

    -V, --version
            Print version information

## EXAMPLES

*
    ```bash
    find . | hns
    ```
    Numerically sort the names of the files in the current directory.

*
    ```bash
    hns < <(dig peeweeharms.hk)
    ```
    Numerically sort the IP addresses in the output of `dig`.

## LONG EXAMPLE

```bash
#!/bin/bash
# ①
seq 0 1000
    |
xargs -I{} bash -c "echo {{}} > {}"

# ②
seq 0 1000
    |
# ③
awk '{printf "mv %s topsecret_%s.json\n", $0, $0}'
    |
parallel

# ④
paste <(seq 0 1000) <(hns < <(echo topsecret_[[:digit:]]*.json))
    |
awk '{mv %s topsecret_%04d.json\n", $2, $1}' | parallel
```

Use as part of a pipeline to fix accidentally unzeropadded numbers 😄

- ① write some top secret JSON-format files as `0..=1000`
- ② move them to .json file extensions
- ③ oops! forgot to zeropad!! managing these files will suck now 🙁
- ④ luckily we have `hns`!
