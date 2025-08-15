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
# long form
statictranspl -t ./path/to/file.stpl --out ./output_dir/

# short form
statictranspl -t ./path/to/file.stpl -o ./output_dir/
```

The transpiled code is unformatted by default and does not have whitespaces. Statictranspl provides a simple experimental formatter that follows the KISS (Keep It Simple Stupid) principle. You can make the transpiled code be formatted before writing by using the **--format** (or **-f**) flag.

**Example:**

```bash
# long form
statictranspl -t ./path/to/file.stpl -o ./output_dir/ --format

# short form
statictranspl -t ./path/to/file.stpl -o ./output_dir/ -f
```
