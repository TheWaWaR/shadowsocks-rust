// The MIT License (MIT)

// Copyright (c) 2014 Y. T. CHUNG <zonyitoo@gmail.com>

// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! TcpRelay server that running on local environment

use std::rc::Rc;

use futures::Future;

use tokio_core::reactor::Handle;

use config::Config;

use relay::{BoxIoFuture, boxed_future};

use super::socks5_local;
use super::http_local;

/// Starts a TCP local server
pub fn run(config: Rc<Config>, handle: Handle) -> BoxIoFuture<()> {
    let tcp_fut = socks5_local::run(config.clone(), handle.clone());
    match &config.http_proxy {
        &Some(..) => {
            let http_fut = http_local::run(config, handle);
            boxed_future(tcp_fut.join(http_fut)
                .map(|_| ()))
        }
        &None => tcp_fut,
    }
}
