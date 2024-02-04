# A simple, local D&D stat tracker.

Clone of [this]([https://slint.dev/](https://github.com/Maerorr/dnd-stat-tracker)) but with Slint UI toolkit (https://slint.dev/).

It's built to work as a maximized application.

TODO:
- [X] - character basic info (name, class, exp) + editable
- [X] - ability scores + editable
- [X] - saving throws and skill modifiers, proficiencies and expertise + editable
- [X] - hitpoints, AC, temporary hitpoints, hit dice etc. + editable
- [X] - money, including a converter
- [ ] - equipement as text
- [ ] - weapons and hit/dmg modifiers
- [X] - spell list + spell slots
- [X] - more spells as json files
- [ ] - local character saves
- [ ] - refactor how callbacks are handled into pure callbacks from weird chains on callbacks upstream

WIP screenshot: ![image](images/stattrackerslint.png)

All spells are from [here](https://github.com/jcquinlan/dnd-spells), reformatted for my own use.


## License

MIT License

Copyright (c) <2024> <Hubert Łabuda>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
