local dap = require("dap")

local codelldb = vim.fn.expand("$HOME/.local/share/nvim/mason/bin/codelldb")

local project_name = vim.fn.fnamemodify(vim.fn.getcwd(), ":t")

dap.adapters.codelldb = {
  type = "server",
  port = "${port}",
  executable = {
    command = codelldb,
    args = { "--port", "${port}" },
  },
}

dap.configurations.rust = {
  {
    name = "Launch",
    type = "codelldb",
    request = "launch",
    program = function()
      os.execute("cargo build")
      return vim.fn.getcwd() .. "/target/debug/" .. project_name
    end,
    cwd = "${workspaceFolder}",
    stopOnEntry = false,
    runInTerminal = true,
    args = {},
  },
}
