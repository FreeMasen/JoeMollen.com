# Directions

Download the program from [the release page](https://github.com/FreeMasen/JoeMollen.com/releases), you only need the file `jm.tar.gz`, be sure to select the most recent version.
## Getting Set Up

Create your site's folder with the following layout, read on in [Using the program](#using-the-program) for a way to do this quickly.

```
/
├─ build (included in .tar.gz file)
├─ build_site (included in .tar.gz file)
└─ input (included in .tar.gz file)
    ├─ portfolio
    │     └─ [project name] (repeated)
    │           ├─ img
    │           │   └─ [image] (repeated)
    │           ├─ content.md
    │           └─ meta.toml
    ├─ templates (included in .tar.gz file)
    │     ├─ about.html
    │     ├─ base.html
    │     ├─ contact.html
    │     ├─ index.html
    │     └─ page.html
    ├─ about.md
    └─ joe.jpg [image file]
```

Each project folder in the portfolio folder will create an entry on the home page for the site. This will use the first image listed (alphabetically) in the `img` folder. The content will be generated from the other two files in that folder `content.md` and `meta.toml`.

### content

This is a `markdown` file, it allows for generating HTML from a more user friendly syntax, [this cheatsheet](http://commonmark.org/help/) is a good place to get started. The content here will fill in the left section of each project page.

### meta

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

This folder will include the templates that generate your site. each of the `html` files are required for this to work. In the `.tar.gz` file they will already be included in the `input/templates` folder.

### about

Just like the [content.md](#content) file in each of the project folders this `markdown` file will provide the content of the about page.

### joe.jpg
> Note currently this needs to be a `jpg` file but I might try and make this more flexable.

This file will act as the image placed on the about page.

## Using the program

One quick way to do most of that would be to use the program `build_site` in the folder. If you double click this file, it should open a terminal and prompt you with the following.

> note: you may need to update the security settings for this or the permissions on this file.

```
JoeMollen.Com Site Builder
What are you looking to do?
Your options are
----------
1. setup - setup the basic folder structure
2. add - add a new empty project to your portfolio
3. build - build the site
4. layout - see the folder layout
```

By entering either the number or the option name you will then be prompted for an input folder path.
```
Where is your input folder? [./input]
```

If you just hit enter, it will assume that the input folder is located in the same folder as the `build_site` file. If they are not in the same place you can enter the file path to the input folder.

If you are running the `build` command, it will prompt you for an output folder.

```
Where is your output folder? [./www]
```

If you just hit enter, it will assume that you want to create/update a folder titled `www` in the same folder as the `build_site` file. If you want to have it put the files somewhere else, you can enter your desired file path.

Now, what do each of the options do?

1. `setup` - This will setup all of the input files/folders for you, even creating a sample project folder.
2. `build` - This will actually build your site, looking for the input you specified and putting the files in the output you specified.
3. `add` - This will create a new project folder with the required files inside of the folder you specified as your input folder.
4. `layout` - This will print out both the input and output directory structures.


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

## Advanced

### Using the terminal
One way to get around all of the prompting from the above method would be to launch `build` from the terminal, to do this you first want to open the app `terminal`. When you do this it will open with your current directory set to your main user folder (this is the parent folder to Documents). You want to move into the folder that was generated when you unzipped the download, you can do that by typing the following.

```bash
$ cd Downloads/jm
```
> assuming that it ended up in your downloads folder.

Once there you can type use the command 
```bash
build [COMMAND] (-i=/path/to/input) (-o=/path/to/output)
```

where `[COMMAND]` is one of the options listed above (`build`, `add`, `setup`, `layout`), the arguments in parenthesis are optional. `-i` is to use a custom input folder, if not provided it will use `./input` and `-o` is to use a custom output folder (note this is only for the `build` command), if not provided it will use `./www`
### Updating the permissions

If you encounter a permissions error with `build_site`, open a terminal and use `cd [path]` where the path is the folder that `build_site` is located. once there you can enter `chmod +x build_site` to allow this file to run.

If you are promted that `build` or `build_site` is not from a trusted publisher (you trust me... right?), open your prefrences and select `Security & Privacy`. On this page, you should see a button that say "Open Anyway" with the name `build` or `build_site` next to it. 
