# Data Attribution

The emoji data in this directory is sourced from:

**Emoji-List-Unicode**
- Repository: https://github.com/Tarikul-Islam-Anik/Emoji-List-Unicode
- License: MIT
- Data Source: Unicode official website (https://unicode.org/emoji/charts/full-emoji-list.html)

## Files:

- `all-emoji.json` - Contains all standard Unicode emojis
- `full-emoji-modifiers.json` - Contains emojis with skin tone modifiers

These files are used at compile time by the build script to generate a static list of emoji patterns that are embedded directly into the binary.