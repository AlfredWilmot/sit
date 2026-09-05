# `sit` (Simple Interval Timer)

Tired of fumbling with timers on your phone while trying to make some gains?!

Grab your nearest CLI and use `sit` to manage the timings of your whole interval training session with one invocation!

Put your phone down, and focus on your form; just listen out for the *beeps*, and that final *bloop*.

```txt
Simple Interval Timer

A dead-simple CLI tool for quickly defining and keeping track of an arbitrary number of consecutive set/break intervals,
repeated by the number of specified rounds. The start of each interval is marked by a high-pitched *beep*,
and the end of the final round is marked by a more bassy *bloop*.
Intervals must be provided as positive integer values, and are assumed to be of unit seconds by default,
but units of seconds/minutes/hours can be specified using the corresponding suffix.
The number of rounds is one by default if the -r/--rounds flag is not used.

Example Usage:
# 5 rounds of 2 minute 'set' intervals, followed by 30s 'break' intervals
sit -r 5 2m 30s
# 1 round of a 60 second 'set' interval, followed by 1h 'break' interval
sit 60 1h

Usage: sit [OPTIONS] [INTERVAL]...

Arguments:
  [INTERVAL]...


Options:
  -r, --rounds <ROUNDS>  [default: 1]
  -h, --help             Print help

```
