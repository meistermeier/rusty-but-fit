# Garmin .FIT parser

A project to get a little bit more familiar with rust-lang.

The tool itself parses .FIT files and converts it to JSON-formatted messages.

At the moment heavy work in progress... but you can already try it out.

## Install
```bash
cargo install rusty-but-fit
```

## Commands
```text
Parsing for Garmin's FIT file format

Usage: rusty-but-fit [OPTIONS] --file <FILE> <COMMAND>

Commands:
  summary   Create summary of all messages and their count
  messages  Return messages defined by the -m parameter
  raw       Outputs all messages, incl. unknown messages and invalid fields
  help      Print this message or the help of the given subcommand(s)

Options:
  -f, --file <FILE>     FIT file to parse
  -d                    Debug output (cannot be piped to jq)
  -u, --unknown-fields  Output unknown fields
  -i, --invalid-values  Output invalid values
  -h, --help            Print help
  -V, --version         Print version
```

### Examples

#### `summary` command
##### Output message types in the file and their count:
```bash
rusty-but-fit -f activity2.fit summary
```
```json
{
  "Lap": 1,
  "Session": 1,
  "Time in zone": 2,
  "GPS Metadata": 1826,
  "Totals": 1,
  "Record": 382,
  "Unknown": 2004,
  "Sport": 1,
  "Training file": 2,
  "Activity": 1,
  "Zones target": 1,
  "Device settings": 1,
  "Connected devices (undocumented)": 1,
  "Event": 7,
  "User profile": 1,
  "Device info": 20,
  "File Id": 1,
  "File creator": 1
}
```
#### `messages` command

##### `messages` arguments
```text
Return messages defined by the -m parameter

Usage: rusty-but-fit --file <FILE> messages [OPTIONS]

Options:
  -m, --message_type <MESSAGE_TYPE>  Message types as enumerated from 'summary' command. Can be repeated for multiple messages.
  -h, --help                         Print help
```
##### Get messages of a certain type
```bash
rusty-but-fit -f activity2.fit messages -m "Activity"
```
```json
{
  "local_timestamp": 1078141854,
  "num_sessions": 1,
  "type": "Manual",
  "event": "Activity",
  "timestamp": 1078138254,
  "event_type": "Stop",
  "total_timer_time": 1717468
}
```

##### Read positional data from `Record` type
This might need some conversion from semicircle to degrees.
```bash
rusty-but-fit -f activity2.fit messages -m 'Record' | jq --argjson conversion "$((2**31))" '.[].message | {lon: (."position_long" * 180/$conversion), lat: (."position_lat" * 180/$conversion)}'
```
```json
...
{
  "lon": 10.557905668392777,
  "lat": 52.23186925984919
}
{
  "lon": 10.557780275121331,
  "lat": 52.231866996735334
}
{
  "lon": 10.557699054479599,
  "lat": 52.23186397925019
}
{
  "lon": 10.55741960182786,
  "lat": 52.231861716136336
}
...
```
