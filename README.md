# This repository is just a copy of sub-directory in [savvy's repository](https://github.com/yutannihilation/savvy) to try building package for WebR

You can try the following on [a demo of the webR REPL](https://webr.r-wasm.org/latest/).

```r
webr::install("savvyExamples", repos = "https://yutannihilation.github.io/savvy-webr-test/")

library(savvyExamples)

# Returning error works fine, too!
times_two_int("1")

# CAUTION: currently this freezes...
times_two_int(1L)
```
