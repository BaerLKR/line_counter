# line_counter
A simple program for counting lines in a project directory

## Basic Usage
By just running the program with

    lc

it will count every line in the directory.

If you want to run the program recursively for the subdirectories, run it with the **-r** flag:

    lc -r

In order to count lines in a directory you need to specify a path:

    lc <FILE_PATH>
    
You can also provide the **-s** flag if you want empty lines to be ignored:

    lc -s

