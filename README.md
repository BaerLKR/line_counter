# line_counter
A simple program for counting lines in a project directory

## Basic Usage
By running the program with

    lc <FILE_PATH> 

it will count every line in given file. This also works for directories.

If you want to run the program recursively to count files in subdirectories, run it with the **-r** flag:

    lc -r

You can also provide the **-s** flag if you want empty lines to be ignored:

    lc -s

If you provide the **-c** flag it will count all characters excluding newline and tab characters:

    lc -c
