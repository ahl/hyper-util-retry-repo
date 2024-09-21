This reproduces an issue we've seen with reqwest 0.12 that we tracked down to
hyper-util. We see what appears to be aberrant behavior when a client
(`reqwest::Client` or `hyper_util::client::legacy::Client`) is making a request
to a server that may close the connection deliberately. In particular, we see
that the client opens a new connection and may try opening connections many
times!

The client stops opening connections once it has been able to send the full
request prior to the server performing a `shutdown` on the TCP stream.

The results of running this program are non-deterministic as the client writing
the request races with the server closing the stream, but you should see output
like this:

```console
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/hyper-util-retry-repo`
closed peer: 127.0.0.1:64480
closed peer: 127.0.0.1:64481
closed peer: 127.0.0.1:64482
closed peer: 127.0.0.1:64483
closed peer: 127.0.0.1:64484
closed peer: 127.0.0.1:64485
closed peer: 127.0.0.1:64486
closed peer: 127.0.0.1:64487
closed peer: 127.0.0.1:64488
closed peer: 127.0.0.1:64489
closed peer: 127.0.0.1:64490
closed peer: 127.0.0.1:64491
closed peer: 127.0.0.1:64492
closed peer: 127.0.0.1:64493
closed peer: 127.0.0.1:64494
closed peer: 127.0.0.1:64495
closed peer: 127.0.0.1:64496
closed peer: 127.0.0.1:64497
closed peer: 127.0.0.1:64498
closed peer: 127.0.0.1:64499
closed peer: 127.0.0.1:64500
closed peer: 127.0.0.1:64501
closed peer: 127.0.0.1:64502
closed peer: 127.0.0.1:64503
closed peer: 127.0.0.1:64504
closed peer: 127.0.0.1:64505
closed peer: 127.0.0.1:64506
Err(
    hyper_util::client::legacy::Error(
        SendRequest,
        hyper::Error(
            IncompleteMessage,
        ),
    ),
)
```

We would expect to see a single connection attempt by the client rather than
many.
