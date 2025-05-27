_wallust() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="wallust"
                ;;
            wallust,cs)
                cmd="wallust__cs"
                ;;
            wallust,debug)
                cmd="wallust__debug"
                ;;
            wallust,help)
                cmd="wallust__help"
                ;;
            wallust,migrate)
                cmd="wallust__migrate"
                ;;
            wallust,pywal)
                cmd="wallust__pywal"
                ;;
            wallust,run)
                cmd="wallust__run"
                ;;
            wallust,theme)
                cmd="wallust__theme"
                ;;
            wallust__help,cs)
                cmd="wallust__help__cs"
                ;;
            wallust__help,debug)
                cmd="wallust__help__debug"
                ;;
            wallust__help,help)
                cmd="wallust__help__help"
                ;;
            wallust__help,migrate)
                cmd="wallust__help__migrate"
                ;;
            wallust__help,pywal)
                cmd="wallust__help__pywal"
                ;;
            wallust__help,run)
                cmd="wallust__help__run"
                ;;
            wallust__help,theme)
                cmd="wallust__help__theme"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        wallust)
            opts="-I -q -s -T -u -C -d -N -h -V --ignore-sequence --quiet --skip-sequences --skip-templates --update-current --config-file --config-dir --templates-dir --no-config --help --version run cs theme migrate debug pywal help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --ignore-sequence)
                    COMPREPLY=($(compgen -W "background foreground cursor color0 color1 color2 color3 color4 color5 color6 color7 color8 color9 color10 color11 color12 color13 color14 color15" -- "${cur}"))
                    return 0
                    ;;
                -I)
                    COMPREPLY=($(compgen -W "background foreground cursor color0 color1 color2 color3 color4 color5 color6 color7 color8 color9 color10 color11 color12 color13 color14 color15" -- "${cur}"))
                    return 0
                    ;;
                --config-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --templates-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__cs)
            opts="-f -I -q -s -T -u -C -d -N -h --format --ignore-sequence --quiet --skip-sequences --skip-templates --update-current --config-file --config-dir --templates-dir --no-config --help <COLORSCHEME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --format)
                    COMPREPLY=($(compgen -W "pywal terminal-sexy wallust" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "pywal terminal-sexy wallust" -- "${cur}"))
                    return 0
                    ;;
                --ignore-sequence)
                    COMPREPLY=($(compgen -W "background foreground cursor color0 color1 color2 color3 color4 color5 color6 color7 color8 color9 color10 color11 color12 color13 color14 color15" -- "${cur}"))
                    return 0
                    ;;
                -I)
                    COMPREPLY=($(compgen -W "background foreground cursor color0 color1 color2 color3 color4 color5 color6 color7 color8 color9 color10 color11 color12 color13 color14 color15" -- "${cur}"))
                    return 0
                    ;;
                --config-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --templates-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__debug)
            opts="-I -q -s -T -u -C -d -N -h --ignore-sequence --quiet --skip-sequences --skip-templates --update-current --config-file --config-dir --templates-dir --no-config --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --ignore-sequence)
                    COMPREPLY=($(compgen -W "background foreground cursor color0 color1 color2 color3 color4 color5 color6 color7 color8 color9 color10 color11 color12 color13 color14 color15" -- "${cur}"))
                    return 0
                    ;;
                -I)
                    COMPREPLY=($(compgen -W "background foreground cursor color0 color1 color2 color3 color4 color5 color6 color7 color8 color9 color10 color11 color12 color13 color14 color15" -- "${cur}"))
                    return 0
                    ;;
                --config-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --templates-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__help)
            opts="run cs theme migrate debug pywal help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__help__cs)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__help__debug)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__help__migrate)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__help__pywal)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__help__run)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__help__theme)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__migrate)
            opts="-I -q -s -T -u -C -d -N -h --ignore-sequence --quiet --skip-sequences --skip-templates --update-current --config-file --config-dir --templates-dir --no-config --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --ignore-sequence)
                    COMPREPLY=($(compgen -W "background foreground cursor color0 color1 color2 color3 color4 color5 color6 color7 color8 color9 color10 color11 color12 color13 color14 color15" -- "${cur}"))
                    return 0
                    ;;
                -I)
                    COMPREPLY=($(compgen -W "background foreground cursor color0 color1 color2 color3 color4 color5 color6 color7 color8 color9 color10 color11 color12 color13 color14 color15" -- "${cur}"))
                    return 0
                    ;;
                --config-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --templates-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__pywal)
            opts="-a -b -f -c -i -l -n -o -q -r -R -s -t -v -e -I -T -u -C -d -N -h --backend --theme --iterative --saturate --preview --vte --ignore-sequence --skip-templates --update-current --config-file --config-dir --templates-dir --no-config --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                -a)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -b)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --backend)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --theme)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --saturate)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --ignore-sequence)
                    COMPREPLY=($(compgen -W "background foreground cursor color0 color1 color2 color3 color4 color5 color6 color7 color8 color9 color10 color11 color12 color13 color14 color15" -- "${cur}"))
                    return 0
                    ;;
                -I)
                    COMPREPLY=($(compgen -W "background foreground cursor color0 color1 color2 color3 color4 color5 color6 color7 color8 color9 color10 color11 color12 color13 color14 color15" -- "${cur}"))
                    return 0
                    ;;
                --config-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --templates-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__run)
            opts="-a -b -c -f -k -n -p -t -w -I -q -s -T -u -C -d -N -h --alpha --backend --colorspace --fallback-generator --check-contrast --no-cache --palette --saturation --threshold --dynamic-threshold --overwrite-cache --ignore-sequence --quiet --skip-sequences --skip-templates --update-current --config-file --config-dir --templates-dir --no-config --help <FILE>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --alpha)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -a)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --backend)
                    COMPREPLY=($(compgen -W "full resized wal thumb fastresize kmeans" -- "${cur}"))
                    return 0
                    ;;
                -b)
                    COMPREPLY=($(compgen -W "full resized wal thumb fastresize kmeans" -- "${cur}"))
                    return 0
                    ;;
                --colorspace)
                    COMPREPLY=($(compgen -W "lab labmixed lch lchmixed lchansi" -- "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -W "lab labmixed lch lchmixed lchansi" -- "${cur}"))
                    return 0
                    ;;
                --fallback-generator)
                    COMPREPLY=($(compgen -W "interpolate complementary" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "interpolate complementary" -- "${cur}"))
                    return 0
                    ;;
                --palette)
                    COMPREPLY=($(compgen -W "dark dark16 darkcomp darkcomp16 ansidark ansidark16 harddark harddark16 harddarkcomp harddarkcomp16 light light16 lightcomp lightcomp16 softdark softdark16 softdarkcomp softdarkcomp16 softlight softlight16 softlightcomp softlightcomp16" -- "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -W "dark dark16 darkcomp darkcomp16 ansidark ansidark16 harddark harddark16 harddarkcomp harddarkcomp16 light light16 lightcomp lightcomp16 softdark softdark16 softdarkcomp softdarkcomp16 softlight softlight16 softlightcomp softlightcomp16" -- "${cur}"))
                    return 0
                    ;;
                --saturation)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -t)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --ignore-sequence)
                    COMPREPLY=($(compgen -W "background foreground cursor color0 color1 color2 color3 color4 color5 color6 color7 color8 color9 color10 color11 color12 color13 color14 color15" -- "${cur}"))
                    return 0
                    ;;
                -I)
                    COMPREPLY=($(compgen -W "background foreground cursor color0 color1 color2 color3 color4 color5 color6 color7 color8 color9 color10 color11 color12 color13 color14 color15" -- "${cur}"))
                    return 0
                    ;;
                --config-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --templates-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__theme)
            opts="-p -I -q -s -T -u -C -d -N -h --preview --ignore-sequence --quiet --skip-sequences --skip-templates --update-current --config-file --config-dir --templates-dir --no-config --help 3024-Day 3024-Night Aci Acme Aco Adventure-Time Afterglow Alien-Blood Apprentice Argonaut Arthur Astrodark Atelier-Cave Atelier-Dune Atelier-Estuary Atelier-Forest Atelier-Heath Atelier-Lakeside Atelier-Plateau Atelier-Savanna Atelier-Seaside Atelier-Sulphurpool Atom Aura Ayaka Ayu-Dark Ayu-Light Ayu-Mirage Azu Base2Tone-Cave Base2Tone-Desert Base2Tone-Drawbridge Base2Tone-Earth Base2Tone-Evening Base2Tone-Field Base2Tone-Forest Base2Tone-Garden Base2Tone-Heath Base2Tone-Lake Base2Tone-Lavender Base2Tone-Mall Base2Tone-Meadow Base2Tone-Morning Base2Tone-Motel Base2Tone-Pool Base2Tone-Porch Base2Tone-Sea Base2Tone-Space Base2Tone-Suburb Base4Tone-Classic-A Base4Tone-Classic-B Base4Tone-Classic-C Base4Tone-Classic-D Base4Tone-Classic-E Base4Tone-Classic-F Base4Tone-Classic-I Base4Tone-Classic-L Base4Tone-Classic-O Base4Tone-Classic-P Base4Tone-Classic-Q Base4Tone-Classic-R Base4Tone-Classic-S Base4Tone-Classic-T Base4Tone-Classic-U Base4Tone-Classic-W Base4Tone-Modern-C Base4Tone-Modern-N Base4Tone-Modern-W Belafonte-Day Belafonte-Night Bim Birds-Of-Paradise Blazer Blue-Dolphin Blue-Moon Blue-Moon-Light Bluloco-Light Bluloco-Zsh-Light Borland Breadog Breath Breath-Darker Breath-Light Breath-Silverfox Breeze Broadcast Brogrammer Butrin C64 Cai Campbell Catppuccin-Frappé Catppuccin-Latte Catppuccin-Macchiato Catppuccin-Mocha Chalk Chalkboard Chameleon Ciapre City-Lights Clone-Of-Ubuntu Clrs Cobalt-2 Cobalt-Neon Colorcli Crayon-Pony-Fish Dark-Pastel Darkside Dehydration Desert Dimmed-Monokai Dissonance Doom-One Dracula Earthsong Elemental Elementary Elic Elio Espresso Espresso-Libre Everblush Everforest-Dark-Hard Everforest-Dark-Medium Everforest-Dark-Soft Everforest-Light-Hard Everforest-Light-Medium Everforest-Light-Soft Fairy-Floss Fairy-Floss-Dark Fishtank Flat Flat-Remix Flatland Flexoki-Dark Flexoki-Light Foxnightly Freya Frontend-Delight Frontend-Fun-Forrest Frontend-Galaxy Geohot Github-Dark Github-Light Gogh Gooey Google-Dark Google-Light Gotham Grape Grass Gruvbox Gruvbox-Dark Gruvbox-Material-Dark Gruvbox-Material-Light Hardcore Harper Hemisu-Dark Hemisu-Light Highway Hipster-Green Homebrew Homebrew-Light Homebrew-Ocean Horizon-Bright Horizon-Dark Hurtado Hybrid Ibm-3270-High-Contrast Ibm3270 Ic-Green-Ppl Ic-Orange-Ppl Iceberg Idle-Toes Ir-Black Jackie-Brown Japanesque Jellybeans Jup Kanagawa-Dragon Kanagawa-Lotus Kanagawa-Wave Kibble Kokuban Laserwave Later-This-Evening Lavandula Liquid-Carbon Liquid-Carbon-Transparent Lunaria-Dark Lunaria-Eclipse Lunaria-Light Maia Man-Page Mar Material Mathias Medallion Minimalist-Dark Miramare Misterioso Modus-Operandi Modus-Operandi-Tinted Modus-Vivendi Modus-Vivendi-Tinted Molokai Mona-Lisa Mono-Amber Mono-Cyan Mono-Green Mono-Red Mono-White Mono-Yellow Monokai-Dark Monokai-Pro Monokai-Pro-Ristretto Monokai-Soda Moonfly Morada N0Tch2K Nanosecond Neon-Night Neopolitan Nep Neutron Night-Owl Nightfly Nightlion-V1 Nightlion-V2 Nighty Nord Nord-Light Novel Obsidian Ocean-Dark Oceanic-Next Ollie Omni One-Dark One-Half-Black One-Light Oxocarbon-Dark Palenight Pali Panda Paper Papercolor-Dark Papercolor-Light Paraiso-Dark Paul-Millr Pencil-Dark Pencil-Light Peppermint Pixiefloss Pnevma Powershell Predawn Pro Purple-People-Eater Quiet Red-Alert Red-Sands Relaxed Rippedcasts Rosé-Pine Rosé-Pine-Dawn Rosé-Pine-Moon Royal Sat Sea-Shells Seafoam-Pastel Selenized-Black Selenized-Dark Selenized-Light Selenized-White Seoul256 Seoul256-Light Seti Shaman Shel Slate Smyck Snazzy Soft-Server Solarized-Darcula Solarized-Dark Solarized-Dark-Higher-Contrast Solarized-Light Sonokai Spacedust Spacegray Spacegray-Eighties Spacegray-Eighties-Dull Sparky Spring Square Srcery Summer-Pop Sundried Sweet-Eliverlara Sweet-Terminal Symphonic Synthwave Synthwave-Alpha Teerb Tempus-Autumn Tempus-Classic Tempus-Dawn Tempus-Day Tempus-Dusk Tempus-Fugit Tempus-Future Tempus-Night Tempus-Past Tempus-Rift Tempus-Spring Tempus-Summer Tempus-Tempest Tempus-Totus Tempus-Warp Tempus-Winter Tender Terminal-Basic Terminix-Dark Thayer-Bright Tin Tokyo-Night Tokyo-Night-Light Tokyo-Night-Storm Tomorrow Tomorrow-Night Tomorrow-Night-Blue Tomorrow-Night-Bright Tomorrow-Night-Eighties Toy-Chest Treehouse Twilight Ura Urple Vag Vaombe Vaughn Vibrant-Ink Vs-Code-Dark+ Vs-Code-Light+ Warm-Neon Website Wez Wild-Cherry Wombat Wryan Wzoreck Zenburn dkeg-petal base16-paraiso sexy-s3r0-modified sexy-rasi solarized-dark sexy-muse base16-rebecca base16-classic-dark dkeg-leaf dkeg-harbing base16-atelier-forest-light sexy-simple_rainbow base16-bright base16-embers base16-chalk dkeg-flapr sexy-tangoesque base16-twilight sexy-visibone base16-brewer sexy-x-dotshare sexy-astromouse dkeg-bulb base16-summerfruit-light dkeg-chaires base16-black-metal-funeral base16-black-metal-burzum base16-codeschool base16-cupcake tempus_autumn dkeg-link sexy-gslob-nature-suede sexy-hybrid base16-materialer-dark dkeg-view dkeg-blumune base16-zenburn base16-gruvbox-pale dkeg-sprout sexy-mikado dkeg-scag base16-macintosh base16-black-metal-khold base16-atelier-plateau-dark base16-unikitty-light base16-google-light base16-materialer-light base16-pop base16-flat base16tooth base16-black-metal-venom base16-atelier-sulphurpool-dark base16-atelier-estuary-light sexy-zenburn base16-eighties sexy-eqie6 base16-3024 sexy-dwmrob base16-black-metal-marduk dkeg-brownstone dkeg-escen sexy-user-77-mashup-colors base16-mocha base16-mexico tempus_dusk base16-grayscale-light dkeg-novmbr dkeg-urban tempus_rift sexy-thwump dkeg-transposet tempus_future dkeg-stv base16-railscasts sexy-mikazuki base16-tomorrow base16-unikitty-dark sexy-jasonwryan dkeg-tealights base16-solarflare dkeg-raild sexy-invisibone base16-material-palenight dkeg-scape base16-black-metal-gorgoroth base16-solarized-light base16-black-metal-bathory base16-outrun dkeg-bark dkeg-spire sexy-sexcolors base16-woodland dkeg-simplicity base16-monokai base16-mellow-purple base16-xcode-dusk base16-porple base16-isotope dkeg-fendr dkeg-sundr sexy-nancy base16-classic-light dkeg-5725 base16-atelier-lakeside-dark sexy-navy-and-ivory 3024-light monokai base16-oceanicnext sexy-euphrasia sexy-visibone-alt-2 dkeg-mattd base16-atelier-seaside-light base16-atelier-savanna-light base16-atelier-heath-light dkeg-vans dkeg-coco sexy-gjm sexy-kasugano base16-atelier-sulphurpool-light base16-nord base16-black-metal-nile base16-tomorrow-night sexy-material base16-cupertino sexy-tlh srcery base16-phd base16-github tempus_summer base16-gruvbox-soft-dark base16-bespin sexy-rydgel dkeg-forst dkeg-slate sexy-theme2 base16-marrakesh sexy-colorfulcolors sexy-neon dkeg-diner base16-apathy sexy-gnometerm sexy-parker_brothers sexy-mostly-bright sexy-doomicideocean base16-atelier-dune-light zenburn sexy-rezza dkeg-relax ashes-light dkeg-provrb dkeg-skigh base16-tube dkeg-amiox sexy-numixdarkest base16-atelier-forest-dark sexy-pretty-and-pastel sexy-splurge base16-irblack base16-materia base16-gruvbox-hard-light dkeg-owl base16-atelier-heath-dark sexy-insignificato gruvbox darktooth sexy-trim-yer-beard sexy-dotshare dkeg-designr dkeg-poly base16-brushtrees base16-dracula base16-atelier-lakeside-light tempus_totus base16-atelier-cave-dark sexy-hund tempus_fugit base16-black-metal-mayhem base16-default-light dkeg-shade base16-default-dark vscode base16-atelier-plateau-light base16-hopscotch base16-grayscale-dark base16-atelier-estuary-dark base16-icy 3024-dark sexy-cloud rose-pine sexy-phrak1 base16-snazzy rose-pine-moon sexy-deafened dkeg-fury dkeg-blok base16-summerfruit-dark base16-pico sexy-sweetlove sexy-belge dkeg-victory sexy-bitmute base16-spacemacs sexy-orangish tempus_dawn dkeg-blend dkeg-book github base16-circus base16-gruvbox-medium-dark dkeg-prevail dkeg-depth base16-black-metal-immortal dkeg-soundwave sexy-gotham base16-atelier-seaside-dark tempus_warp base16-onedark dkeg-pastely base16-harmonic-light sexy-vacuous2 tempus_winter base16-atelier-savanna-dark base16-ashes rose-pine-dawn sexy-swayr sexy-digerati base16-google-dark sexy-monokai dkeg-branch dkeg-squares sexy-tartan tempus_spring base16-one base16-black-metal tempus_past base16-gruvbox-medium-light dkeg-bluetype base16-atelier-cave-light dkeg-subtle solarized-light base16-gruvbox-hard-dark base16-atelier-dune-dark sexy-tango dkeg-wintry base16-greenscreen base16-harmonic-dark base16-ocean dkeg-parkd sexy-derp dkeg-paints ashes-dark dkeg-kit base16-solarized-dark base16-seti sexy-dawn base16-gruvbox-soft-light hybrid-material dkeg-corduroy dkeg-traffic base16-shapeshifter base16-material random list"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --ignore-sequence)
                    COMPREPLY=($(compgen -W "background foreground cursor color0 color1 color2 color3 color4 color5 color6 color7 color8 color9 color10 color11 color12 color13 color14 color15" -- "${cur}"))
                    return 0
                    ;;
                -I)
                    COMPREPLY=($(compgen -W "background foreground cursor color0 color1 color2 color3 color4 color5 color6 color7 color8 color9 color10 color11 color12 color13 color14 color15" -- "${cur}"))
                    return 0
                    ;;
                --config-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --templates-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _wallust -o nosort -o bashdefault -o default wallust
else
    complete -F _wallust -o bashdefault -o default wallust
fi
