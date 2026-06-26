{
  description = "animus Rust workspace";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
    octet.url = "git+ssh://git@github.com/OnixResearch/octet.git";
    basalt = {
      url = "git+ssh://git@github.com/OnixResearch/basalt.git?rev=005e1496a4a1be477ba84008ecbcdf8793a236c6";
      flake = false;
    };
    cairn = {
      url = "git+ssh://git@github.com/OnixResearch/cairn.git?rev=24381f77987b7e519ebbf05a62b959290d89c582";
      flake = false;
    };
    trellis = {
      url = "git+ssh://git@github.com/OnixResearch/trellis.git?rev=68ac5824f0ef664e4bedeb8ea92ee938b9e00da0";
      flake = false;
    };
    ucan = {
      url = "git+ssh://git@github.com/OnixResearch/ucan.git?rev=ad61b53e89fa45f9bf7d313ce14c45de645bf53d";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
      octet,
      basalt,
      cairn,
      trellis,
      ucan,
    }:
    let
      eachSystem =
        function: nixpkgs.lib.genAttrs (import systems) (system: function nixpkgs.legacyPackages.${system});
    in
    {
      packages = eachSystem (
        pkgs:
        let
          cargoOctet = octet.packages.${pkgs.stdenv.hostPlatform.system}.cargo-octet;
          system = pkgs.stdenv.hostPlatform.system;
          strictRuntimePath = pkgs.lib.makeBinPath [
            pkgs.coreutils
            pkgs.nix
            pkgs.git
            pkgs.cargo
            pkgs.rustc
            pkgs.gcc
            pkgs.clippy
            cargoOctet
          ];
        in
        {
          cargo-tigerstyle = pkgs.writeShellApplication {
            name = "cargo-tigerstyle";
            runtimeInputs = [ ];
            text = ''
              export PATH=${pkgs.lib.escapeShellArg strictRuntimePath}
              if [ "''${1:-}" = "tigerstyle" ]; then
                shift
              fi
              if [ "''${1:-}" = "check" ]; then
                shift
                exec nix build .#checks.${system}.octet --no-link "$@"
              fi
              exec cargo-octet "$@"
            '';
          };

          check-nickel-configs = pkgs.writeShellApplication {
            name = "check-nickel-configs";
            runtimeInputs = [
              pkgs.diffutils
              pkgs.nickel
            ];
            text = ''
              tmp_dir="$(mktemp -d)"
              trap 'rm -rf "$tmp_dir"' EXIT

              mkdir -p "$tmp_dir/.cargo"
              nickel export --format toml --field cargo_config ${./config/repo.ncl} > "$tmp_dir/.cargo/config.toml"
              nickel export --format toml --field clippy_config ${./config/repo.ncl} > "$tmp_dir/clippy.toml"
              nickel export --format toml --field dylint_config ${./config/repo.ncl} > "$tmp_dir/dylint.toml"
              nickel export --format yaml --field pre_commit_config ${./config/repo.ncl} > "$tmp_dir/.pre-commit-config.yaml"

              diff -u ${./.cargo/config.toml} "$tmp_dir/.cargo/config.toml"
              diff -u ${./clippy.toml} "$tmp_dir/clippy.toml"
              diff -u ${./dylint.toml} "$tmp_dir/dylint.toml"
              diff -u ${./.pre-commit-config.yaml} "$tmp_dir/.pre-commit-config.yaml"
            '';
          };
        }
      );

      apps = eachSystem (pkgs: {
        cargo-tigerstyle = {
          type = "app";
          program = "${
            self.packages.${pkgs.stdenv.hostPlatform.system}.cargo-tigerstyle
          }/bin/cargo-tigerstyle";
          meta.description = "Run the pinned Octet/Tigerstyle cargo subcommand.";
        };

        check-nickel-configs = {
          type = "app";
          program = "${
            self.packages.${pkgs.stdenv.hostPlatform.system}.check-nickel-configs
          }/bin/check-nickel-configs";
          meta.description = "Verify generated config files match the Nickel source of truth.";
        };

        octet = self.apps.${pkgs.stdenv.hostPlatform.system}.cargo-tigerstyle;
      });

      checks = eachSystem (
        pkgs:
        let
          source = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter =
              path: type:
              let
                baseName = builtins.baseNameOf path;
                pathText = builtins.toString path;
                isGenerated = pkgs.lib.hasInfix "/.git" pathText || pkgs.lib.hasInfix "/target" pathText;
              in
              !isGenerated
              && (
                type == "directory"
                || baseName == "Cargo.lock"
                || baseName == "Cargo.toml"
                || baseName == "clippy.toml"
                || baseName == "dylint.toml"
                || baseName == ".pre-commit-config.yaml"
                || baseName == "config.toml"
                || baseName == "README.md"
                || pkgs.lib.hasSuffix ".ncl" baseName
                || pkgs.lib.hasSuffix ".rs" baseName
                || pkgs.lib.hasSuffix ".md" baseName
              );
          };

          cargoDeps =
            (pkgs.rustPlatform.importCargoLock.override {
              fetchgit =
                args:
                if args.url == "ssh://git@github.com/OnixResearch/basalt.git" then
                  basalt
                else if args.url == "ssh://git@github.com/OnixResearch/cairn.git" then
                  cairn
                else if args.url == "ssh://git@github.com/OnixResearch/trellis.git" then
                  trellis
                else if args.url == "ssh://git@github.com/OnixResearch/ucan.git" then
                  ucan
                else
                  pkgs.fetchgit (
                    args
                    // {
                      nativeBuildInputs = (args.nativeBuildInputs or [ ]) ++ [ pkgs.openssh ];
                    }
                  );
            })
              {
                lockFile = ./Cargo.lock;
                outputHashes = {
                  "basalt-0.1.0" = "sha256-kPESje/5w1vk2KKJgFz0Z+RqyxkjJAiGT7mgortULX4=";
                  "cairn-core-0.1.0" = "sha256-Xv9uAhYI+byXcKbqR+A4rESDYmmX44ZWhDLARgqbv5k=";
                  "ucan-0.1.0" = "sha256-Hn0sK7y/lnGNKWxGuqiPOlqni4OTfh+aqhMxXNZ0oTI=";
                  "ucan-core-0.1.0" = "sha256-Hn0sK7y/lnGNKWxGuqiPOlqni4OTfh+aqhMxXNZ0oTI=";
                  "verified-logic-0.1.0" = "sha256-Hn0sK7y/lnGNKWxGuqiPOlqni4OTfh+aqhMxXNZ0oTI=";
                  "verified-logic-0.2.0" = "sha256-xhKyUabrb8Xx6c4vKrWfHmaQDwZITCMNefnxB8mxmwg=";
                };
              };

          cargoNativeBuildInputs = with pkgs; [
            cargo
            rustc
            gcc
          ];

          cargoCheck =
            name: extraNativeBuildInputs: command:
            pkgs.rustPlatform.buildRustPackage {
              pname = name;
              version = "0.1.0";
              src = source;
              inherit cargoDeps;
              nativeBuildInputs = extraNativeBuildInputs;
              doCheck = false;
              buildPhase = ''
                runHook preBuild
                ${command}
                runHook postBuild
              '';
              installPhase = ''
                mkdir -p "$out"
              '';
            };
        in
        {
          cargo-fmt = cargoCheck "animus-cargo-fmt" [ pkgs.rustfmt ] ''
            cargo fmt --all -- --check
          '';

          cargo-check = cargoCheck "animus-cargo-check" [ ] ''
            cargo check --workspace --all-targets --locked
          '';

          cargo-test = cargoCheck "animus-cargo-test" [ ] ''
            cargo test --workspace --locked
          '';

          cargo-basalt-reference = cargoCheck "animus-cargo-basalt-reference" [ ] ''
            cargo check -p animus --features basalt-reference --locked
          '';

          cargo-cairn-reference = cargoCheck "animus-cargo-cairn-reference" [ ] ''
            cargo check -p animus --features cairn-reference --locked
          '';

          cargo-preserves = cargoCheck "animus-cargo-preserves" [ ] ''
            cargo check -p animus-preserves --locked
          '';

          cargo-preserves-feature = cargoCheck "animus-cargo-preserves-feature" [ ] ''
            cargo check -p animus --features preserves --locked
          '';

          cargo-steel = cargoCheck "animus-cargo-steel" [ ] ''
            cargo check -p animus-steel --locked
          '';

          cargo-steel-feature = cargoCheck "animus-cargo-steel-feature" [ ] ''
            cargo check -p animus --features steel --locked
          '';

          cargo-trellis-reference = cargoCheck "animus-cargo-trellis-reference" [ ] ''
            cargo check -p animus --features trellis-reference --locked
          '';

          cargo-clippy = cargoCheck "animus-cargo-clippy" [ pkgs.clippy ] ''
            cargo clippy --workspace --all-targets --locked -- -D warnings
          '';

          nickel-config = pkgs.runCommand "animus-nickel-config" { } ''
            ${self.packages.${pkgs.stdenv.hostPlatform.system}.check-nickel-configs}/bin/check-nickel-configs
            touch "$out"
          '';

          octet = octet.lib.mkConsumerCheck {
            inherit (pkgs.stdenv.hostPlatform) system;
            cargoLock = ./Cargo.lock;
            nativeBuildInputs = cargoNativeBuildInputs ++ [ pkgs.clippy ];
            packages = [
              "animus"
              "animus-adapters"
              "animus-core"
              "animus-preserves"
              "animus-std"
              "animus-steel"
            ];
            src = source;
          };
        }
      );

      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            self.packages.${stdenv.hostPlatform.system}.cargo-tigerstyle
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer
            gcc
            nickel
            pkg-config
            pre-commit
            git
          ];

          RUSTC_WRAPPER = "";
          BASALT_SOURCE = "${basalt}";
          CAIRN_SOURCE = "${cairn}";
          TRELLIS_SOURCE = "${trellis}";
          UCAN_SOURCE = "${ucan}";

          shellHook = ''
            echo "animus dev shell: cargo, clippy, rustfmt, rust-analyzer, nickel, pre-commit, and Octet are available"
            echo "verification: cargo test --workspace; cargo clippy --workspace --all-targets -- -D warnings; cargo tigerstyle check; check-nickel-configs"
          '';
        };
      });
    };
}
