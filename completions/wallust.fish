# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_wallust_global_optspecs
	string join \n I/ignore-sequence= q/quiet s/skip-sequences T/skip-templates u/update-current C/config-file= d/config-dir= templates-dir= N/no-config h/help V/version
end

function __fish_wallust_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_wallust_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_wallust_using_subcommand
	set -l cmd (__fish_wallust_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c wallust -n "__fish_wallust_needs_command" -s I -l ignore-sequence -d 'Won\'t send these colors sequences' -r -f -a "background\t''
foreground\t''
cursor\t''
color0\t''
color1\t''
color2\t''
color3\t''
color4\t''
color5\t''
color6\t''
color7\t''
color8\t''
color9\t''
color10\t''
color11\t''
color12\t''
color13\t''
color14\t''
color15\t''"
complete -c wallust -n "__fish_wallust_needs_command" -s C -l config-file -d 'Use CONFIG_FILE as the config file' -r -F
complete -c wallust -n "__fish_wallust_needs_command" -s d -l config-dir -d 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)' -r -F
complete -c wallust -n "__fish_wallust_needs_command" -l templates-dir -d 'Uses TEMPLATE_DIR as the template directory' -r -F
complete -c wallust -n "__fish_wallust_needs_command" -s q -l quiet -d 'Don\'t print anything'
complete -c wallust -n "__fish_wallust_needs_command" -s s -l skip-sequences -d 'Skip setting terminal sequences'
complete -c wallust -n "__fish_wallust_needs_command" -s T -l skip-templates -d 'Skip templating process'
complete -c wallust -n "__fish_wallust_needs_command" -s u -l update-current -d 'Only update the current terminal'
complete -c wallust -n "__fish_wallust_needs_command" -s N -l no-config -d 'Won\'t read the config and avoids creating it\'s config path'
complete -c wallust -n "__fish_wallust_needs_command" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c wallust -n "__fish_wallust_needs_command" -s V -l version -d 'Print version'
complete -c wallust -n "__fish_wallust_needs_command" -f -a "run" -d 'Generate a palette from an image'
complete -c wallust -n "__fish_wallust_needs_command" -f -a "cs" -d 'Apply a certain colorscheme'
complete -c wallust -n "__fish_wallust_needs_command" -f -a "theme" -d 'Apply a custom built in theme'
complete -c wallust -n "__fish_wallust_needs_command" -f -a "migrate" -d 'Migrate v2 config to v3 (might lose comments,)'
complete -c wallust -n "__fish_wallust_needs_command" -f -a "debug" -d 'Print information about the program and the enviroment it uses'
complete -c wallust -n "__fish_wallust_needs_command" -f -a "pywal" -d 'A drop-in cli replacement for pywal'
complete -c wallust -n "__fish_wallust_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c wallust -n "__fish_wallust_using_subcommand run" -s a -l alpha -d 'Alpha *template variable* value, used only for templating (default is 100)' -r
complete -c wallust -n "__fish_wallust_using_subcommand run" -s b -l backend -d 'Choose which backend to use (overwrites config)' -r -f -a "full\t'Read and return the whole image pixels (more precision, slower)'
resized\t'Resizes the image before parsing, mantaining it\'s aspect ratio'
wal\t'Uses image magick `convert` to generate the colors, like pywal'
thumb\t'Faster algo hardcoded to 512x512 (no ratio respected)'
fastresize\t'A much faster resize algo that uses SIMD. For some reason it fails on some images where `resized` doesn\'t, for this reason it doesn\'t *replace* but rather it\'s a new option'
kmeans\t'Kmeans is an algo that divides and picks pixels all around the image, giving a more diverse look'"
complete -c wallust -n "__fish_wallust_using_subcommand run" -s c -l colorspace -d 'Choose which colorspace to use (overwrites config)' -r -f -a "lab\t'Uses Cie L*a*b color space'
labmixed\t'Variant of `lab` that mixes the colors gathered, if not enough colors it fallbacks to usual lab (not recommended in small images)'
lch\t'CIE Lch, you can understand this color space like LAB but with chrome and hue added. Could help when sorting'
lchmixed\t'CIE Lch variant that mixed on every similar color'
lchansi\t'Variant of Lch which preserves 8 colors: black, red, green, yellow, blue, magenta, cyan and gray. This works best with \'darkansi\' palette, allowing a constant color order'"
complete -c wallust -n "__fish_wallust_using_subcommand run" -s f -l fallback-generator -d 'Choose which fallback generation method to use (overwrites config)' -r -f -a "interpolate\t'uses [`interpolate`]'
complementary\t'uses [`complementary`]'"
complete -c wallust -n "__fish_wallust_using_subcommand run" -s p -l palette -d 'Choose which palette to use (overwrites config)' -r -f -a "dark\t'8 dark colors, dark background and light contrast'
dark16\t'Same as `dark` but uses the 16 colors trick'
darkcomp\t'This is a `dark` variant that changes all colors to it\'s complementary counterpart, giving the feeling of a \'new palette\' but that still makes sense with the image provided'
darkcomp16\t'16 variation of the dark complementary variant'
ansidark\t'This is not a \'dark\' variant, is a new palette that is meant to work with `lchansi` colorspace, which will maintain \'tty\' like color order and only adjusting the colors acording to the theme. A possible solution for LS_COLORS and the like. Should workout with other colorspace, but the result may not be optimal'
ansidark16\t'The ansidark palette with 16 color variation'
harddark\t'Same as `dark` with hard hue colors'
harddark16\t'Harddark with 16 color variation'
harddarkcomp\t'complementary colors variation of harddark scheme'
harddarkcomp16\t'complementary colors variation of harddark scheme'
light\t'Light bg, dark fg'
light16\t'Same as `light` but uses the 16 color trick'
lightcomp\t'complementary colors variation of light'
lightcomp16\t'complementary colors variation of light with the 16 color variation'
softdark\t'Variant of softlight, uses the lightest colors and a dark background (could be interpreted as `dark` inversed)'
softdark16\t'softdark with 16 color variation'
softdarkcomp\t'complementary variation for softdark'
softdarkcomp16\t'complementary variation for softdark with the 16 color variation'
softlight\t'Light with soft pastel colors, counterpart of `harddark`'
softlight16\t'softlight with 16 color variation'
softlightcomp\t'softlight with complementary colors'
softlightcomp16\t'softlight with complementary colors with 16 colors'"
complete -c wallust -n "__fish_wallust_using_subcommand run" -l saturation -d 'Add saturation from 1% to 100% (overwrites config)' -r
complete -c wallust -n "__fish_wallust_using_subcommand run" -s t -l threshold -d 'Choose a custom threshold, between 1 and 100 (overwrites config)' -r
complete -c wallust -n "__fish_wallust_using_subcommand run" -s I -l ignore-sequence -d 'Won\'t send these colors sequences' -r -f -a "background\t''
foreground\t''
cursor\t''
color0\t''
color1\t''
color2\t''
color3\t''
color4\t''
color5\t''
color6\t''
color7\t''
color8\t''
color9\t''
color10\t''
color11\t''
color12\t''
color13\t''
color14\t''
color15\t''"
complete -c wallust -n "__fish_wallust_using_subcommand run" -s C -l config-file -d 'Use CONFIG_FILE as the config file' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand run" -s d -l config-dir -d 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand run" -l templates-dir -d 'Uses TEMPLATE_DIR as the template directory' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand run" -s k -l check-contrast -d 'Ensure a readable contrast by checking colors in reference to the background (overwrites config)'
complete -c wallust -n "__fish_wallust_using_subcommand run" -s n -l no-cache -d 'Don\'t cache the results'
complete -c wallust -n "__fish_wallust_using_subcommand run" -l dynamic-threshold -d 'Dynamically changes the threshold to be best fit'
complete -c wallust -n "__fish_wallust_using_subcommand run" -s w -l overwrite-cache -d 'Generates colors even if there is a cache version of it'
complete -c wallust -n "__fish_wallust_using_subcommand run" -s q -l quiet -d 'Don\'t print anything'
complete -c wallust -n "__fish_wallust_using_subcommand run" -s s -l skip-sequences -d 'Skip setting terminal sequences'
complete -c wallust -n "__fish_wallust_using_subcommand run" -s T -l skip-templates -d 'Skip templating process'
complete -c wallust -n "__fish_wallust_using_subcommand run" -s u -l update-current -d 'Only update the current terminal'
complete -c wallust -n "__fish_wallust_using_subcommand run" -s N -l no-config -d 'Won\'t read the config and avoids creating it\'s config path'
complete -c wallust -n "__fish_wallust_using_subcommand run" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c wallust -n "__fish_wallust_using_subcommand cs" -s f -l format -d 'Specify a custom format. Without this option, wallust will sequentially try to decode it by trying one by one' -r -f -a "pywal\t'uses the wal colorscheme format, see <https://github.com/dylanaraps/pywal/tree/master/pywal/colorschemes>'
terminal-sexy\t'uses <https://terminal.sexy> JSON export'
wallust\t'cached wallust files'"
complete -c wallust -n "__fish_wallust_using_subcommand cs" -s I -l ignore-sequence -d 'Won\'t send these colors sequences' -r -f -a "background\t''
foreground\t''
cursor\t''
color0\t''
color1\t''
color2\t''
color3\t''
color4\t''
color5\t''
color6\t''
color7\t''
color8\t''
color9\t''
color10\t''
color11\t''
color12\t''
color13\t''
color14\t''
color15\t''"
complete -c wallust -n "__fish_wallust_using_subcommand cs" -s C -l config-file -d 'Use CONFIG_FILE as the config file' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand cs" -s d -l config-dir -d 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand cs" -l templates-dir -d 'Uses TEMPLATE_DIR as the template directory' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand cs" -s q -l quiet -d 'Don\'t print anything'
complete -c wallust -n "__fish_wallust_using_subcommand cs" -s s -l skip-sequences -d 'Skip setting terminal sequences'
complete -c wallust -n "__fish_wallust_using_subcommand cs" -s T -l skip-templates -d 'Skip templating process'
complete -c wallust -n "__fish_wallust_using_subcommand cs" -s u -l update-current -d 'Only update the current terminal'
complete -c wallust -n "__fish_wallust_using_subcommand cs" -s N -l no-config -d 'Won\'t read the config and avoids creating it\'s config path'
complete -c wallust -n "__fish_wallust_using_subcommand cs" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c wallust -n "__fish_wallust_using_subcommand theme" -s I -l ignore-sequence -d 'Won\'t send these colors sequences' -r -f -a "background\t''
foreground\t''
cursor\t''
color0\t''
color1\t''
color2\t''
color3\t''
color4\t''
color5\t''
color6\t''
color7\t''
color8\t''
color9\t''
color10\t''
color11\t''
color12\t''
color13\t''
color14\t''
color15\t''"
complete -c wallust -n "__fish_wallust_using_subcommand theme" -s C -l config-file -d 'Use CONFIG_FILE as the config file' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand theme" -s d -l config-dir -d 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand theme" -l templates-dir -d 'Uses TEMPLATE_DIR as the template directory' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand theme" -s p -l preview -d 'Only preview the selected theme'
complete -c wallust -n "__fish_wallust_using_subcommand theme" -s q -l quiet -d 'Don\'t print anything'
complete -c wallust -n "__fish_wallust_using_subcommand theme" -s s -l skip-sequences -d 'Skip setting terminal sequences'
complete -c wallust -n "__fish_wallust_using_subcommand theme" -s T -l skip-templates -d 'Skip templating process'
complete -c wallust -n "__fish_wallust_using_subcommand theme" -s u -l update-current -d 'Only update the current terminal'
complete -c wallust -n "__fish_wallust_using_subcommand theme" -s N -l no-config -d 'Won\'t read the config and avoids creating it\'s config path'
complete -c wallust -n "__fish_wallust_using_subcommand theme" -s h -l help -d 'Print help'
complete -c wallust -n "__fish_wallust_using_subcommand migrate" -s I -l ignore-sequence -d 'Won\'t send these colors sequences' -r -f -a "background\t''
foreground\t''
cursor\t''
color0\t''
color1\t''
color2\t''
color3\t''
color4\t''
color5\t''
color6\t''
color7\t''
color8\t''
color9\t''
color10\t''
color11\t''
color12\t''
color13\t''
color14\t''
color15\t''"
complete -c wallust -n "__fish_wallust_using_subcommand migrate" -s C -l config-file -d 'Use CONFIG_FILE as the config file' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand migrate" -s d -l config-dir -d 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand migrate" -l templates-dir -d 'Uses TEMPLATE_DIR as the template directory' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand migrate" -s q -l quiet -d 'Don\'t print anything'
complete -c wallust -n "__fish_wallust_using_subcommand migrate" -s s -l skip-sequences -d 'Skip setting terminal sequences'
complete -c wallust -n "__fish_wallust_using_subcommand migrate" -s T -l skip-templates -d 'Skip templating process'
complete -c wallust -n "__fish_wallust_using_subcommand migrate" -s u -l update-current -d 'Only update the current terminal'
complete -c wallust -n "__fish_wallust_using_subcommand migrate" -s N -l no-config -d 'Won\'t read the config and avoids creating it\'s config path'
complete -c wallust -n "__fish_wallust_using_subcommand migrate" -s h -l help -d 'Print help'
complete -c wallust -n "__fish_wallust_using_subcommand debug" -s I -l ignore-sequence -d 'Won\'t send these colors sequences' -r -f -a "background\t''
foreground\t''
cursor\t''
color0\t''
color1\t''
color2\t''
color3\t''
color4\t''
color5\t''
color6\t''
color7\t''
color8\t''
color9\t''
color10\t''
color11\t''
color12\t''
color13\t''
color14\t''
color15\t''"
complete -c wallust -n "__fish_wallust_using_subcommand debug" -s C -l config-file -d 'Use CONFIG_FILE as the config file' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand debug" -s d -l config-dir -d 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand debug" -l templates-dir -d 'Uses TEMPLATE_DIR as the template directory' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand debug" -s q -l quiet -d 'Don\'t print anything'
complete -c wallust -n "__fish_wallust_using_subcommand debug" -s s -l skip-sequences -d 'Skip setting terminal sequences'
complete -c wallust -n "__fish_wallust_using_subcommand debug" -s T -l skip-templates -d 'Skip templating process'
complete -c wallust -n "__fish_wallust_using_subcommand debug" -s u -l update-current -d 'Only update the current terminal'
complete -c wallust -n "__fish_wallust_using_subcommand debug" -s N -l no-config -d 'Won\'t read the config and avoids creating it\'s config path'
complete -c wallust -n "__fish_wallust_using_subcommand debug" -s h -l help -d 'Print help'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s a -d 'Set terminal background transparency. *Only works in URxvt*' -r
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s b -d 'Custom background color to use' -r
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -l backend -d 'Which color backend to use' -r
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s f -l theme -d 'Which colorscheme file to use. Use \'wal --theme\' to list builtin themes' -r
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -l saturate -d 'Set the color saturation' -r
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s i -d 'Which image or directory to use' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s o -d 'External script to run after "wal"' -r
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s I -l ignore-sequence -d 'Won\'t send these colors sequences' -r -f -a "background\t''
foreground\t''
cursor\t''
color0\t''
color1\t''
color2\t''
color3\t''
color4\t''
color5\t''
color6\t''
color7\t''
color8\t''
color9\t''
color10\t''
color11\t''
color12\t''
color13\t''
color14\t''
color15\t''"
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s C -l config-file -d 'Use CONFIG_FILE as the config file' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s d -l config-dir -d 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -l templates-dir -d 'Uses TEMPLATE_DIR as the template directory' -r -F
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -l iterative -d 'When pywal is given a directory as input and this flag is used: Go through the images in order instead of shuffled'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -l preview -d 'Print the current color palette'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -l vte -d 'Fix text-artifacts printed in VTE terminals'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s c -d 'Delete all cached colorschemes'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s l -d 'Generate a light colorscheme'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s n -d 'Skip setting the wallpaper'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s q -d 'Quiet mode, don\'t print anything'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s r -d '\'wal -r\' is deprecated: Use (cat ~/.cache/wal/sequences &) instead'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s R -d 'Restore previous colorscheme'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s s -d 'Skip changing colors in terminals'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s t -d 'Skip changing colors in tty'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s v -d 'Print "wal" version'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s e -d 'Skip reloading gtk/xrdb/i3/sway/polybar'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s T -l skip-templates -d 'Skip templating process'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s u -l update-current -d 'Only update the current terminal'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s N -l no-config -d 'Won\'t read the config and avoids creating it\'s config path'
complete -c wallust -n "__fish_wallust_using_subcommand pywal" -s h -l help -d 'Print help'
complete -c wallust -n "__fish_wallust_using_subcommand help; and not __fish_seen_subcommand_from run cs theme migrate debug pywal help" -f -a "run" -d 'Generate a palette from an image'
complete -c wallust -n "__fish_wallust_using_subcommand help; and not __fish_seen_subcommand_from run cs theme migrate debug pywal help" -f -a "cs" -d 'Apply a certain colorscheme'
complete -c wallust -n "__fish_wallust_using_subcommand help; and not __fish_seen_subcommand_from run cs theme migrate debug pywal help" -f -a "theme" -d 'Apply a custom built in theme'
complete -c wallust -n "__fish_wallust_using_subcommand help; and not __fish_seen_subcommand_from run cs theme migrate debug pywal help" -f -a "migrate" -d 'Migrate v2 config to v3 (might lose comments,)'
complete -c wallust -n "__fish_wallust_using_subcommand help; and not __fish_seen_subcommand_from run cs theme migrate debug pywal help" -f -a "debug" -d 'Print information about the program and the enviroment it uses'
complete -c wallust -n "__fish_wallust_using_subcommand help; and not __fish_seen_subcommand_from run cs theme migrate debug pywal help" -f -a "pywal" -d 'A drop-in cli replacement for pywal'
complete -c wallust -n "__fish_wallust_using_subcommand help; and not __fish_seen_subcommand_from run cs theme migrate debug pywal help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
