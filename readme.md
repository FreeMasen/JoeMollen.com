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

One quick way to get a bunch of this out of the way would be to use the program `build_site` in the folder. You first want to open the app `terminal`. You this will open with your current directory set to your main user folder (this is the parent folder to Documents). You want to move into the folder that was generated when you unzipped the download, you can do that by typing the following.

```bash
$ cd Downloads/jm
```
*assuming that it ended up in your downloads folder, it may be in `Desktop` or `Documents`.

Once there you can type `build_site`, this will give you a nice help message with some directions. There are 5 different options you have.

1. `setup` - This will setup all of the directories for you, even creating a sample project folder.
2. `build` - This will actually build your site, by default it looks for the folder `input` and outputs the whole site to `www`.
3. `add` - This will create a new project folder with the required files.
4. `layout` - This will print out both the input and output directory structures.
5. `help` - this will print the same message as just typing build_site.


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

