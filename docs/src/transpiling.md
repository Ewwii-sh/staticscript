# Transpiling

Once you have written your configuration in `stpl`, you can transpile it to `rhai` with the **--transpile** flag.

**Usage example:**

```bash
# long form
statictranspl --transpile ./path/to/file.stpl

# short form
statictranspl -t ./path/to/file.stpl
```

Other than providing just one `stpl` file, you can provide multiple `stpl` files as well! Just add them as well to the command.

**Example:**

```bash
# NOTE: you can also use the long form of `-t` which is `--transpile`
statictranspl -t ./path/to/file.stpl ./path/to/file2.stpl
```

The transpile command outputs the transpiled `.rhai` files in the current directory which the command is ran. If you want to specify a specific directory which you want the `.rhai` files to appear, then you can add the **--out** flag (or **-o** flag in short).

**Example:**

```bash
statictranspl -t ./path/to/file.stpl -o ./output_dir/
```
