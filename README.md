> [!WARNING]
> This project has been deprecated!
>
> - See [XmlMan](https://github.com/Ewwii-sh/xmlman) which is a better alternative.

# StaticScript

`StaticScript` is a static transpiler for ewwii. It efficiently transpiles `sts` files to `rhai` files.

Just as the name suggests, it is a static transpiler and does **not** support complex things like if/else, for/while, variable definition, et cetera yet. It is made only for beginners to create a simple configuration quickly and easily.

## Installation

You can install StaticScript with ewwii's package manager, [eiipm](https://github.com/Ewwii-sh/eiipm).

```bash
eiipm i staticscript
```

This command will install the `stsc` binary which is the StaticScript Compiler.

## Usage

You use the `stsc` (staticscript compiler) binary to transpile `.sts` files to `.rhai` files. You use the **--transpile** argument to transpile `.sts` files.

**Example use:**

```bash
# long form
stsc --transpile ./file1.sts ./path/to/file2.sts --out ./output_dir/

# short form
stsc -t ./file1.sts ./path/to/file2.sts -o ./output_dir/
```

The transpiled code does not have whitespaces which may make it unredable. Staticscript provides a simple experimental formatter that follows the KISS (Keep It Simple Stupid) principle. You can make the transpiled code be formatted by using the **--format** (or **-f**) flag.

**Example:**

```bash
# long form
stsc --transpile ./file.sts--out ./output_dir/ --format

# short form
stsc -t ./file.sts -o ./output_dir/ -f
```

## Example

Here is a simple example of using sts:

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
