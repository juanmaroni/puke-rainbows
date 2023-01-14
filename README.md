# Puke Rainbows
Dumb and useless CLI app which takes a text and pukes it like a beautiful gnome would.

![Gnome puking](https://media.tenor.com/IAR8RQwY3UoAAAAM/vomit-gnome.gif)

## Usage
```
Puke Rainbows (puke-rainbows)

Usage:
    puke-rainbows -h
    puke-rainbows -t <text>
    puke-rainbows -f <file>

Options:
    -h      Show this screen.
    -t      Print a given text.
    -f      Print a given file.

Examples:
    puke-rainbows -h
    puke-rainbows -t Example
    puke-rainbows -t "Hello there, this is an example text"
    puke-rainbows -f lorem-ipsum
    puke-rainbows -f /path/to/myfile
```

## Notes
RGB range is 1-255. Idea: use modulo operation (and add other math changes) to clamp between 0 and 255 (% 256), inclusive.
I don't think that it matters very much.

## Resources
https://flaviocopes.com/go-tutorial-lolcat/

https://rust-cli.github.io/book/index.html
