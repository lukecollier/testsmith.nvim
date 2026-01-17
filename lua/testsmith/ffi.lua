-- Lua FFI bindings for Testsmith
-- Allows calling Rust code directly without subprocess overhead

local ffi = require("ffi")
local M = {}

-- FFI type definitions
ffi.cdef[[
  typedef struct {
    int success;
    char* message;
    int created;
    int line_number;
  } TestsmithResult;

  // Find or create test file (language auto-detected from source_path)
  TestsmithResult* testsmith_find_or_create(
    const char* source_path,
    const char* structure,
    const char* framework,
    int create,
    int dry_run
  );

  // Free result
  void testsmith_result_free(TestsmithResult* result);
]]

-- Try to load the shared library
local lib = nil
local lib_loaded = false

-- Detect platform and architecture
local function get_platform_and_arch()
  local uname_s = vim.fn.system("uname -s"):gsub("\n", "")
  local uname_m = vim.fn.system("uname -m"):gsub("\n", "")

  if uname_s == "Darwin" then
    -- macOS
    if uname_m == "arm64" then
      return "macos-arm64", "libtestsmith_nvim.dylib"
    else
      return "macos-x86_64", "libtestsmith_nvim.dylib"
    end
  elseif uname_s == "Linux" then
    return "linux-x86_64", "libtestsmith_nvim.so"
  elseif uname_s:match("^MINGW") or uname_s:match("^MSYS") then
    return "windows-x86_64", "testsmith_nvim.dll"
  end

  return nil, nil
end

-- Try to find and load the library
local function load_library()
  if lib_loaded then
    return lib ~= nil
  end

  lib_loaded = true

  -- Get the plugin directory (for fallback paths)
  local plugin_dir = vim.fn.fnamemodify(debug.getinfo(1).source:sub(2), ":h:h:h")

  -- First: Try system/standard library locations (for development and system-installed versions)
  -- This is preferred for easier testing - just run `cargo build --release`
  local lib_names = {
    "libtestsmith_nvim.so",     -- Linux
    "libtestsmith_nvim.dylib",  -- macOS
    "testsmith_nvim.dll",       -- Windows
  }

  for _, name in ipairs(lib_names) do
    local ok, result = pcall(function()
      return ffi.load(name)
    end)
    if ok then
      lib = result
      return true
    end
  end

  -- Second: Try development build directory (target/release/)
  if plugin_dir then
    local target, lib_name = get_platform_and_arch()
    if target and lib_name then
      local dev_path = plugin_dir .. "/target/release/" .. lib_name
      local ok, result = pcall(function()
        return ffi.load(dev_path)
      end)
      if ok then
        lib = result
        return true
      end
    end
  end

  -- Third: Try bundled lib directory (for plugin distribution)
  if plugin_dir then
    local platform, lib_name = get_platform_and_arch()
    if platform and lib_name then
      local lib_path = plugin_dir .. "/lib/" .. platform .. "/" .. lib_name
      local ok, result = pcall(function()
        return ffi.load(lib_path)
      end)
      if ok then
        lib = result
        return true
      end
    end
  end

  return false
end

--- Find or create test file via FFI
---@param source_path string Path to source file (language auto-detected from extension)
---@param opts table Options: structure, framework ("auto" for auto-detection or explicit framework), create, dry_run
---@return table Result with fields: success, message, created, line_number
function M.find_or_create(source_path, opts)
  opts = opts or {}

  if not load_library() then
    return {
      success = false,
      message = "Testsmith library not found. Make sure libtestsmith_nvim is built and accessible.",
    }
  end

  local structure = opts.structure or "maven"
  -- framework should always be provided - use "auto" for auto-detection from project config
  local framework = opts.framework or "auto"
  local create = opts.create ~= false and 1 or 0
  local dry_run = opts.dry_run and 1 or 0

  -- Call the FFI function (language auto-detected from source_path, framework auto-detected if "auto" is specified)
  local result = lib.testsmith_find_or_create(
    source_path,
    structure,
    framework,
    create,
    dry_run
  )

  if result == nil then
    return {
      success = false,
      message = "FFI call failed",
    }
  end

  -- Convert result to Lua table
  local message = ffi.string(result.message)
  local success = result.success ~= 0
  local created = result.created ~= 0
  local line_number = result.line_number

  -- Free the result
  lib.testsmith_result_free(result)

  return {
    success = success,
    message = message,
    created = created,
    line_number = line_number,
  }
end

--- Check if FFI is available
---@return boolean
function M.is_available()
  return load_library()
end

return M
