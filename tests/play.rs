#![allow(warnings)]
use std::{
    future::{poll_fn, Future},
    pin::pin,
    task::{Context, Poll},
};
