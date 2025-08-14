# statictranspl

`statictranspl` is a static transpiler for ewwii rhai. It uses the `stpl` file extension and is made to make configuring ewwii easier.

Just as the name suggests, it is a static transpiler and does **not** support complex things like if/else, for/while, variable definition, et cetera yet. It is made only for beginners to create a simple configuration quickly and easily.

## Usage

You use the `statictranspl` binary to transpile `.stpl` files to `.rhai` files. You use the **--transpile** argument to transpile `.stpl` files.

**Example use:**

```bash
# long form
statictranspl --transpile ./file1.stpl ./file2.stpl ./path/to/file3.stpl --out ./output_dir/

# short form
statictranspl -t ./file1.stpl ./file2.stpl ./path/to/file3.stpl -o ./output_dir/
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
