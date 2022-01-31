# RGB Controller CLI

## CLI Usage

```
rgb ip:show
rgb ip:set <ip>

rgb set <r> <g> <b>
rgb set <preset>

rgb preset:add <preset-name> <r> <g> <b>
rgb preset:remove <preset-name>
rgb preset:list

rgb default:show
rgb default:set <device-id>
```

## Communication protocol

### Basic structure

```
structure: |<device-id>|<command>|<r>|<g>|<b>|
           |           |         |   |   |   |
length:    |    <2>    |   <1>   |<3>|<3>|<3>|
```

_(device id 00 means all devices)_

example: set color R: 255, G: 255, B: 255 on device 01

```
01C255255255
```
