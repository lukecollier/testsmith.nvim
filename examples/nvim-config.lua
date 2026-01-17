-- Example Neovim Configuration with Testsmith Plugin
-- Copy this into your init.lua or neovim config

-- If using lazy.nvim, add to your plugins spec:
{
  "manuel-woelker/testsmith.nvim",
  config = function()
    require("testsmith").setup({
      -- Path to testsmith-nvim binary
      -- If it's in your PATH, just use "testsmith-nvim"
      -- Otherwise, provide full path: "/usr/local/bin/testsmith-nvim"
      binary = "testsmith-nvim",

      -- Default project structure (can be: maven, gradle, flat, same-file)
      structure = "maven",

      -- Whether to open the test file after creation
      open_on_create = true,

      -- Keybindings for quick access
      keybinds = {
        find_or_create = "<leader>tf",       -- Find or create test file
        find_or_create_vertical = "<leader>tv", -- Open in vertical split
      },
    })
  end,
}

-- Or if using packer.nvim:
--[[
use {
  "manuel-woelker/testsmith.nvim",
  config = function()
    require("testsmith").setup()
  end,
}
]]

-- Manual setup (if not using a plugin manager):
--[[
require("testsmith").setup({
  binary = "testsmith-nvim",
  structure = "maven",
  open_on_create = true,
  keybinds = {
    find_or_create = "<leader>tf",
    find_or_create_vertical = "<leader>tv",
  },
})
]]

-- Additional keybindings after setup:
local testsmith = require("testsmith")

-- Find/create with different split modes
vim.keymap.set("n", "<leader>tfs", function()
  testsmith.find_or_create_test({ split = "horizontal" })
end, { noremap = true, silent = true, desc = "Testsmith: Horizontal split" })

vim.keymap.set("n", "<leader>tft", function()
  testsmith.find_or_create_test({ split = "tab" })
end, { noremap = true, silent = true, desc = "Testsmith: New tab" })

-- Preview what would be created
vim.keymap.set("n", "<leader>tp", function()
  testsmith.preview_test()
end, { noremap = true, silent = true, desc = "Testsmith: Preview" })

-- Find only (don't create)
vim.keymap.set("n", "<leader>to", function()
  testsmith.find_test()
end, { noremap = true, silent = true, desc = "Testsmith: Find only" })
