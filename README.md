# tcalc

A little duration calculator.

```sh
$ tcalc 5m 32s + 4m 2s
9m 37s
$ tcalc 23h + 5h
1d 4h
$ tcalc 30s + 600s
10m 30s
```

Should be a pretty fun little project if/when I get the time.

Basically just needs:

1. A parser to parse the duration expressions (supporting `w` for weeks, `d` for days, `h` for hours, `m` for minutes, and `s` for seconds), with all durations being parsed into seconds, and with only `+` and `-` operations being supported.
2. A printer to take a number of seconds and print it with the largest unit possible filled first.
