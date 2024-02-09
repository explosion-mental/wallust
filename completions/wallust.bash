_wallust() {
    local i cur prev opts cmd
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
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
            opts="-h -V --help --version run cs theme debug help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
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
        wallust__cs)
            opts="-f -q -s -T -u -h --format --quiet --skip-sequences --skip-templates --update-current --help <FILE>"
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
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__debug)
            opts="-h --help"
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
        wallust__help)
            opts="run cs theme debug help"
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
        wallust__run)
            opts="-a -b -c -C -d -f -p -g -k -n -q -s -t -T -u -w -h --alpha --backend --colorspace --config-path --config-dir --filter --palette --generation --check-contrast --no-cache --quiet --skip-sequences --saturation --threshold --skip-templates --update-current --overwrite-cache --help <FILE>"
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
                    COMPREPLY=($(compgen -W "lab labmixed labfast" -- "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -W "lab labmixed labfast" -- "${cur}"))
                    return 0
                    ;;
                --config-path)
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
                --palette)
                    COMPREPLY=($(compgen -W "dark dark16 darkcomp darkcomp16 harddark harddark16 harddarkcomp harddarkcomp16 light light16 lightcomp lightcomp16 softdark softdark16 softdarkcomp softdarkcomp16 softlight softlight16 softlightcomp softlightcomp16" -- "${cur}"))
                    return 0
                    ;;
                --filter)
                    COMPREPLY=($(compgen -W "dark dark16 darkcomp darkcomp16 harddark harddark16 harddarkcomp harddarkcomp16 light light16 lightcomp lightcomp16 softdark softdark16 softdarkcomp softdarkcomp16 softlight softlight16 softlightcomp softlightcomp16" -- "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -W "dark dark16 darkcomp darkcomp16 harddark harddark16 harddarkcomp harddarkcomp16 light light16 lightcomp lightcomp16 softdark softdark16 softdarkcomp softdarkcomp16 softlight softlight16 softlightcomp softlightcomp16" -- "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -W "dark dark16 darkcomp darkcomp16 harddark harddark16 harddarkcomp harddarkcomp16 light light16 lightcomp lightcomp16 softdark softdark16 softdarkcomp softdarkcomp16 softlight softlight16 softlightcomp softlightcomp16" -- "${cur}"))
                    return 0
                    ;;
                --generation)
                    COMPREPLY=($(compgen -W "interpolate complementary" -- "${cur}"))
                    return 0
                    ;;
                -g)
                    COMPREPLY=($(compgen -W "interpolate complementary" -- "${cur}"))
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
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        wallust__theme)
            opts="-p -q -s -T -u -h --preview --quiet --skip-sequences --skip-templates --update-current --help <THEME>"
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
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _wallust -o nosort -o bashdefault -o default wallust
else
    complete -F _wallust -o bashdefault -o default wallust
fi
