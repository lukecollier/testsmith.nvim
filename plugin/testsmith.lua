-- Testsmith Plugin Entry Point
-- This file is loaded by Neovim automatically

if vim.g.testsmith_loaded then
  return
end
vim.g.testsmith_loaded = true

local testsmith = require("testsmith")

-- Create user commands
vim.api.nvim_create_user_command("TestsmithFindOrCreate", function()
  testsmith.find_or_create_test()
end, { desc = "Find or create test file for current source" })

vim.api.nvim_create_user_command("TestsmithFind", function()
  testsmith.find_test()
end, { desc = "Find and open existing test file" })

vim.api.nvim_create_user_command("TestsmithPreview", function()
  testsmith.preview_test()
end, { desc = "Preview test file creation (dry-run)" })

vim.api.nvim_create_user_command("TestsmithFindVertical", function()
  testsmith.find_or_create_test({ split = "vertical" })
end, { desc = "Find or create test file in vertical split" })

vim.api.nvim_create_user_command("TestsmithFindHorizontal", function()
  testsmith.find_or_create_test({ split = "horizontal" })
end, { desc = "Find or create test file in horizontal split" })

vim.api.nvim_create_user_command("TestsmithFindTab", function()
  testsmith.find_or_create_test({ split = "tab" })
end, { desc = "Find or create test file in new tab" })

-- Auto-setup with defaults (can be overridden by user)
if not vim.g.testsmith_setup_done then
  testsmith.setup()
  vim.g.testsmith_setup_done = true
end
