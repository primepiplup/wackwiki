# WackWiki

The goal of this project is to create an open wiki system that can be used in personal or professional settings.
The aim is to keep the format as open as possible, allowing tools to easily be written that can parse and link the different wiki files.
The wiki software should bend to the user's will, not constraining the expression of their ideas.
By providing a simple system it should be easy to write plugins for your favourite editors or small scripts to automate certain elements of wiki management.

## Use

### Setting up
Setting up wackwiki is pretty simple. You have to create a directory somewhere on your system that you want to be used as your wiki directory.
After you have created this folder you have to ensure that the `$WIKIPATH` environment variable is set to the full path to that directory.
I use `export WIKIPATH=[path]` in my `.bashrc` to achieve this.
This `$WIKIPATH` environment variable is used by all wackwiki programs to find, traverse and manipulate the wiki structure.

### WikiCLI
WikiCLI was created to make management, creation and editing of wiki files simpler.
It provides easy ways to create groups and subgroups, create entries within groups, list groups and entries, edit entries, and to insert symlinks to linked local files into groups.

### WikiSERV
WikiSERV is a simple HTTP server which parses your wiki pages and serves them as HTML documents.
Using your webbrowser to navigate to WikiSERV will show you a listing of groups and entries.
Group pages are shown as a list of links to their contents.
Entry pages are parsed and shown as HTML documents.
The basic formatting of WikiSERV pages is... basic, and not very pretty. You can fully customize this however.
A template is picked up from the WikiSERV config directory, and the parsed HTML is inserted into your chosen location within the template.
WikiSERV inserts class names into all parsed HTML tags.
All of this means that you can create a template webpage with chosen layout, CSS and even Javascript (if you so choose) into which the WikiSERV application will then put the contents of the wikipage you choose to view.

### The path and linking system
The wiki system includes a global directory located at `$WIKIPATH` as well as subgroups located at `$WIKIPATH/[subgroup]` which can be nested indefinitely.

Entries are text files, formatted in the format described in the section below.

The main feature of wackwiki is automatic linking of mentioned words within a group.

Say you create an entry called `cats` and another entry called `animals`.
Within the `animals` entry you compile a list of animals, which includes `cats`.
Wackwiki will recognize that such an entry exists within the same group, and generates a link to that entry.

Relative linking is also possible.
Imagine you have a group called `reptiles` which is contained within the global group.
The `reptiles` group contains an entry called `crocodile`
You can mention this crocodile from within the `animals` entry simply by saying `reptiles/crocodile`, and wackwiki will recognize the entry, generating a link towards it.
This behaviour can be used for nested groups as well by providing the relative path from the current group down into the subgroup and ending with the mentioned entry.

You can also use absolute links.
Imagine you have another group within the global group called `birds`.
Within this group you create an entry called `plover`, and you wish to mention that plovers sometimes sit on crocodiles.
You can mention crocodiles from within the `plover` entry by saying `/reptiles/crocodile`, and wackwiki will generate the link.
This behaviour is generally available. You can mention any entry by providing an absolute path from the wiki root.

### The text format
The wiki format is a partial implementation of markdown with some slight alterations.

You create headings by using #

You can make text bold by surrounding it with *

You can make text italic by surrounding it with _

You can make text striken through by surrounding it with =

You can make text underlined by surrounding it with -

You can make by putting a > in front, multiple > characters deepen the quote level.

You can make unordered lists by putting a - at the start of a line. Indenting with four spaces increases the indentation level of the list.

You can make ordered lists by putting a number followed by a dot at the start of a line. Indenting with four spaces increases the indentation level of the list.

You can include links by inserting text surrounded by [ and ] immediately followed by a link surrounded by ( and ). The text in the square brackets will be the link text and the link in the brackets will be the place the link points to.

### Images
#### External images
Images can be included in wiki files.
You can link to external (not contained on your computer) images with exactly the same syntax as inputting an external link into a wiki entry.
You provide alt-text in [ square brackets ] and provide the link to the image in ( brackets ). Currently only `.jpg` and `.png` extensions are recognized as images however.

#### Local images
Local images also use the [ alt-text ] and ( link ) syntax.
Images are expected within a hidden `.link` subdirectory within the group in which they are mentioned.
For instance, if you want to include an image of your cat within the `cats` wiki page within the global wiki directory, then you should include `my_cat.png` within `$WIKIPATH/.link/my_cat.png`. 
(This is aided by a utility provided by the WikiCLI application which can be used to easily create symlinks for files which are then inserted into the `.link` group of your choosing.)

