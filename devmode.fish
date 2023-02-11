set content "$(cat templates/base.html)"

read -l -P "Devmode:"\n\t"on(1)"\n\t"off(0)"\n"> " devmode
switch $devmode
    case 1
        set content $(string replace /style.css $(pwd)/docs/style.css $content)
        set content $(string replace /index.html $(pwd)/docs/index.html $content)
        set content $(string replace /stories.html $(pwd)/docs/stories.html $content)
        echo $content > $(pwd)/templates/base.html
    case 0
        git restore $(pwd)/templates/base.html
end
