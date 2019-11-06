# METAR Autotest

This tool is intended to run in a half-hourly cronjob to automatically test for
METARs which break the metar-rs library, so they can be added as tests.

A cronjob for this might be:

```
59 23 24 12 * ~/metar-test >/dev/null
```
