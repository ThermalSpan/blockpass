# blockpass

## What does it do
This tool takes an input file, and its looks for deliminated block at the top of that file (usually YAML) and then will append that to the output file. 

### Why 
I like [pandoc](https://pandoc.org/), but I ran into a situation where it didn't do exactly what I wanted. Until I can figure out how to use pandoc better, blockpass is my stop gap measure. The issue is this: I am using pandoc to transform markdown into html for [posts on my website](https://github.com/ThermalSpan/pandoc-posts). However those html documents are then consumed by a templater. Pacdoc supports YAML metadata in markdown files, but doesn't (to the best of my knowledge) support passing those blocks through to the final product. Hence this tool which will take the block from the input file and pass it to the output file. 

### Example

    russell$ cat a.md
    ---
    title: Markdown Post
    extends: default.liquid
    ---
    Look at me! I'm some example text.

    russell$ pandoc -o b.html a.md
    
    russell$ cat b.html
    <p>Look at me! I'm some example text.</p>

    russell$ blockpass --input a.md --output b.html

    russell$ cat b.html
    ---
    title: Markdown Post
    extends: default.liquid
    ---
    <p>Look at me! I'm some example text.</p>k
