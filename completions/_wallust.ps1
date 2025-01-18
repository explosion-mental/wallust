
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'wallust' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'wallust'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'wallust' {
            [CompletionResult]::new('-I', '-I ', [CompletionResultType]::ParameterName, 'Won''t send these colors sequences')
            [CompletionResult]::new('--ignore-sequence', '--ignore-sequence', [CompletionResultType]::ParameterName, 'Won''t send these colors sequences')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Use CONFIG_FILE as the config file')
            [CompletionResult]::new('--config-file', '--config-file', [CompletionResultType]::ParameterName, 'Use CONFIG_FILE as the config file')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)')
            [CompletionResult]::new('--config-dir', '--config-dir', [CompletionResultType]::ParameterName, 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)')
            [CompletionResult]::new('--templates-dir', '--templates-dir', [CompletionResultType]::ParameterName, 'Uses TEMPLATE_DIR as the template directory')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Don''t print anything')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Don''t print anything')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Skip setting terminal sequences')
            [CompletionResult]::new('--skip-sequences', '--skip-sequences', [CompletionResultType]::ParameterName, 'Skip setting terminal sequences')
            [CompletionResult]::new('-T', '-T ', [CompletionResultType]::ParameterName, 'Skip templating process')
            [CompletionResult]::new('--skip-templates', '--skip-templates', [CompletionResultType]::ParameterName, 'Skip templating process')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'Only update the current terminal')
            [CompletionResult]::new('--update-current', '--update-current', [CompletionResultType]::ParameterName, 'Only update the current terminal')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'Won''t read the config and avoids creating it''s config path')
            [CompletionResult]::new('--no-config', '--no-config', [CompletionResultType]::ParameterName, 'Won''t read the config and avoids creating it''s config path')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('run', 'run', [CompletionResultType]::ParameterValue, 'Generate a palette from an image')
            [CompletionResult]::new('cs', 'cs', [CompletionResultType]::ParameterValue, 'Apply a certain colorscheme')
            [CompletionResult]::new('theme', 'theme', [CompletionResultType]::ParameterValue, 'Apply a custom built in theme')
            [CompletionResult]::new('migrate', 'migrate', [CompletionResultType]::ParameterValue, 'Migrate v2 config to v3 (might lose comments,)')
            [CompletionResult]::new('debug', 'debug', [CompletionResultType]::ParameterValue, 'Print information about the program and the enviroment it uses')
            [CompletionResult]::new('pywal', 'pywal', [CompletionResultType]::ParameterValue, 'A drop-in cli replacement for pywal')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'wallust;run' {
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'Alpha *template variable* value, used only for templating (default is 100)')
            [CompletionResult]::new('--alpha', '--alpha', [CompletionResultType]::ParameterName, 'Alpha *template variable* value, used only for templating (default is 100)')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'Choose which backend to use (overwrites config)')
            [CompletionResult]::new('--backend', '--backend', [CompletionResultType]::ParameterName, 'Choose which backend to use (overwrites config)')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Choose which colorspace to use (overwrites config)')
            [CompletionResult]::new('--colorspace', '--colorspace', [CompletionResultType]::ParameterName, 'Choose which colorspace to use (overwrites config)')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Choose which fallback generation method to use (overwrites config)')
            [CompletionResult]::new('--fallback-generator', '--fallback-generator', [CompletionResultType]::ParameterName, 'Choose which fallback generation method to use (overwrites config)')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Choose which palette to use (overwrites config)')
            [CompletionResult]::new('--palette', '--palette', [CompletionResultType]::ParameterName, 'Choose which palette to use (overwrites config)')
            [CompletionResult]::new('--saturation', '--saturation', [CompletionResultType]::ParameterName, 'Add saturation from 1% to 100% (overwrites config)')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Choose a custom threshold, between 1 and 100 (overwrites config)')
            [CompletionResult]::new('--threshold', '--threshold', [CompletionResultType]::ParameterName, 'Choose a custom threshold, between 1 and 100 (overwrites config)')
            [CompletionResult]::new('-I', '-I ', [CompletionResultType]::ParameterName, 'Won''t send these colors sequences')
            [CompletionResult]::new('--ignore-sequence', '--ignore-sequence', [CompletionResultType]::ParameterName, 'Won''t send these colors sequences')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Use CONFIG_FILE as the config file')
            [CompletionResult]::new('--config-file', '--config-file', [CompletionResultType]::ParameterName, 'Use CONFIG_FILE as the config file')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)')
            [CompletionResult]::new('--config-dir', '--config-dir', [CompletionResultType]::ParameterName, 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)')
            [CompletionResult]::new('--templates-dir', '--templates-dir', [CompletionResultType]::ParameterName, 'Uses TEMPLATE_DIR as the template directory')
            [CompletionResult]::new('-k', '-k', [CompletionResultType]::ParameterName, 'Ensure a readable contrast by checking colors in reference to the background (overwrites config)')
            [CompletionResult]::new('--check-contrast', '--check-contrast', [CompletionResultType]::ParameterName, 'Ensure a readable contrast by checking colors in reference to the background (overwrites config)')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'Don''t cache the results')
            [CompletionResult]::new('--no-cache', '--no-cache', [CompletionResultType]::ParameterName, 'Don''t cache the results')
            [CompletionResult]::new('--dynamic-threshold', '--dynamic-threshold', [CompletionResultType]::ParameterName, 'Dynamically changes the threshold to be best fit')
            [CompletionResult]::new('-w', '-w', [CompletionResultType]::ParameterName, 'Generates colors even if there is a cache version of it')
            [CompletionResult]::new('--overwrite-cache', '--overwrite-cache', [CompletionResultType]::ParameterName, 'Generates colors even if there is a cache version of it')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Don''t print anything')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Don''t print anything')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Skip setting terminal sequences')
            [CompletionResult]::new('--skip-sequences', '--skip-sequences', [CompletionResultType]::ParameterName, 'Skip setting terminal sequences')
            [CompletionResult]::new('-T', '-T ', [CompletionResultType]::ParameterName, 'Skip templating process')
            [CompletionResult]::new('--skip-templates', '--skip-templates', [CompletionResultType]::ParameterName, 'Skip templating process')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'Only update the current terminal')
            [CompletionResult]::new('--update-current', '--update-current', [CompletionResultType]::ParameterName, 'Only update the current terminal')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'Won''t read the config and avoids creating it''s config path')
            [CompletionResult]::new('--no-config', '--no-config', [CompletionResultType]::ParameterName, 'Won''t read the config and avoids creating it''s config path')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            break
        }
        'wallust;cs' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Specify a custom format. Without this option, wallust will sequentially try to decode it by trying one by one')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Specify a custom format. Without this option, wallust will sequentially try to decode it by trying one by one')
            [CompletionResult]::new('-I', '-I ', [CompletionResultType]::ParameterName, 'Won''t send these colors sequences')
            [CompletionResult]::new('--ignore-sequence', '--ignore-sequence', [CompletionResultType]::ParameterName, 'Won''t send these colors sequences')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Use CONFIG_FILE as the config file')
            [CompletionResult]::new('--config-file', '--config-file', [CompletionResultType]::ParameterName, 'Use CONFIG_FILE as the config file')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)')
            [CompletionResult]::new('--config-dir', '--config-dir', [CompletionResultType]::ParameterName, 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)')
            [CompletionResult]::new('--templates-dir', '--templates-dir', [CompletionResultType]::ParameterName, 'Uses TEMPLATE_DIR as the template directory')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Don''t print anything')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Don''t print anything')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Skip setting terminal sequences')
            [CompletionResult]::new('--skip-sequences', '--skip-sequences', [CompletionResultType]::ParameterName, 'Skip setting terminal sequences')
            [CompletionResult]::new('-T', '-T ', [CompletionResultType]::ParameterName, 'Skip templating process')
            [CompletionResult]::new('--skip-templates', '--skip-templates', [CompletionResultType]::ParameterName, 'Skip templating process')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'Only update the current terminal')
            [CompletionResult]::new('--update-current', '--update-current', [CompletionResultType]::ParameterName, 'Only update the current terminal')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'Won''t read the config and avoids creating it''s config path')
            [CompletionResult]::new('--no-config', '--no-config', [CompletionResultType]::ParameterName, 'Won''t read the config and avoids creating it''s config path')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            break
        }
        'wallust;theme' {
            [CompletionResult]::new('-I', '-I ', [CompletionResultType]::ParameterName, 'Won''t send these colors sequences')
            [CompletionResult]::new('--ignore-sequence', '--ignore-sequence', [CompletionResultType]::ParameterName, 'Won''t send these colors sequences')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Use CONFIG_FILE as the config file')
            [CompletionResult]::new('--config-file', '--config-file', [CompletionResultType]::ParameterName, 'Use CONFIG_FILE as the config file')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)')
            [CompletionResult]::new('--config-dir', '--config-dir', [CompletionResultType]::ParameterName, 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)')
            [CompletionResult]::new('--templates-dir', '--templates-dir', [CompletionResultType]::ParameterName, 'Uses TEMPLATE_DIR as the template directory')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Only preview the selected theme')
            [CompletionResult]::new('--preview', '--preview', [CompletionResultType]::ParameterName, 'Only preview the selected theme')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Don''t print anything')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Don''t print anything')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Skip setting terminal sequences')
            [CompletionResult]::new('--skip-sequences', '--skip-sequences', [CompletionResultType]::ParameterName, 'Skip setting terminal sequences')
            [CompletionResult]::new('-T', '-T ', [CompletionResultType]::ParameterName, 'Skip templating process')
            [CompletionResult]::new('--skip-templates', '--skip-templates', [CompletionResultType]::ParameterName, 'Skip templating process')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'Only update the current terminal')
            [CompletionResult]::new('--update-current', '--update-current', [CompletionResultType]::ParameterName, 'Only update the current terminal')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'Won''t read the config and avoids creating it''s config path')
            [CompletionResult]::new('--no-config', '--no-config', [CompletionResultType]::ParameterName, 'Won''t read the config and avoids creating it''s config path')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'wallust;migrate' {
            [CompletionResult]::new('-I', '-I ', [CompletionResultType]::ParameterName, 'Won''t send these colors sequences')
            [CompletionResult]::new('--ignore-sequence', '--ignore-sequence', [CompletionResultType]::ParameterName, 'Won''t send these colors sequences')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Use CONFIG_FILE as the config file')
            [CompletionResult]::new('--config-file', '--config-file', [CompletionResultType]::ParameterName, 'Use CONFIG_FILE as the config file')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)')
            [CompletionResult]::new('--config-dir', '--config-dir', [CompletionResultType]::ParameterName, 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)')
            [CompletionResult]::new('--templates-dir', '--templates-dir', [CompletionResultType]::ParameterName, 'Uses TEMPLATE_DIR as the template directory')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Don''t print anything')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Don''t print anything')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Skip setting terminal sequences')
            [CompletionResult]::new('--skip-sequences', '--skip-sequences', [CompletionResultType]::ParameterName, 'Skip setting terminal sequences')
            [CompletionResult]::new('-T', '-T ', [CompletionResultType]::ParameterName, 'Skip templating process')
            [CompletionResult]::new('--skip-templates', '--skip-templates', [CompletionResultType]::ParameterName, 'Skip templating process')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'Only update the current terminal')
            [CompletionResult]::new('--update-current', '--update-current', [CompletionResultType]::ParameterName, 'Only update the current terminal')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'Won''t read the config and avoids creating it''s config path')
            [CompletionResult]::new('--no-config', '--no-config', [CompletionResultType]::ParameterName, 'Won''t read the config and avoids creating it''s config path')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'wallust;debug' {
            [CompletionResult]::new('-I', '-I ', [CompletionResultType]::ParameterName, 'Won''t send these colors sequences')
            [CompletionResult]::new('--ignore-sequence', '--ignore-sequence', [CompletionResultType]::ParameterName, 'Won''t send these colors sequences')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Use CONFIG_FILE as the config file')
            [CompletionResult]::new('--config-file', '--config-file', [CompletionResultType]::ParameterName, 'Use CONFIG_FILE as the config file')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)')
            [CompletionResult]::new('--config-dir', '--config-dir', [CompletionResultType]::ParameterName, 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)')
            [CompletionResult]::new('--templates-dir', '--templates-dir', [CompletionResultType]::ParameterName, 'Uses TEMPLATE_DIR as the template directory')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Don''t print anything')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Don''t print anything')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Skip setting terminal sequences')
            [CompletionResult]::new('--skip-sequences', '--skip-sequences', [CompletionResultType]::ParameterName, 'Skip setting terminal sequences')
            [CompletionResult]::new('-T', '-T ', [CompletionResultType]::ParameterName, 'Skip templating process')
            [CompletionResult]::new('--skip-templates', '--skip-templates', [CompletionResultType]::ParameterName, 'Skip templating process')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'Only update the current terminal')
            [CompletionResult]::new('--update-current', '--update-current', [CompletionResultType]::ParameterName, 'Only update the current terminal')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'Won''t read the config and avoids creating it''s config path')
            [CompletionResult]::new('--no-config', '--no-config', [CompletionResultType]::ParameterName, 'Won''t read the config and avoids creating it''s config path')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'wallust;pywal' {
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'Set terminal background transparency. *Only works in URxvt*')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'Custom background color to use')
            [CompletionResult]::new('--backend', '--backend', [CompletionResultType]::ParameterName, 'Which color backend to use')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Which colorscheme file to use. Use ''wal --theme'' to list builtin themes')
            [CompletionResult]::new('--theme', '--theme', [CompletionResultType]::ParameterName, 'Which colorscheme file to use. Use ''wal --theme'' to list builtin themes')
            [CompletionResult]::new('--saturate', '--saturate', [CompletionResultType]::ParameterName, 'Set the color saturation')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Which image or directory to use')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'External script to run after "wal"')
            [CompletionResult]::new('-I', '-I ', [CompletionResultType]::ParameterName, 'Won''t send these colors sequences')
            [CompletionResult]::new('--ignore-sequence', '--ignore-sequence', [CompletionResultType]::ParameterName, 'Won''t send these colors sequences')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Use CONFIG_FILE as the config file')
            [CompletionResult]::new('--config-file', '--config-file', [CompletionResultType]::ParameterName, 'Use CONFIG_FILE as the config file')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)')
            [CompletionResult]::new('--config-dir', '--config-dir', [CompletionResultType]::ParameterName, 'Uses CONFIG_DIR as the config directory, which holds both `wallust.toml` and the templates files (if existent)')
            [CompletionResult]::new('--templates-dir', '--templates-dir', [CompletionResultType]::ParameterName, 'Uses TEMPLATE_DIR as the template directory')
            [CompletionResult]::new('--iterative', '--iterative', [CompletionResultType]::ParameterName, 'When pywal is given a directory as input and this flag is used: Go through the images in order instead of shuffled')
            [CompletionResult]::new('--preview', '--preview', [CompletionResultType]::ParameterName, 'Print the current color palette')
            [CompletionResult]::new('--vte', '--vte', [CompletionResultType]::ParameterName, 'Fix text-artifacts printed in VTE terminals')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Delete all cached colorschemes')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Generate a light colorscheme')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'Skip setting the wallpaper')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Quiet mode, don''t print anything')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, '''wal -r'' is deprecated: Use (cat ~/.cache/wal/sequences &) instead')
            [CompletionResult]::new('-R', '-R ', [CompletionResultType]::ParameterName, 'Restore previous colorscheme')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Skip changing colors in terminals')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Skip changing colors in tty')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print "wal" version')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'Skip reloading gtk/xrdb/i3/sway/polybar')
            [CompletionResult]::new('-T', '-T ', [CompletionResultType]::ParameterName, 'Skip templating process')
            [CompletionResult]::new('--skip-templates', '--skip-templates', [CompletionResultType]::ParameterName, 'Skip templating process')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'Only update the current terminal')
            [CompletionResult]::new('--update-current', '--update-current', [CompletionResultType]::ParameterName, 'Only update the current terminal')
            [CompletionResult]::new('-N', '-N ', [CompletionResultType]::ParameterName, 'Won''t read the config and avoids creating it''s config path')
            [CompletionResult]::new('--no-config', '--no-config', [CompletionResultType]::ParameterName, 'Won''t read the config and avoids creating it''s config path')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'wallust;help' {
            [CompletionResult]::new('run', 'run', [CompletionResultType]::ParameterValue, 'Generate a palette from an image')
            [CompletionResult]::new('cs', 'cs', [CompletionResultType]::ParameterValue, 'Apply a certain colorscheme')
            [CompletionResult]::new('theme', 'theme', [CompletionResultType]::ParameterValue, 'Apply a custom built in theme')
            [CompletionResult]::new('migrate', 'migrate', [CompletionResultType]::ParameterValue, 'Migrate v2 config to v3 (might lose comments,)')
            [CompletionResult]::new('debug', 'debug', [CompletionResultType]::ParameterValue, 'Print information about the program and the enviroment it uses')
            [CompletionResult]::new('pywal', 'pywal', [CompletionResultType]::ParameterValue, 'A drop-in cli replacement for pywal')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'wallust;help;run' {
            break
        }
        'wallust;help;cs' {
            break
        }
        'wallust;help;theme' {
            break
        }
        'wallust;help;migrate' {
            break
        }
        'wallust;help;debug' {
            break
        }
        'wallust;help;pywal' {
            break
        }
        'wallust;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
