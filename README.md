# Puke Rainbows
Dumb and useless CLI app which takes a text and pukes it like a beautiful gnome would.

![Gnome puking](https://media.tenor.com/IAR8RQwY3UoAAAAM/vomit-gnome.gif)

## Usage
```
Puke Rainbows (puke-rainbows)

Usage:
    puke-rainbows -help
    puke-rainbows -text <text> [-save]
    puke-rainbows -file <file> [-save]

Options:
    -help   Show this screen.
    -text   Print a given text.
    -file   Print a given file.
    -save   Save ANSI output as a file named "puke.txt" in the current directory.

Examples:
    puke-rainbows -help
    puke-rainbows -text Example
    puke-rainbows -text "Hello there, this is an example text" -save
    puke-rainbows -file lorem-ipsum
    puke-rainbows -file /path/to/myfile -save
```

## Actions
Testing done on a clone.

### build
Runs when there is a push to the main branch or manually in the GitHub UI.

Checks that code compiles, passes tests and builds binaries.

### create-release
Runs manually in the GitHub UI.

Checks if there is a tag with the current app version (from "Cargo.toml" file). If there isn't, continues.

Builds binaries and uploads them as artifacts.

Creates a tag and a release, and attaches the previously generated artifacts to the release.

## Notes
RGB range is 1-255. Idea: use modulo operation (and add other math changes) to clamp between 0 and 255 (% 256), inclusive.
I don't think that it matters very much.

## Resources
https://flaviocopes.com/go-tutorial-lolcat/

https://rust-cli.github.io/book/index.html
