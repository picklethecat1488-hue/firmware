# Harmless no-op `from` user command preventing Python REPL debug-tracker extensions
# (e.g. OcpCadViewer) from emitting `Undefined command: "from". Try "help".` in GDB.
define from
end
