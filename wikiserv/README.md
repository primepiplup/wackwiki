# WikiServ

An HTTP server for WackWiki.

My idea with this part of the project is to write a simple HTTP server that serves wiki files.
I think it would be fun to serve these files dynamically, meaning that if a certain keyword is requested, the server translates the wiki file into an HTML document which is then provided to the web-browser.
This means that the server has to:
 1. Speak the HTTP protocol
 2. Parse wiki files
 3. Output HTML files that conserve formatting and contain links such that navigation becomes possible

## How to use

Using WikiServ is really simple if you're already using other WackWiki programs.
You have to set up the `$WIKIPATH` environment variable, which will enable the WikiServ application to find the wiki files.
You can also set up a template for wiki entries! `$HOME/.config/wikiserv/template.html` is used by the application.
The content of your wiki pages is parsed and then inserted in the first place where `--wikicontent--` occurs in the template file.
