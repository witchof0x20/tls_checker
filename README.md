# tls_checker

Often when gathering background traffic using selenium or other browser automation, it's useful to use some list of top N websites. However, this has many problems.
* The host may not support HTTPS, meaning you must navigate starting with the insecure HTTP version of the website. This also is detrimental if you only want HTTPS traffic
* The host may be part of a larger hostname, and not meaningfully navigable by itself
* The host may not even resolve

This program goes through a list and filters down to just websites where a `GET` request to `https://host` works.

## Problems:
Sometimes connections fail for other reasons. Perhaps this program wasn't written exactly right, the website has robust bot blocking, or there was some spurious DNS issue. Use at your own discretion

## Inputs
Takes in a csv of rank,host pairs, similar to alexa top N files

## Usage
Usage pasted from the program's help. When running from source, it's `cargo run --` or `cargo run --release --` instead of `tls_checker`
```
USAGE:
    tls_checker [OPTIONS] <CSV_PATH> <OUT_PATH>

ARGS:
    <CSV_PATH>    Path to the top website csv (number, hostname)
    <OUT_PATH>    Path to write a website list to

OPTIONS:
    -c, --count <COUNT>    Size of the desired list. If absent, the program will go through all urls
```
