# yawn

An alternative to `sleep` written in Rust that allows specifying a timestamp and exit code.

## Usage

```
yawn [OPTIONS] <DURATION or TIMESTAMP>

OPTIONS:

    -e, --exit-code NUM
        Exit code, defaults to 0.

DURATION or TIMESTAMP:

    Durations are of the format:

        any positive integer or float are interpreted as seconds (eg 123, 123.45)

        a single string with 'd', 'h', 'm', 's', 'ms' as units, corresponding to days,
        hours, minutes, seconds, and milliseconds. (eg 1d2h3m4s5ms, 1d15m)

    Exact timestamps are of the format:

        RFC3339                (eg 1996-12-19T16:39:57-08:00)
        RFC2822                (eg Tue, 1 Jul 2003 10:52:37 +0200)
        YYYY-mm-dd HH:MM:SS    [local time] (eg 2005-01-03 12:23:34)
        YYYY-mm-dd HH:MM       (eg. 2005-01-03 12:23)

    Inexact-timestamps are of the format:

        HH:MM                  [24-hour format] (eg 14:23)
        HH:MM:SS               [24-hour format] (eg 14:23:45)
        HH:MM:SS.nnn           [24-hour format] (eg 14:23:45.678)
        h:MM P                 [12-hour format] (eg 8:12 am)

    Inexact timestamps always refer to next upcoming date.

EXAMPLES:

    yawn 1d2h3m4s

    Sleep for 1 day, 2 hours, 3 minutes, and 4 seconds.

    yawn '8:00 am'

    Sleep until the upcoming 8 am.

    yawn '2022-04-01T00:00:00Z'

    Sleep until midnight of April 1st, 2022 UTC.
```
