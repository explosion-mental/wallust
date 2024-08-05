# Template Variables

## The _"color"_ type
These types are formated like as HEX rgb (e.g. '#0A0B0C') by default. However a
**color literal** can be represented in multiple ways, like HEXA rgba (e.g.
'`#0A0B0CFF`', where '`FF`' is the transparency value) or HEX rgb without the
leading '`#`' ('`0a0b0c`').

Avaliable values:

- **color0**
- **color1**
- **color2**
- **color3**
- **color4**
- **color5**
- **color6**
- **color7**
- **color8**
- **color9**
- **color10**
- **color11**
- **color12**
- **color13**
- **color14**
- **color15**
- **background**
- **foreground**
- **cursor**


### **colors**

Additionally, this variable (`colors`) returns a **vector** of all the presented colors in the following order:

Starts with **color0** to **color15**, **background**, **foreground** and at the end, (index 18 if starting from 0), **cursor**.


## MISCELLANEOUS
Other avaliable variables:

### wallpaper
The full path to the current wallpaper, colorscheme file or the name of the theme in use.

### backend
Current backend being used.

### colorspace
Current **colorspace** being used.

### palette
Current **palette** being used.

### alpha
Default to 100, can be modified in the config file or with `--alpha`/`-a`.

### alpha_dec
Instead of 0 to 100, displays it from 0.00 to 1.00.

