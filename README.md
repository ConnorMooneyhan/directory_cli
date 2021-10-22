# directory_cli
This is a simple CLI designed to create and manage a phone directory on the user's local file system. It will create/read a 'directory.json' file in the same folder as the executable, read it to a HashMap, and allow the user to perform CRUD (Create, Read, Update, Delete) operations from the command line.

## Available Commands
*search [name (can be partial)]
*reverse-search [number (can be partial)]
*add [first name] [last name] [phone number w/ no spaces]
*edit [first name] [last name] [phone number w/ no spaces]
*delete [name/part of name]