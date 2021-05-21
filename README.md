# font2img

A tool for converting TTF icon font to images.

## Install

### MacOS
```bash
brew tap kofj
```

## Help

```bash
# font2img v1.0.0
Fanjiankong <kfanjian@gmail.com>
A tool for converting TTF icon font to images.

example: 
        font2png --charter $(printf '\ue957') -s 80 -f a -o src/assets/on/user.png -c "#d43c33"

USAGE:
    font2png [FLAGS] [OPTIONS] --charter <charter> --color <color> --font <fontpath> --output <output>

FLAGS:
    -h, --help           Prints help information
    -t, --transparent    transparent background
    -V, --version        Prints version information

OPTIONS:
        --charter <charter>    icon charter
    -c, --color <color>        icon css style color
    -f, --font <fontpath>      font file path
    -o, --output <output>      output filename
    -s, --size <size>          output image's height and width(pixel) [default: 120]
```

## TODO
- [ ] Enble/disable transparent background.
- [ ] Custom background color.
- [ ] Batch convert cmap list.
- [ ] Custom batch convert.
