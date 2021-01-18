# Window manager

## Theming

The main changes that are being made to the window manager are via themes. You can find the theme related code in the `themes` folder.

### Working with icons

We are creating icons in the `svg` file format and then exporting them to the window manager theme. Dark theme icons are made by exporting the light theme icons.

You can edit the icons by opening the svg in your editor of choice (we prefer inkscape). Then run the `themes/iconExport.sh` file to generate new icons. This requires you to have `gimp` installed on your computer.

## Melonager

System dependencies:

- An xorg server
- `xterm`
- `dmenu`

Dev dependencies:

- `xorg-server-xephyr`
- `entr`
- `inkscape` (icon editor)
- `gimp` (icon export script)
