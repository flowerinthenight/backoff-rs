[![main](https://github.com/flowerinthenight/backoff-rs/actions/workflows/main.yml/badge.svg)](https://github.com/flowerinthenight/backoff-rs/actions/workflows/main.yml)

## Overview

`backoff-rs` implements jittered backoff. Useful when retrying operations that can potentially fail (i.e. network calls). The implementation is based on [this article](https://www.awsarchitectureblog.com/2015/03/backoff.html) from the AWS Architecture Blog.

## Usage

You can use it like so:

``` rust
use exp_backoff::*;
use std::error::Error;
use std::{thread, time::Duration};

fn func_that_can_fail() -> Result<(), Box<dyn Error>> {
    if true {
        return Err("some error")?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut bo = BackoffBuilder::new().build();
    for _ in 0..5 {
        match func_that_can_fail() {
            Err(e) => {
                println!("failed: {:?}, retry...", e);
                thread::sleep(Duration::from_nanos(bo.pause()));
            }
            _ => println!("we're okay"),
        }
    }

    Ok(())
}
```
