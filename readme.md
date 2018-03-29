# Directions

Download the program from [the release page](https://github.com/FreeMasen/release)
## Getting Set Up

Create your site's folder with the following layout

```bash
/
├─ jm
└─ input
    ├─ portfolio
    │     └─ [project name] (repeated)
    │           ├─ img
    │           │   └─ [image] (repeated)
    │           ├─ content.md
    │           └─ meta.toml
    ├─ templates
    │     ├─ about.html
    │     ├─ base.html
    │     ├─ contact.html
    │     ├─ index.html
    │     └─ page.html
    ├─ about.md
    └─ joe.jpg [image file]
```

Each project folder in the portfolio foler will create an entry on the home page for the site. This will use the first image listed (alphabetically) in the `img` folder. The content will be generated from the other two files in that folder `content.md` and `meta.toml`.

### content.md

This is a `markdown` file, it allows for generating HTML from a more user friendly sintax, [this cheatsheet](http://commonmark.org/help/) is a good place to get started. The content here will fill in the left section of each project page.

### meta.toml

This file will provide some meta information about each of the projects in the portfolio and will generate the header content for the left portion of each project page. It needs to have the following format.

```toml
title = "Project Title"
context = "Project Sub-Title"
teammates = [
    "person 1",
    "person 2"
]
```

Each of these 3 key/value pairs is required, if there are no teammates for a project include empty brackets (`[]`).

### templates
> note: I am hoping to get these zipped up with the programm for future releases

This folder will include the templates that generate your site. each of the `html` files are required for this to work.

### about.md

Just like the content.md file in each of the project folders this `markdown` file will provide the content of the about page.

### joe.jpg
> Note currently this needs to be a `jpg` file but I might try and make this more flexable.

This file will act as the image placed on the about page.

## Using the program

Now that you have your folder setup, you should be able to just double click the `jm` file and the site will generate into a folder titled `www`. Once you have your hosting provider setup, you can upload this folder according to their directions.

## output
The output folder will be structed like this.
```
/
├─ portfolio
│     └─ [project name] (repeated)
│           ├─ img 
│           │   └─ [image] (repeated)
│           ├─ index.html
├─ about
│     └─index.html
├─ contact
│     └─index.html
└─ index.html
```

### Advanced

You do have the option to provide both an input and output folder to the program, to do so you would need to open an `terminal` and type the following.

```bash
$ jm -i=/path/to/input -o=/path/to/output
```

