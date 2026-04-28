# `spangrep` - Grep, but for groups of lines.

Takes a “from” and “to” regex (both optional). Reads all files in the input,
when a line matches the from regex, it starts printing, and stops when a line matches
the to regex.

Some log files (e.g. PostgreSQL) use a variable number of lines per message.
This tool allows you to grep them.
