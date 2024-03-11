declare-option str wikigroup
set-option global wikigroup %sh{pwd | sed 's\'"$WIKIPATH"'/\\g'}

add-highlighter global/wikiwords regex %sh{ls -p | grep -v / | tr '\n' '|'} 0:cyan

define-command -docstring "Select a whitespace or newline delimited string" wiki-select-whitespace-delimited %{
    execute-keys %{<a-/>(\h|\n)<ret>L<a-;>?(\h|\n)<ret>H}
}

define-command -docstring "Open/Edit the selected wiki entry" wiki-edit-entry %{
    evaluate-commands %{edit %reg{.}}
}

declare-user-mode wack

map global user w %{:enter-user-mode wack<ret>} -docstring "WackWiki mode"
map global wack e %{:wiki-select-whitespace-delimited<ret>:wiki-edit-entry<ret>} -docstring "Edit/Open the wiki entry the cursor is currently on"

