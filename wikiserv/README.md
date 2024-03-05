# WikiServ

An HTTP server for WackWiki.

My idea with this part of the project is to write a simple HTTP server that serves wiki files.
I think it would be fun to serve these files dynamically, meaning that if a certain keyword is requested, the server translates the wiki file into an HTML document which is then provided to the web-browser.
This means that the server has to:
 1. Speak the HTTP protocol
 2. Parse wiki files
 3. Output HTML files that conserve formatting and contain links such that navigation becomes possible
