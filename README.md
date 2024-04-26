# RGB Controller CLI

## CLI Usage

### Install (with cargo)

```
cargo install --path="."
```

## Communication protocol

```
structure: |<channel>|<r>|<g>|<b>|
           |         |   |   |   |
bytes:     |   <1>   |<1>|<1>|<1>|
```