
use builtin;
use str;

set edit:completion:arg-completer[wallust] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'wallust'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'wallust'= {
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
            cand run 'Generate a palette from an image'
            cand cs 'Apply a certain colorscheme'
            cand debug 'Print information about the program and the enviroment it uses'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'wallust;run'= {
            cand -a 'Alpha *template variable* value, used only for templating (default is 100)'
            cand --alpha 'Alpha *template variable* value, used only for templating (default is 100)'
            cand -b 'Choose which backend to use (overwrites config)'
            cand --backend 'Choose which backend to use (overwrites config)'
            cand -c 'Choose which colorspace to use (overwrites config)'
            cand --colorspace 'Choose which colorspace to use (overwrites config)'
            cand -C 'Use FILE as the config file'
            cand --config-path 'Use FILE as the config file'
            cand -d 'Use DIR as the config directory'
            cand --config-dir 'Use DIR as the config directory'
            cand -p 'Choose which palette to use (overwrites config)'
            cand -f 'Choose which palette to use (overwrites config)'
            cand --palette 'Choose which palette to use (overwrites config)'
            cand --filter 'Choose which palette to use (overwrites config)'
            cand -g 'Choose which generation method to use (overwrites config)'
            cand --generation 'Choose which generation method to use (overwrites config)'
            cand --saturation 'Add saturation from 1% to 100% (overwrites config)'
            cand -t 'Choose a custom threshold (overwrites config)'
            cand --threshold 'Choose a custom threshold (overwrites config)'
            cand -k 'Ensure a readable contrast by checking colors in reference to the background (overwrites config)'
            cand --check-contrast 'Ensure a readable contrast by checking colors in reference to the background (overwrites config)'
            cand -n 'Don''t cache the results'
            cand --no-cache 'Don''t cache the results'
            cand -q 'Don''t print anything'
            cand --quiet 'Don''t print anything'
            cand -s 'Skip setting terminal sequences'
            cand --skip-sequences 'Skip setting terminal sequences'
            cand -T 'Skip the templating process'
            cand --skip-templates 'Skip the templating process'
            cand -u 'Only update the current terminal colros'
            cand --update-current 'Only update the current terminal colros'
            cand -w 'Generates colors even if there is a cache version of it'
            cand --overwrite-cache 'Generates colors even if there is a cache version of it'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
        }
        &'wallust;cs'= {
            cand -f 'Specify a custom format. Without this option, wallust will sequentially try to decode it by trying one by one'
            cand --format 'Specify a custom format. Without this option, wallust will sequentially try to decode it by trying one by one'
            cand -q 'Don''t print anything'
            cand --quiet 'Don''t print anything'
            cand -s 'Skip setting terminal sequences'
            cand --skip-sequences 'Skip setting terminal sequences'
            cand -T 'Skip templating process'
            cand --skip-templates 'Skip templating process'
            cand -u 'Only update the current terminal'
            cand --update-current 'Only update the current terminal'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
        }
        &'wallust;debug'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'wallust;help'= {
            cand run 'Generate a palette from an image'
            cand cs 'Apply a certain colorscheme'
            cand debug 'Print information about the program and the enviroment it uses'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'wallust;help;run'= {
        }
        &'wallust;help;cs'= {
        }
        &'wallust;help;debug'= {
        }
        &'wallust;help;help'= {
        }
    ]
    $completions[$command]
}
