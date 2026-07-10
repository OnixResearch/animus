{
  description = "animus Rust workspace";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
    octet.url = "git+ssh://git@github.com/OnixResearch/octet.git";
    rats = {
      url = "git+ssh://git@github.com/brittonr/rats.git?rev=e9cb7865fe400de6220aea8bfc08ddf4f75da3d3";
      flake = false;
    };
    basalt = {
      url = "git+ssh://git@github.com/OnixResearch/basalt.git?rev=005e1496a4a1be477ba84008ecbcdf8793a236c6";
      flake = false;
    };
    cairn = {
      url = "git+ssh://git@github.com/OnixResearch/cairn.git?rev=fb1a7403a7897f7fa161e0b3c5d86b4cf19e52f8";
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
      rats,
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

          cairn = pkgs.writeShellApplication {
            name = "cairn";
            runtimeInputs = [ pkgs.nix ];
            text = ''
              policy_path=${pkgs.lib.escapeShellArg "${cairn}/cairn-policy/generated/cairn-policy.json"}
              source_flake=${pkgs.lib.escapeShellArg "path:${cairn}"}

              has_policy_arg=false
              for argument in "$@"; do
                if [ "$argument" = "--policy" ]; then
                  has_policy_arg=true
                  break
                fi
              done

              command="''${1:-}"
              case "$command" in
                doctor|validate|release-readiness|policy|probe|steel-orchestration|migrate|gate|sync|archive)
                  if [ "$has_policy_arg" = false ]; then
                    exec nix run "$source_flake#cairn" -- "$@" --policy "$policy_path"
                  fi
                  ;;
              esac

              exec nix run "$source_flake#cairn" -- "$@"
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

        cairn = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn";
          meta.description = "Run the pinned Cairn lifecycle CLI with Animus' locked Cairn policy.";
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
                || baseName == "check-clankers-product-surfaces"
                || baseName == "check-dogfood-readiness"
                || baseName == "check-adapter-conformance-index"
                || baseName == "check-adapter-family-crates"
                || baseName == "check-ast-grep-agent-boundary-audits"
                || baseName == "check-component-catalog"
                || baseName == "check-clash-ucan-authority"
                || baseName == "check-composition-conformance"
                || baseName == "check-current-head-release-evidence-index"
                || baseName == "check-exec-policy"
                || baseName == "check-packaged-headless-dogfood-smoke"
                || baseName == "check-plugin-manifest-runtime-neutral"
                || baseName == "check-request-lifecycle"
                || baseName == "check-router-policy"
                || baseName == "check-steel-runtime-policy"
                || baseName == "check-syndicate-dataspace-isolation"
                || baseName == "check-trellis-verified-core"
                || baseName == "check-neutral-ledger"
                || baseName == "check-injected-services"
                || baseName == "check-provenance-ledger"
                || baseName == "check-release-evidence-index"
                || baseName == "check-runtime-facade-inventory"
                || baseName == "check-runtime-kernel-split"
                || baseName == "check-safe-receipts-artifacts"
                || baseName == "check-wasm-surface"
                || baseName == "generate-current-head-release-evidence-index"
                || pkgs.lib.hasSuffix ".json" baseName
                || pkgs.lib.hasSuffix ".ncl" baseName
                || pkgs.lib.hasSuffix ".rs" baseName
                || pkgs.lib.hasSuffix ".scm" baseName
                || pkgs.lib.hasSuffix ".md" baseName
                || pkgs.lib.hasSuffix ".txt" baseName
                || pkgs.lib.hasSuffix ".yml" baseName
              );
          };

          cargoOctet = octet.packages.${pkgs.stdenv.hostPlatform.system}.cargo-octet;
          octetDriver = octet.packages.${pkgs.stdenv.hostPlatform.system}.dylint-driver;
          octetPkg = octet.packages.${pkgs.stdenv.hostPlatform.system}.default;
          octetToolchainChannel =
            (builtins.fromTOML (builtins.readFile "${octet}/rust-toolchain.toml")).toolchain.channel;
          octetHostTriple = pkgs.stdenv.hostPlatform.rust.rustcTarget;
          octetRustupToolchain =
            if pkgs.lib.hasSuffix octetHostTriple octetToolchainChannel then
              octetToolchainChannel
            else
              "${octetToolchainChannel}-${octetHostTriple}";
          octetSharedLibSuffix = if pkgs.stdenv.hostPlatform.isDarwin then ".dylib" else ".so";
          octetDllPrefix = if pkgs.stdenv.hostPlatform.isDarwin then "lib" else "lib";
          octetDylintLibDir = pkgs.runCommand "animus-octet-dylint-lib" { } ''
            mkdir -p "$out/lib"
            ln -s ${octetPkg}/lib/${octetDllPrefix}octet${octetSharedLibSuffix} \
              "$out/lib/${octetDllPrefix}octet@${octetRustupToolchain}${octetSharedLibSuffix}"
          '';
          octetDylintLibPath = "${octetDylintLibDir}/lib/${octetDllPrefix}octet@${octetRustupToolchain}${octetSharedLibSuffix}";

          cargoLockSpec = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "basalt-0.1.0" = "sha256-kPESje/5w1vk2KKJgFz0Z+RqyxkjJAiGT7mgortULX4=";
              "cairn-core-0.1.0" = "sha256-Xv9uAhYI+byXcKbqR+A4rESDYmmX44ZWhDLARgqbv5k=";
              "rat-chrome-0.1.0" = "sha256-Ct58ZfWnIe7KJ6q/a+gUMVCa4ylTuSv7ZnjI1IYHzgY=";
              "rat-widgets-0.1.0" = "sha256-Ct58ZfWnIe7KJ6q/a+gUMVCa4ylTuSv7ZnjI1IYHzgY=";
              "ucan-0.1.0" = "sha256-Hn0sK7y/lnGNKWxGuqiPOlqni4OTfh+aqhMxXNZ0oTI=";
              "ucan-core-0.1.0" = "sha256-Hn0sK7y/lnGNKWxGuqiPOlqni4OTfh+aqhMxXNZ0oTI=";
              "verified-logic-0.1.0" = "sha256-Hn0sK7y/lnGNKWxGuqiPOlqni4OTfh+aqhMxXNZ0oTI=";
              "verified-logic-0.2.0" = "sha256-xhKyUabrb8Xx6c4vKrWfHmaQDwZITCMNefnxB8mxmwg=";
            };
          };

          cargoDeps =
            (pkgs.rustPlatform.importCargoLock.override {
              fetchgit =
                args:
                if args.url == "ssh://git@github.com/brittonr/rats.git" then
                  rats
                else if args.url == "ssh://git@github.com/OnixResearch/basalt.git" then
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
              cargoLockSpec;

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

          cargo-router = cargoCheck "animus-cargo-router" [ ] ''
            cargo check -p animus-router --locked
          '';

          cargo-router-feature = cargoCheck "animus-cargo-router-feature" [ ] ''
            cargo check -p animus --features router --locked
          '';

          router-policy = cargoCheck "animus-router-policy" [ pkgs.nickel ] ''
            sh scripts/cairn/check-router-policy --root .
          '';

          cargo-runtime = cargoCheck "animus-cargo-runtime" [ ] ''
            cargo check -p animus-runtime --locked
          '';

          runtime-kernel-split = cargoCheck "animus-runtime-kernel-split" [ ] ''
            sh scripts/cairn/check-runtime-kernel-split --root .
          '';

          adapter-family-crates = cargoCheck "animus-adapter-family-crates" [ ] ''
            sh scripts/cairn/check-adapter-family-crates --root .
          '';

          ast-grep-agent-boundary-audits = cargoCheck "animus-ast-grep-agent-boundary-audits" [
            pkgs.ast-grep
            pkgs.b3sum
            pkgs.nickel
          ] ''
            sh scripts/cairn/check-ast-grep-agent-boundary-audits --root .
          '';

          cargo-runtime-feature = cargoCheck "animus-cargo-runtime-feature" [ ] ''
            cargo check -p animus --features runtime --locked
          '';

          cargo-steel = cargoCheck "animus-cargo-steel" [ ] ''
            cargo check -p animus-steel --locked
          '';

          cargo-steel-feature = cargoCheck "animus-cargo-steel-feature" [ ] ''
            cargo check -p animus --features steel --locked
          '';

          cargo-tui = cargoCheck "animus-cargo-tui" [ ] ''
            cargo check -p animus-tui --locked
          '';

          cargo-tui-feature = cargoCheck "animus-cargo-tui-feature" [ ] ''
            cargo check -p animus --features tui --locked
          '';

          cargo-trellis-reference = cargoCheck "animus-cargo-trellis-reference" [ ] ''
            cargo check -p animus --features trellis-reference --locked
          '';

          wasm-surface = cargoCheck "animus-wasm-surface" [ ] ''
            sh scripts/cairn/check-wasm-surface --root .
          '';

          safe-receipts-artifacts = cargoCheck "animus-safe-receipts-artifacts" [ ] ''
            sh scripts/cairn/check-safe-receipts-artifacts --root .
          '';

          neutral-ledger = cargoCheck "animus-neutral-ledger" [ ] ''
            sh scripts/cairn/check-neutral-ledger --root .
          '';

          provenance-ledger = cargoCheck "animus-provenance-ledger" [ ] ''
            sh scripts/cairn/check-provenance-ledger --root .
          '';

          steel-runtime-policy = cargoCheck "animus-steel-runtime-policy" [ pkgs.nickel ] ''
            sh scripts/cairn/check-steel-runtime-policy --root .
          '';

          exec-policy = cargoCheck "animus-exec-policy" [ pkgs.nickel ] ''
            sh scripts/cairn/check-exec-policy --root .
          '';

          injected-services = cargoCheck "animus-injected-services" [ ] ''
            sh scripts/cairn/check-injected-services --root .
          '';

          composition-conformance = cargoCheck "animus-composition-conformance" [ pkgs.b3sum ] ''
            sh scripts/cairn/check-composition-conformance --root .
          '';

          adapter-conformance-index =
            pkgs.runCommand "animus-adapter-conformance-index"
              {
                nativeBuildInputs = [
                  pkgs.rustc
                  pkgs.gcc
                  pkgs.nickel
                  pkgs.b3sum
                ];
              }
              ''
                cp -r ${source} source
                chmod -R u+w source
                cd source
                sh scripts/cairn/check-adapter-conformance-index --root .
                touch "$out"
              '';

          component-catalog =
            pkgs.runCommand "animus-component-catalog"
              {
                nativeBuildInputs = [
                  pkgs.rustc
                  pkgs.gcc
                  pkgs.nickel
                  pkgs.b3sum
                ];
              }
              ''
                cp -r ${source} source
                chmod -R u+w source
                cd source
                sh scripts/cairn/check-component-catalog --root .
                touch "$out"
              '';

          clash-ucan-authority = cargoCheck "animus-clash-ucan-authority" [ pkgs.nickel ] ''
            sh scripts/cairn/check-clash-ucan-authority --root .
          '';

          trellis-verified-core = cargoCheck "animus-trellis-verified-core" [ ] ''
            sh scripts/cairn/check-trellis-verified-core --root .
          '';

          syndicate-dataspace-isolation = cargoCheck "animus-syndicate-dataspace-isolation" [ ] ''
            sh scripts/cairn/check-syndicate-dataspace-isolation --root .
          '';

          cargo-clippy = cargoCheck "animus-cargo-clippy" [ pkgs.clippy ] ''
            cargo clippy --workspace --all-targets --locked -- -D warnings
          '';

          nickel-config = pkgs.runCommand "animus-nickel-config" { } ''
            ${self.packages.${pkgs.stdenv.hostPlatform.system}.check-nickel-configs}/bin/check-nickel-configs
            touch "$out"
          '';

          clankers-product-surfaces =
            pkgs.runCommand "animus-clankers-product-surfaces"
              {
                nativeBuildInputs = [
                  pkgs.rustc
                  pkgs.gcc
                ];
              }
              ''
                cp -r ${source} source
                chmod -R u+w source
                cd source
                sh scripts/cairn/check-clankers-product-surfaces --root .
                touch "$out"
              '';

          dogfood-readiness =
            pkgs.runCommand "animus-dogfood-readiness"
              {
                nativeBuildInputs = [
                  pkgs.rustc
                  pkgs.gcc
                ];
              }
              ''
                cp -r ${source} source
                chmod -R u+w source
                cd source
                sh scripts/cairn/check-dogfood-readiness --root .
                touch "$out"
              '';

          packaged-headless-dogfood-smoke = cargoCheck "animus-packaged-headless-dogfood-smoke" [ ] ''
            sh scripts/cairn/check-packaged-headless-dogfood-smoke --root .
          '';

          plugin-manifest-runtime-neutral =
            pkgs.runCommand "animus-plugin-manifest-runtime-neutral"
              {
                nativeBuildInputs = [
                  pkgs.rustc
                  pkgs.gcc
                ];
              }
              ''
                cp -r ${source} source
                chmod -R u+w source
                cd source
                sh scripts/cairn/check-plugin-manifest-runtime-neutral --root .
                touch "$out"
              '';

          request-lifecycle =
            pkgs.runCommand "animus-request-lifecycle"
              {
                nativeBuildInputs = [
                  pkgs.rustc
                  pkgs.gcc
                ];
              }
              ''
                cp -r ${source} source
                chmod -R u+w source
                cd source
                sh scripts/cairn/check-request-lifecycle --root .
                touch "$out"
              '';

          runtime-facade-inventory =
            pkgs.runCommand "animus-runtime-facade-inventory"
              {
                nativeBuildInputs = [
                  pkgs.rustc
                  pkgs.gcc
                  pkgs.nickel
                  pkgs.b3sum
                ];
              }
              ''
                cp -r ${source} source
                chmod -R u+w source
                cd source
                sh scripts/cairn/check-runtime-facade-inventory --root .
                touch "$out"
              '';

          release-evidence-index =
            pkgs.runCommand "animus-release-evidence-index"
              {
                nativeBuildInputs = [
                  pkgs.rustc
                  pkgs.gcc
                  pkgs.nickel
                  pkgs.b3sum
                ];
              }
              ''
                cp -r ${source} source
                chmod -R u+w source
                cd source
                sh scripts/cairn/check-release-evidence-index --root .
                touch "$out"
              '';

          current-head-release-evidence-index =
            pkgs.runCommand "animus-current-head-release-evidence-index"
              {
                nativeBuildInputs = [
                  pkgs.rustc
                  pkgs.gcc
                ];
              }
              ''
                cp -r ${source} source
                chmod -R u+w source
                cd source
                sh scripts/cairn/check-current-head-release-evidence-index --root .
                touch "$out"
              '';

          # Octet currently reports a vendored serde_core module-name finding
          # while linting crates that pull Steel/Preserves through a store-backed
          # Cargo vendor directory. Cargo/clippy checks still cover those crates.
          octet =
            pkgs.runCommand "animus-octet"
              {
                nativeBuildInputs = cargoNativeBuildInputs ++ [
                  pkgs.clippy
                  cargoOctet
                ];
              }
              ''
                export HOME="$PWD/home"
                export CARGO_HOME="$PWD/.cargo-home"
                mkdir -p "$HOME" "$CARGO_HOME"
                sed 's|directory = "cargo-vendor-dir"|directory = "${cargoDeps}"|' \
                  "${cargoDeps}/.cargo/config.toml" > "$CARGO_HOME/config.toml"
                cp -r ${source} source
                chmod -R u+w source
                cd source
                export TIGERSTYLE_DRIVER_PATH="${octetDriver}/bin/dylint-driver"
                export TIGERSTYLE_LINT_LIB="${octetDylintLibPath}"
                export TIGERSTYLE_TOOLCHAIN="${octetRustupToolchain}"
                export RUSTFLAGS="''${RUSTFLAGS:-} --cfg animus_octet"
                cargo-octet check \
                  -p animus \
                  -p animus-adapters \
                  -p animus-core \
                  -p animus-router \
                  -p animus-runtime \
                  -p animus-std \
                  -p animus-tui
                mkdir -p "$out"
              '';
        }
      );

      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            self.packages.${stdenv.hostPlatform.system}.cargo-tigerstyle
            self.packages.${stdenv.hostPlatform.system}.cairn
            b3sum
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
          RATS_SOURCE = "${rats}";
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
