{
  lib,
  nix-gitignore,
  rustPlatform,
  imagemagick,
  makeWrapper,
  installShellFiles,
}: let
  version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version + "-git";
  src = nix-gitignore.gitignoreSource [] ./.;
in
  rustPlatform.buildRustPackage {
    pname = "wallust";
    inherit version src;

    cargoLock.lockFile = ./Cargo.lock;

    nativeBuildInputs = [makeWrapper installShellFiles];

    postInstall = ''
      installManPage man/wallust*
      installShellCompletion --cmd wallust \
        --bash completions/wallust.bash \
        --zsh completions/_wallust \
        --fish completions/wallust.fish
    '';

    postFixup = ''
      wrapProgram $out/bin/wallust \
        --prefix PATH : "${lib.makeBinPath [imagemagick]}"
    '';

    enableParallelBuilding = true;

    meta = {
      description = "A better pywal, written in Rust";
      license = lib.licenses.mit;
      mainProgram = "wallust";
    };
  }
