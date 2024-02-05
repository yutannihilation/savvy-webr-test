# A repository for compiling Rust-powered R packages for webR

## Packages

* `savvyExamples`: an example package for testing
* [`prqlr`](https://github.com/eitsupi/prqlr)
* [`string2path`](https://github.com/yutannihilation/string2path)

## Usages

You can try running the following code on [a demo of the webR REPL](https://webr.r-wasm.org/latest/).

```r
webr::install("savvyExamples", repos = "https://yutannihilation.github.io/savvy-webr-test/")

library(savvyExamples)

times_two_int(1:10)

x <- Person$new()
x$set_name("湯谷")
x$name()
```
