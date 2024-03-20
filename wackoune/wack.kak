declare-option str wikipath
set-option global wikipath %sh{echo $WIKIPATH}

hook global GlobalSetOption within_wiki=true %{

    declare-option str wikigroup
    set-option global wikigroup %sh{pwd | sed 's\'"$WIKIPATH"'/\\g'}

    add-highlighter global/global-wikiwords regex %sh{find $WIKIPATH -type f | grep -v "\..*$" | sed 's\'"$WIKIPATH"'\\g' | tr '\n' '|'} 0:yellow+u
    add-highlighter global/relative-wikiwords regex %sh{find . -type f | grep -v "\./\..*$" | sed 's-\./--g' | tr '\n' '|'} 0:bright-cyan+u
    add-highlighter global/wackwiki-bold regex (?<!\\)\*.*?(?<!\\)\* 0:+b
    add-highlighter global/wackwiki-italic regex (?<!\\)_.*?(?<!\\)_ 0:+i
    add-highlighter global/wackwiki-underline regex (?<!\\)=.*?(?<!\\)= 0:+u
    add-highlighter global/wackwiki-strikethrough regex (?<!\\)~.*?(?<!\\)~ 0:+s
    add-highlighter global/wackwiki-header regex ^#.*?$ 0:bright-yellow+b
    add-highlighter global/wackwiki-link regex \[(?<content>.*?)\]\((?<link>.*?)\) 0:blue content:+b link:+i

    define-command -docstring "Select a whitespace or newline delimited string" wiki-select-whitespace-delimited %{
        execute-keys %{<a-/>(\h|\n)<ret>L<a-;>?(\h|\n)<ret>H}
    }

    define-command -docstring "Open/Edit the selected wiki entry" wiki-edit-entry %{
        evaluate-commands %{edit %sh{echo $WIKIPATH/$kak_reg_dot}}
    }

    define-command -docstring "Print the contents of a wiki entry in the kakoune info box" wiki-print-entry %{
        evaluate-commands %{info %sh{cat $WIKIPATH/$kak_reg_dot}}
    }

    declare-user-mode wack

    map global user w %{:enter-user-mode wack<ret>} -docstring "WackWiki mode"
    map global wack e %{:wiki-select-whitespace-delimited<ret>:wiki-edit-entry<ret>} -docstring "Edit/Open the wiki entry the cursor is currently on"
    map global wack h %{:wiki-select-whitespace-delimited<ret>:wiki-print-entry<ret>} -docstring "Print the contents of the entry in an info box"
}

declare-option bool within_wiki
set-option global within_wiki %sh{
    if [ -z $(pwd | grep $WIKIPATH) ]
    then
        echo "false"
    else
        echo "true"
    fi
}
