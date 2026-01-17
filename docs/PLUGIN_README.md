# Testsmith Neovim Plugin

A Neovim plugin to find or create test files for Java applications using the testsmith CLI tool.

## Features

- 🔍 Find existing test files in your project
- ✨ Automatically create test files with JUnit boilerplate
- 🎯 Supports Maven project structures (extensible)
- 🔌 Easy-to-use commands and keybindings
- ⚡ Async execution (doesn't block the editor)
- 🔄 Multiple split modes (vertical, horizontal, tab)

## Installation

### Using a Plugin Manager

**lazy.nvim:**
```lua
{
  "manuel-woelker/testsmith.nvim",
  config = function()
    require("testsmith").setup({
      binary = "testsmith-nvim",
      structure = "maven",
      open_on_create = true,
      keybinds = {
        find_or_create = "<leader>tf",
        find_or_create_vertical = "<leader>tv",
      },
    })
  end,
}
```

**packer.nvim:**
```lua
use {
  "manuel-woelker/testsmith.nvim",
  config = function()
    require("testsmith").setup()
  end,
}
```

**vim-plug:**
```vim
Plug 'manuel-woelker/testsmith.nvim'

" In your init.vim:
lua require("testsmith").setup()
```

## Usage

### Commands

- `:TestsmithFindOrCreate` - Find existing test file or create new one
- `:TestsmithFind` - Only find existing test file
- `:TestsmithPreview` - Preview what would be created (dry-run)
- `:TestsmithFindVertical` - Find/create and open in vertical split
- `:TestsmithFindHorizontal` - Find/create and open in horizontal split
- `:TestsmithFindTab` - Find/create and open in new tab

### Keybindings

Default keybindings (can be customized):

- `<leader>tf` - Find or create test file
- `<leader>tv` - Find or create test file in vertical split

### Lua API

```lua
local testsmith = require("testsmith")

-- Configure plugin
testsmith.setup({
  binary = "testsmith-nvim",      -- Path to testsmith binary
  structure = "maven",             -- Project structure type
  open_on_create = true,           -- Open file after creation
  keybinds = {
    find_or_create = "<leader>tf",
    find_or_create_vertical = "<leader>tv",
  },
})

-- Find or create test file
testsmith.find_or_create_test()

-- Find or create with options
testsmith.find_or_create_test({
  split = "vertical",  -- "vertical", "horizontal", "tab"
})

-- Only find existing test file
testsmith.find_test()

-- Preview test creation (dry-run)
testsmith.preview_test()
```

## Configuration

### Default Configuration

```lua
require("testsmith").setup({
  -- Path to testsmith-nvim binary (if in PATH, just use "testsmith-nvim")
  binary = "testsmith-nvim",

  -- Default project structure
  structure = "maven",  -- "maven", "gradle", "flat", "same-file"

  -- Whether to automatically open created test files
  open_on_create = true,

  -- Keybindings (set to "" to disable)
  keybinds = {
    find_or_create = "<leader>tf",
    find_or_create_vertical = "<leader>tv",
  },
})
```

## Examples

### Example 1: Basic Setup

```lua
-- init.lua
require("testsmith").setup()

-- Now use <leader>tf to find/create test files
```

### Example 2: Custom Keybindings

```lua
require("testsmith").setup({
  keybinds = {
    find_or_create = "<leader>tt",
    find_or_create_vertical = "<leader>ts",
  },
})
```

### Example 3: Disable Keybindings

```lua
require("testsmith").setup({
  keybinds = {
    find_or_create = "",
    find_or_create_vertical = "",
  },
})

-- Use commands instead
vim.keymap.set("n", "<leader>tf", ":TestsmithFindOrCreate<CR>")
```

### Example 4: Custom Configuration via Autocmd

```lua
local testsmith = require("testsmith")

-- Only auto-open test files, don't use keybindings
testsmith.setup({
  open_on_create = true,
  keybinds = { find_or_create = "", find_or_create_vertical = "" },
})

-- Set up custom keybindings for specific file types
vim.api.nvim_create_autocmd("FileType", {
  pattern = "java",
  callback = function()
    vim.keymap.set("n", "<leader>tf", testsmith.find_or_create_test, { buffer = true })
  end,
})
```

## How It Works

1. **Get current file**: The plugin gets the path to the currently open file
2. **Run testsmith CLI**: Calls the `testsmith-nvim` binary as a subprocess
3. **Extract result**: Parses the output to find the test file path
4. **Open file**: Opens the test file in Neovim (or splits/tabs based on options)

## Requirements

- Neovim 0.8.0+
- `testsmith-nvim` CLI tool (compiled Rust binary)

## Troubleshooting

### Command not found: "testsmith-nvim"

Make sure the testsmith binary is in your PATH or configure the full path:

```lua
require("testsmith").setup({
  binary = "/full/path/to/testsmith-nvim",
})
```

### Test file not created

- Verify your source file is in a valid Maven project structure (`src/main/java/...`)
- Check the `:TestsmithPreview` command to see what would happen
- Ensure testsmith-nvim binary has the correct permissions

### Split not working

Make sure Neovim is properly configured for splits. The plugin uses standard Neovim split commands.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

Same as testsmith (check main repository)

## Related

- [testsmith](https://github.com/manuel-woelker/testsmith.nvim) - The core Rust CLI tool
