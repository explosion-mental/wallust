# Distribution Packages

## Arch User Repository (AUR)
Using an Arch based distro, you can use the [wallust](https://aur.archlinux.org/packages/wallust) or [wallust-git](https://aur.archlinux.org/packages/wallust-git) packages.

- `wallust` fetches the latest **stable version** from `static.crates.io`. **Prefer this package.**
- `wallust-git` fetches the latest **unstable version** from the `master`.

Either can be installed on an Arch based distro with the following commands:
```bash
git clone https://aur.archlinux.org/wallust.git # Or wallust-git.git
cd wallust # or wallust-git
makepkg -si
```

## NetBSD
If you are using NetBSD, a native package is available from the official repositories. To install it, simply run:
```
pkgin install wallust
```

## Nix

If you are using Nix, a native package is [available][nix-search].

Install it for your profile:

```bash
nix-env -iA nixos.wallust # change `nixos` for `nixpkgs`, if on a non-NixOS system
```

Try it with `nix-shell`

```bash
nix-shell -p wallust
```

Add the following Nix code to your NixOS Configuration, usually located in `/etc/nixos/configuration.nix`

```nix
  environment.systemPackages = [
    pkgs.wallust
  ];
```

If you are using flakes, you can directly use this repo to get the latest release:

First add this to your `flake.nix`

```nix
  inputs.wallust.url = "git+https://codeberg.org/explosion-mental/wallust?ref=master";
```

Then in your `configuration.nix`

```nix
  environment.systemPackages = [
    inputs.wallust.packages.${pkgs.system}.wallust
  ];
```

You can change ref to a [**tag version**](https://codeberg.org/explosion-mental/wallust/tags) to get a stable release.

[nix-search]: <https://search.nixos.org/packages?channel=unstable&from=0&size=1&sort=relevance&type=packages&query=wallust>
