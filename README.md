# `sit` (Simple Interval Timer)

Tired of fumbling around with timers and distracting UIs on your phone when trying to make some gains?!

Lucky for you, now you can manage all those timings with a single `sit` invocation in your nearest (Linux) terminal!

Enjoy a distraction-free workout in the server room: just listen for that series of `*beeps*`, and enjoy the satisfaction of that final `*bloop*`.

```txt
Simple Interval Timer is for specifying rounds of repeating intervals.
The start of each interval is marked by a high-pitched *beep*, and the end of the final round is marked by a more bassy *bloop*.

Intervals must be provided as positive integer values, and are assumed to be of unit seconds by default.
Units of seconds/minutes/hours can be specified using the corresponding suffix.

The number of rounds is one by default if the -r/--rounds flag is omitted.

Example Usage:

    # 5 rounds in total, each round has a 2 min 'set', followed by a 30 sec 'break'
    sit -r 5 2m 30s

    # 1 round in total, consisting of a 90 sec 'warmup', a 5 min 'set', and a 60 sec 'cooldown'
    sit 90 5m 60

    # a sequence of dubious intervals repeated many times
    sit --rounds 777 6s 6m 110h

Usage: sit [OPTIONS] [INTERVAL]...

Arguments:
  [INTERVAL]...


Options:
  -r, --rounds <ROUNDS>  [default: 1]
  -h, --help             Print help
```
