-- Testsmith Neovim Plugin
-- Find or create test files for Java applications

local M = {}
local ffi_module = require("testsmith.ffi")

-- Default configuration
M.config = {
  -- Path to testsmith-nvim binary (can be overridden)
  binary = "testsmith-nvim",
  -- Default project structure
  structure = "maven",
  -- Whether to automatically open created test files
  open_on_create = true,
  -- Prefer FFI over CLI if available
  prefer_ffi = true,
  -- Keybindings (empty to disable)
  keybinds = {
    find_or_create = "<leader>tf",  -- Find or create test file
    find_or_create_vertical = "<leader>tv",  -- Open in vertical split
  },
}

--- Set up plugin configuration
---@param opts table User configuration options
function M.setup(opts)
  opts = opts or {}
  M.config = vim.tbl_deep_extend("force", M.config, opts)

  -- Set up keybindings
  if M.config.keybinds.find_or_create then
    vim.keymap.set(
      "n",
      M.config.keybinds.find_or_create,
      M.find_or_create_test,
      { noremap = true, silent = true, desc = "Testsmith: Find or create test file" }
    )
  end

  if M.config.keybinds.find_or_create_vertical then
    vim.keymap.set(
      "n",
      M.config.keybinds.find_or_create_vertical,
      function()
        M.find_or_create_test({ split = "vertical" })
      end,
      { noremap = true, silent = true, desc = "Testsmith: Find or create test file (vertical split)" }
    )
  end
end

--- Execute testsmith via FFI (if available) or CLI
---@param source_file string Path to source file
---@param opts table|nil Additional options (structure, framework="auto", dry_run, create)
---@return string, integer output, exit_code, boolean used_ffi
local function run_testsmith(source_file, opts)
  opts = opts or {}

  -- Auto-detect structure based on file type if not explicitly provided
  local structure = opts.structure or auto_detect_structure(source_file)

  -- Try FFI first if preferred and available
  if M.config.prefer_ffi and ffi_module.is_available() then
    local ffi_opts = {
      structure = structure,
      -- Use explicit framework if provided, otherwise "auto" for auto-detection
      framework = opts.framework or "auto",
      create = opts.create ~= false,
      dry_run = opts.dry_run or false,
    }

    local result = ffi_module.find_or_create(source_file, ffi_opts)

    if result.success then
      return result.message, 0, true
    else
      -- If FFI fails, fall through to CLI
      if result.message and result.message:find("not found") then
        -- Library not found, disable FFI for future calls
        vim.notify(
          "Testsmith library not found. Falling back to CLI.",
          vim.log.levels.WARN,
          { title = "Testsmith" }
        )
        M.config.prefer_ffi = false
      end
    end
  end

  -- Fall back to CLI
  local cmd = { M.config.binary, source_file }

  -- Always include auto-detected structure
  table.insert(cmd, "-s")
  table.insert(cmd, structure)

  if opts.language then
    table.insert(cmd, "-l")
    table.insert(cmd, opts.language)
  end

  if opts.framework then
    table.insert(cmd, "-f")
    table.insert(cmd, opts.framework)
  end

  if opts.dry_run then
    table.insert(cmd, "--dry-run")
  end

  local output = vim.fn.system(table.concat(cmd, " "))
  local exit_code = vim.v.shell_error

  return output, exit_code, false
end

--- Detect file extension and auto-select structure
---@param file_path string Path to source file
---@return string Structure type ("maven", "same-file", etc)
local function auto_detect_structure(file_path)
  -- Get file extension
  local ext = file_path:match("%.([^%.]+)$")

  -- Rust files use same-file structure
  if ext == "rs" then
    return "same-file"
  end

  -- Default to configured structure (usually maven for Java)
  return M.config.structure
end

