# statictranspl

`statictranspl` is a static transpiler for the custom language named `stpl`. It efficiently transpiles `stpl` files to `rhai` files.

Just as the name suggests, it is a static transpiler and does **not** support complex things like if/else, for/while, variable definition, et cetera yet. It is made only for beginners to create a simple configuration quickly and easily.

## Installation

You can install statictranspl with ewwii's package manager, [eiipm](https://github.com/Ewwii-sh/eiipm).

```bash
eiipm i statictranspl
```

This command will install the `stplc` binary which is the statictranspl compiler.

## Usage

You use the `stplc` (statictranspl compiler) binary to transpile `.stpl` files to `.rhai` files. You use the **--transpile** argument to transpile `.stpl` files.

**Example use:**

```bash
# long form
stplc --transpile ./file1.stpl ./path/to/file2.stpl --out ./output_dir/

# short form
stplc -t ./file1.stpl ./path/to/file2.stpl -o ./output_dir/
```

The transpiled code does not have whitespaces which may make it unredable. Statictranspl provides a simple experimental formatter that follows the KISS (Keep It Simple Stupid) principle. You can make the transpiled code be formatted by using the **--format** (or **-f**) flag.

**Example:**

```bash
# long form
stplc --transpile ./file.stpl--out ./output_dir/ --format

# short form
stplc -t ./file.stpl -o ./output_dir/ -f
```

## Example

Here is a simple example of using stpl:

```ruby
widget "widget1" {
  box {
    class: "widget1"
    orientation: "h"
    space_evenly: true
    hexpand: true
    spacing: 5
  } {
      label {
        text: "Hello Ewwii!"
      }

      slider {
        min: 0
        max: 101
        value: volume
        onchange: "echo hi! you just triggered the slider"
        dyn_id: "vol"
      }

      button {
        text: "greet"
        onclick: "notify-send 'hello there!'"
      }

      checkbox {
        class: "chckbox"
      }
  }
}

window "main_window" {
  monitor: 0
  windowtype: "dock"
  geometry: { x: 0 y: 0 width: 10 height: 20 }
} "widget1"
```