--- Check if a file is a test file
---@param file_path string Path to file
---@return boolean, string|nil test_type Whether it's a test file and the pattern that matched
local function is_test_file(file_path)
  local file_name = file_path:match("([^/]+)$") or file_path

  -- Java test pattern: FooTest.java
  if file_name:match("Test%.java$") then
    return true, "java_suffix"
  end

  -- Rust test pattern: foo_test.rs
  if file_name:match("_test%.rs$") then
    return true, "rust_suffix"
  end

  return false, nil
end

--- Find the corresponding source file for a test file
---@param test_file string Path to test file
---@param test_type string Type of test pattern detected
---@return string|nil source_file Path to source file (nil if not found)
local function find_source_for_test(test_file, test_type)
  local file_name = test_file:match("([^/]+)$") or test_file
  local dir = test_file:sub(1, #test_file - #file_name)

  if test_type == "java_suffix" then
    -- FooTest.java → Foo.java (in src/main structure)
    local source_name = file_name:gsub("Test%.java$", ".java")

    -- Look in Maven structure (src/main/java)
    local src_main = test_file:gsub("src/test/java", "src/main/java")
    src_main = src_main:gsub("Test%.java$", ".java")

    if vim.fn.filereadable(src_main) == 1 then
      return src_main
    end

    -- Also look in same directory
    local same_dir = dir .. source_name
    if vim.fn.filereadable(same_dir) == 1 then
      return same_dir
    end
  elseif test_type == "rust_suffix" then
    -- foo_test.rs → foo.rs
    local source_name = file_name:gsub("_test%.rs$", ".rs")
    local same_dir = dir .. source_name

    if vim.fn.filereadable(same_dir) == 1 then
      return same_dir
    end
  end

  return nil
end

--- Extract test file path from testsmith output
---@param output string Output from testsmith CLI
---@return string|nil test_file_path
local function extract_test_path(output)
  -- Look for patterns like "Created test file: /path/to/File.java"
  -- or "Found test file: /path/to/File.java"
  local match = output:match("([^:]+%.java)%s*$") or output:match("([^:]+%.java)")
  return match
end

--- Find or create test file and open it
---@param opts table|nil Options (split: "vertical"|"horizontal"|"tab")
function M.find_or_create_test(opts)
  opts = opts or {}

  local current_file = vim.fn.expand("%:p")

  if current_file == "" then
    vim.notify("No file open", vim.log.levels.WARN)
    return
  end

  -- Check if it's a directory (not a file)
  if vim.fn.isdirectory(current_file) == 1 then
    vim.notify("Cannot run testsmith on a directory - please open a file", vim.log.levels.WARN)
    return
  end

  -- Check if current file is a test file and find corresponding source
  local is_test, test_type = is_test_file(current_file)
  if is_test then
    local source_file = find_source_for_test(current_file, test_type)
    if source_file then
      -- Open the source file instead
      local open_cmd = "edit"
      if opts.split == "vertical" then
        open_cmd = "vsplit"
      elseif opts.split == "horizontal" then
        open_cmd = "split"
      elseif opts.split == "tab" then
        open_cmd = "tabnew"
      end

      vim.cmd(open_cmd .. " " .. vim.fn.fnameescape(source_file))
      return
    else
      vim.notify("Could not find source file for: " .. current_file, vim.log.levels.WARN, { title = "Testsmith" })
      return
    end
  end

  -- Try FFI first (synchronous, very fast)
  if M.config.prefer_ffi and ffi_module.is_available() then
    vim.schedule(function()
      local ffi_opts = {
        structure = auto_detect_structure(current_file),
        create = true,
      }

      local result = ffi_module.find_or_create(current_file, ffi_opts)

      if result.success then
        local test_file = result.message
        local open_cmd = "edit"

        if opts.split == "vertical" then
          open_cmd = "vsplit"
        elseif opts.split == "horizontal" then
          open_cmd = "split"
        elseif opts.split == "tab" then
          open_cmd = "tabnew"
        end

        if M.config.open_on_create or not result.created then
          vim.cmd(open_cmd .. " " .. vim.fn.fnameescape(test_file))
          -- Set cursor to the TODO line if available
          if result.line_number and result.line_number > 0 then
            local win = vim.api.nvim_get_current_win()
            vim.api.nvim_win_set_cursor(win, { result.line_number, 0 })
          end
        end
      else
        vim.notify("Error: " .. result.message, vim.log.levels.ERROR, { title = "Testsmith" })
      end
    end)
    return
  end

  -- Fall back to CLI (asynchronous)
  vim.fn.jobstart(M.config.binary .. " " .. current_file, {
    on_stdout = function(_, data)
      local output = table.concat(data, "\n")

      vim.schedule(function()
        local test_file = extract_test_path(output)

        if test_file then
          -- Determine how to open the file
          local open_cmd = "edit"
          if opts.split == "vertical" then
            open_cmd = "vsplit"
          elseif opts.split == "horizontal" then
            open_cmd = "split"
          elseif opts.split == "tab" then
            open_cmd = "tabnew"
          end

          if M.config.open_on_create or output:find("Found") then
            vim.cmd(open_cmd .. " " .. vim.fn.fnameescape(test_file))
          end
        else
          vim.notify("No test file path found", vim.log.levels.ERROR, { title = "Testsmith" })
        end
      end)
    end,

    on_stderr = function(_, data)
      local error_msg = table.concat(data, "\n")
      vim.schedule(function()
        vim.notify("Error: " .. error_msg, vim.log.levels.ERROR, { title = "Testsmith" })
      end)
    end,

    on_exit = function(_, exit_code)
      if exit_code ~= 0 then
        vim.schedule(function()
          vim.notify("Testsmith exited with code " .. exit_code, vim.log.levels.ERROR, { title = "Testsmith" })
        end)
      end
    end,
  })
end

--- Open test file for current source file (without creating)
function M.find_test()
  local current_file = vim.fn.expand("%:p")

  if current_file == "" then
    vim.notify("No file open", vim.log.levels.WARN)
    return
  end

  if vim.fn.isdirectory(current_file) == 1 then
    vim.notify("Cannot run testsmith on a directory - please open a file", vim.log.levels.WARN)
    return
  end

  -- Check if current file is a test file
  local is_test, test_type = is_test_file(current_file)
  if is_test then
    local source_file = find_source_for_test(current_file, test_type)
    if source_file then
      vim.cmd("edit " .. vim.fn.fnameescape(source_file))
    else
      vim.notify("Could not find source file for: " .. current_file, vim.log.levels.WARN, { title = "Testsmith" })
    end
    return
  end

  local output, exit_code = run_testsmith(current_file, {})

  if exit_code == 0 then
    local test_file = extract_test_path(output)
    if test_file then
      vim.cmd("edit " .. vim.fn.fnameescape(test_file))
    end
  else
    vim.notify("Test file not found", vim.log.levels.WARN, { title = "Testsmith" })
  end
end

--- Create test file (preview what would be created)
function M.preview_test()
  local current_file = vim.fn.expand("%:p")

  if current_file == "" then
    vim.notify("No file open", vim.log.levels.WARN)
    return
  end

  if vim.fn.isdirectory(current_file) == 1 then
    vim.notify("Cannot run testsmith on a directory - please open a file", vim.log.levels.WARN)
    return
  end

  -- Check if current file is a test file
  local is_test, test_type = is_test_file(current_file)
  if is_test then
    vim.notify("This is already a test file - nothing to preview", vim.log.levels.INFO, { title = "Testsmith" })
    return
  end

  local output, exit_code = run_testsmith(current_file, {
    dry_run = true,
  })

  if exit_code == 0 then
    vim.notify(output, vim.log.levels.INFO, { title = "Testsmith (Dry Run)" })
  else
    vim.notify("Error: " .. output, vim.log.levels.ERROR, { title = "Testsmith" })
  end
end

return M
