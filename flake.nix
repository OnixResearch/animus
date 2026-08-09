{
  description = "animus Rust workspace";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
    nickel-export = {
      url = "github:OnixResearch/nickel-export/257fafc1c746f1faf156207043a4c826bfb16d49";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    octet.url = "git+ssh://git@github.com/OnixResearch/octet.git";
    host-control = {
      url = "git+ssh://git@github.com/OnixResearch/host-control.git?rev=e359ccb80a52efb860307278510fa5297ac970ed";
      flake = false;
    };
    rats = {
      url = "git+ssh://git@github.com/brittonr/rats.git?rev=3b498c3d7c248c335d3e7faf12f133a18c87d632";
      flake = false;
    };
    basalt = {
      url = "git+ssh://git@github.com/OnixResearch/basalt.git?rev=e9891e32dcdf655c6e12d0bea2a420f7cdca31c7";
      flake = false;
    };
    cairn = {
      url = "git+ssh://git@github.com/OnixResearch/cairn.git?rev=cf5c4a5c10361ca20a13addf14954407d6857028";
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
      nickel-export,
      octet,
      host-control,
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
              nickel-export.packages.${system}.default
            ];
            text = ''
              tmp_dir="$(mktemp -d)"
              trap 'rm -rf "$tmp_dir"' EXIT

              ANIMUS_REPO_ROOT=${./.} ${pkgs.runtimeShell} \
                ${./scripts/config/export-nickel-configs} "$tmp_dir"

              diff -u ${./.cargo/config.toml} "$tmp_dir/.cargo/config.toml"
              diff -u ${./clippy.toml} "$tmp_dir/clippy.toml"
              diff -u ${./dylint.toml} "$tmp_dir/dylint.toml"
              diff -u ${./.pre-commit-config.yaml} "$tmp_dir/.pre-commit-config.yaml"
              diff -u ${./config/generated/agent-check-registry.json} "$tmp_dir/config/generated/agent-check-registry.json"
              diff -u ${./policy/recipe-workflows/generated/recipe-workflows.json} "$tmp_dir/policy/recipe-workflows/generated/recipe-workflows.json"
              diff -u ${./config/generated/model-runtime-profiles.json} "$tmp_dir/config/generated/model-runtime-profiles.json"
              diff -u ${./config/generated/agent-role-profiles.json} "$tmp_dir/config/generated/agent-role-profiles.json"
              diff -u ${./config/generated/aeneas-proof-campaign.json} "$tmp_dir/config/generated/aeneas-proof-campaign.json"
              diff -u ${./config/generated/active-turn-recovery-policy.json} "$tmp_dir/config/generated/active-turn-recovery-policy.json"
              diff -u ${./config/generated/conversation-projection-policy.json} "$tmp_dir/config/generated/conversation-projection-policy.json"
              diff -u ${./config/generated/provider-retry-policy.json} "$tmp_dir/config/generated/provider-retry-policy.json"
              diff -u ${./config/generated/session-tree-export-policy.json} "$tmp_dir/config/generated/session-tree-export-policy.json"
              diff -u ${./config/generated/durable-session-store-policy.json} "$tmp_dir/config/generated/durable-session-store-policy.json"
              diff -u ${./config/generated/session-authority-profile.json} "$tmp_dir/config/generated/session-authority-profile.json"
              diff -u ${./config/generated/rlm-kernel-profile.json} "$tmp_dir/config/generated/rlm-kernel-profile.json"
              diff -u ${./config/generated/lean-loop-profile.json} "$tmp_dir/config/generated/lean-loop-profile.json"
              diff -u ${./config/generated/agent-guardrail-profile.json} "$tmp_dir/config/generated/agent-guardrail-profile.json"
              diff -u ${./config/generated/codebase-graph-profile.json} "$tmp_dir/config/generated/codebase-graph-profile.json"
              diff -u ${./config/generated/mesh-provider-profile.json} "$tmp_dir/config/generated/mesh-provider-profile.json"
              diff -u ${./config/generated/spacewasm-embedded-compat.json} "$tmp_dir/config/generated/spacewasm-embedded-compat.json"
              diff -u ${./config/generated/harbor-evaluation-profile.json} "$tmp_dir/config/generated/harbor-evaluation-profile.json"
              diff -u ${./config/generated/native-evaluation-policy.json} "$tmp_dir/config/generated/native-evaluation-policy.json"
              diff -u ${./config/generated/host-control-source.json} "$tmp_dir/config/generated/host-control-source.json"
              diff -u ${./config/generated/continual-harness-policy.json} "$tmp_dir/config/generated/continual-harness-policy.json"
              diff -u ${./config/generated/daemon-workers-policy.json} "$tmp_dir/config/generated/daemon-workers-policy.json"
              diff -u ${./config/generated/messaging-receipts-policy.json} "$tmp_dir/config/generated/messaging-receipts-policy.json"
              diff -u ${./config/generated/persistent-goals-policy.json} "$tmp_dir/config/generated/persistent-goals-policy.json"
              diff -u ${./config/generated/schedule-reliability-policy.json} "$tmp_dir/config/generated/schedule-reliability-policy.json"
              diff -u ${./config/generated/session-event-replay-policy.json} "$tmp_dir/config/generated/session-event-replay-policy.json"
              diff -u ${./config/generated/execution-backend-selection-policy.json} "$tmp_dir/config/generated/execution-backend-selection-policy.json"
              diff -u ${./config/generated/provider-video-payload-policy.json} "$tmp_dir/config/generated/provider-video-payload-policy.json"
              diff -u ${./config/generated/terminal-image-policy.json} "$tmp_dir/config/generated/terminal-image-policy.json"
              diff -u ${./config/generated/terminal-video-policy.json} "$tmp_dir/config/generated/terminal-video-policy.json"

              nickel typecheck ${./config/nickel-export-requests.ncl}
              NICKEL_EXPORT_BIN=${nickel-export.packages.${system}.default}/bin/nickel-export \
                ${pkgs.runtimeShell} ${./scripts/config/check-standalone-nickel-exports} ${./.} --check
              NICKEL_EXPORT_BIN=${nickel-export.packages.${system}.default}/bin/nickel-export \
                ${pkgs.runtimeShell} ${./scripts/config/check-standalone-nickel-export-negatives}
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
                agent-receipt|doctor|validate|release-readiness|policy|probe|steel-orchestration|tracey|migrate|gate|sync|archive)
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
                || baseName == "flake.nix"
                || baseName == "flake.lock"
                || baseName == ".pre-commit-config.yaml"
                || baseName == "config.toml"
                || baseName == "README.md"
                || baseName == "check-clankers-product-surfaces"
                || baseName == "check-dogfood-readiness"
                || baseName == "check-durable-session-store"
                || baseName == "check-coding-session-host"
                || baseName == "check-agent-control"
                || baseName == "check-codebase-graph"
                || baseName == "check-mesh-provider"
                || baseName == "check-spacewasm-compat"
                || baseName == "check-harbor-evaluation"
                || baseName == "check-host-control-extraction"
                || baseName == "check-branchfs-host"
                || baseName == "check-acp-stdio-host"
                || baseName == "check-acp-nominal-identities"
                || baseName == "check-lean4-loop-fabric"
                || baseName == "check-session-authority-profile"
                || baseName == "check-runtime-authority-seal"
                || baseName == "check-rejected-attempt-context"
                || baseName == "check-rlm-kernel"
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
                || baseName == "check-preserves-boundary-schemas"
                || baseName == "check-request-lifecycle"
                || baseName == "check-router-policy"
                || baseName == "check-steel-runtime-policy"
                || baseName == "check-syndicate-dataspace-isolation"
                || baseName == "check-trellis-verified-core"
                || baseName == "check-neutral-ledger"
                || baseName == "check-nickel-policy-contracts"
                || baseName == "check-continual-harness"
                || baseName == "check-daemon-workers"
                || baseName == "check-daemon-workers.rs"
                || baseName == "check-messaging-receipts"
                || baseName == "check-messaging-receipts.rs"
                || baseName == "check-persistent-goals"
                || baseName == "check-persistent-goals.rs"
                || baseName == "check-schedule-reliability"
                || baseName == "check-schedule-reliability.rs"
                || baseName == "check-session-event-replay"
                || baseName == "check-session-event-replay.rs"
                || baseName == "check-execution-backend-selection"
                || baseName == "check-execution-backend-selection.rs"
                || baseName == "check-provider-video-payloads"
                || baseName == "check-provider-video-payloads.rs"
                || baseName == "check-terminal-image-protocols"
                || baseName == "check-terminal-image-protocols.rs"
                || baseName == "check-terminal-video-playback"
                || baseName == "check-terminal-video-playback.rs"
                || baseName == "check-native-evaluation-core"
                || baseName == "check-native-evaluation-core.rs"
                || baseName == "check-evaluation-adapter-family"
                || baseName == "check-evaluation-adapter-family.rs"
                || baseName == "check-std-evaluation-host"
                || baseName == "check-std-evaluation-host.rs"
                || baseName == "check-evaluation-composition-conformance"
                || baseName == "check-harbor-atif-adapter"
                || baseName == "check-harbor-atif-adapter.rs"
                || baseName == "check-basalt-authority-adapter"
                || baseName == "check-evidence-safety"
                || baseName == "check-model-runtime-profiles"
                || baseName == "check-agent-role-profiles"
                || baseName == "check-aeneas-proof-campaign"
                || baseName == "check-authoritative-agent-completion"
                || baseName == "check-active-turn-recovery"
                || baseName == "check-conversation-projection"
                || baseName == "check-session-branch-explorer"
                || baseName == "check-injected-services"
                || baseName == "check-interrupted-tool-repair"
                || baseName == "check-provider-retry"
                || baseName == "check-sandlock-host"
                || baseName == "check-bounded-execution-evaluation"
                || baseName == "check-application-host-binding"
                || baseName == "check-agent-guardrail"
                || baseName == "check-session-tree-export"
                || baseName == "check-provenance-ledger"
                || baseName == "check-release-evidence-index"
                || baseName == "check-runtime-facade-inventory"
                || baseName == "check-runtime-kernel-split"
                || baseName == "check-safe-receipts-artifacts"
                || baseName == "check-wasm-surface"
                || baseName == "export-policy-projections"
                || baseName == "generate-current-head-release-evidence-index"
                || baseName == "lean-toolchain"
                || pkgs.lib.hasSuffix ".json" baseName
                || pkgs.lib.hasSuffix ".lines" baseName
                || pkgs.lib.hasSuffix ".lean" baseName
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

          cargoLock = builtins.fromTOML (builtins.readFile ./Cargo.lock);
          renderCargoLockPackage = package:
            let
              renderField = name: "${name} = ${builtins.toJSON package.${name}}";
              scalarFields = builtins.filter (name: builtins.hasAttr name package) [
                "name"
                "version"
                "source"
                "checksum"
              ];
              dependencyField = pkgs.lib.optional (package ? dependencies) ''
                dependencies = [
                ${pkgs.lib.concatMapStringsSep "\n" (dependency: " ${builtins.toJSON dependency},") package.dependencies}
                ]'';
            in
            pkgs.lib.concatStringsSep "\n" (
              [ "[[package]]" ] ++ map renderField scalarFields ++ dependencyField
            );
          cargoLockContentsFor = sourcePredicate: ''
            version = ${builtins.toString cargoLock.version}

            ${pkgs.lib.concatMapStringsSep "\n\n" renderCargoLockPackage (
              builtins.filter (package: package ? source && sourcePredicate package.source) cargoLock.package
            )}
          '';
          importCargoLock = pkgs.rustPlatform.importCargoLock.override {
            fetchgit =
              args:
              if args.url == "ssh://git@github.com/brittonr/rats.git" then
                rats
              else if args.url == "ssh://git@github.com/OnixResearch/host-control.git" then
                host-control
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
          };
          cargoDependencyGroups = [
            {
              name = "registry";
              deps = importCargoLock {
                lockFileContents = cargoLockContentsFor (pkgs.lib.hasPrefix "registry+");
              };
            }
            {
              name = "basalt";
              deps = importCargoLock {
                lockFileContents = cargoLockContentsFor (pkgs.lib.hasInfix "/basalt.git");
                outputHashes."basalt-0.1.0" = "sha256-TtKH39yQ+adQdfu3wvh3n2D5DTaJjQ6rBcIAWD140EE=";
              };
            }
            {
              name = "cairn";
              deps = importCargoLock {
                lockFileContents = cargoLockContentsFor (pkgs.lib.hasInfix "/cairn.git");
                outputHashes."cairn-core-0.1.0" = "sha256-Xv9uAhYI+byXcKbqR+A4rESDYmmX44ZWhDLARgqbv5k=";
              };
            }
            {
              name = "host-control";
              deps = importCargoLock {
                lockFileContents = cargoLockContentsFor (pkgs.lib.hasInfix "/host-control.git");
                outputHashes."host-control-core-0.1.0" = "sha256-oQkddmPvU3iA7jiCkQiOq0OUFQH9ufLgrsNI1zY54zo=";
              };
            }
            {
              name = "rats";
              deps = importCargoLock {
                lockFileContents = cargoLockContentsFor (pkgs.lib.hasInfix "/rats.git");
                outputHashes."rat-chrome-0.1.0" = "sha256-Ct58ZfWnIe7KJ6q/a+gUMVCa4ylTuSv7ZnjI1IYHzgY=";
              };
            }
            {
              name = "sandlock";
              deps = importCargoLock {
                lockFileContents = cargoLockContentsFor (pkgs.lib.hasInfix "/sandlock.git");
                outputHashes."sandlock-core-0.7.0" = "sha256-pAFP4xo7ClmQBFi+kHoQ+xauXZEJvg0D6cGpkZNkAGQ=";
              };
            }
            {
              name = "trellis";
              deps = importCargoLock {
                lockFileContents = cargoLockContentsFor (pkgs.lib.hasInfix "/trellis.git");
                outputHashes."verified-logic-0.2.0" = "sha256-xhKyUabrb8Xx6c4vKrWfHmaQDwZITCMNefnxB8mxmwg=";
              };
            }
          ];
          linkCargoDependencyGroup = group: ''
            mkdir -p "$out/${group.name}"
            for crate in ${group.deps}/*; do
              crate_name="$(basename "$crate")"
              if [ "$crate_name" != "Cargo.lock" ]; then
                ln -s "$crate" "$out/${group.name}/$crate_name"
              fi
            done
          '' + pkgs.lib.optionalString (group.name != "registry") ''
            sed -n '/^\[source\."git+/,$p' ${group.deps}/.cargo/config.toml \
              | sed 's/replace-with = "vendored-sources"/replace-with = "animus-${group.name}"/' \
              >> "$out/.cargo/config.toml"
            cat >> "$out/.cargo/config.toml" <<EOF

            [source.animus-${group.name}]
            directory = "cargo-vendor-dir/${group.name}"
            EOF
          '';
          cargoDeps = pkgs.runCommand "cargo-vendor-dir" { } ''
            mkdir -p "$out/.cargo"
            ln -s ${./Cargo.lock} "$out/Cargo.lock"
            cat > "$out/.cargo/config.toml" <<EOF
            [source.crates-io]
            replace-with = "animus-registry"

            [source.animus-registry]
            directory = "cargo-vendor-dir/registry"
            EOF

            ${pkgs.lib.concatMapStringsSep "\n" linkCargoDependencyGroup cargoDependencyGroups}
          '';

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

          cargo-preserves-adapter = cargoCheck "animus-cargo-preserves-adapter" [ ] ''
            cargo check -p animus-preserves-adapter --locked
          '';

          cargo-preserves-schema = cargoCheck "animus-cargo-preserves-schema" [ ] ''
            cargo check -p animus-preserves-schema --locked
          '';

          preserves-boundary-schemas = cargoCheck "animus-preserves-boundary-schemas" [ ] ''
            sh scripts/cairn/check-preserves-boundary-schemas --root .
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

          harbor-evaluation = cargoCheck "animus-harbor-evaluation" [ pkgs.clippy ] ''
            ${self.packages.${pkgs.stdenv.hostPlatform.system}.check-nickel-configs}/bin/check-nickel-configs
            export ANIMUS_HARBOR_SKIP_NICKEL=1
            sh scripts/cairn/check-harbor-evaluation --root .
          '';

          harbor-atif-adapter = cargoCheck "animus-harbor-atif-adapter" [ pkgs.clippy pkgs.nickel ] ''
            ${self.packages.${pkgs.stdenv.hostPlatform.system}.check-nickel-configs}/bin/check-nickel-configs
            export ANIMUS_HARBOR_ATIF_SKIP_NICKEL=1
            export ANIMUS_HARBOR_ATIF_SKIP_CONNECTED=1
            sh scripts/cairn/check-harbor-atif-adapter --root .
          '';

          basalt-authority-adapter = cargoCheck "animus-basalt-authority-adapter" [ pkgs.clippy pkgs.nickel ] ''
            ${self.packages.${pkgs.stdenv.hostPlatform.system}.check-nickel-configs}/bin/check-nickel-configs
            export ANIMUS_BASALT_SKIP_NICKEL=1
            export ANIMUS_BASALT_SKIP_CONNECTED=1
            sh scripts/cairn/check-basalt-authority-adapter --root .
          '';

          native-evaluation-core = cargoCheck "animus-native-evaluation-core" [ pkgs.b3sum pkgs.clippy pkgs.nickel ] ''
            ${self.packages.${pkgs.stdenv.hostPlatform.system}.check-nickel-configs}/bin/check-nickel-configs
            export ANIMUS_NATIVE_EVAL_SKIP_NICKEL=1
            sh scripts/cairn/check-native-evaluation-core --root .
          '';

          evaluation-adapter-family = cargoCheck "animus-evaluation-adapter-family" [ pkgs.b3sum pkgs.clippy pkgs.nickel ] ''
            sh scripts/cairn/check-evaluation-adapter-family --root .
          '';

          std-evaluation-host = cargoCheck "animus-std-evaluation-host" [ pkgs.b3sum pkgs.clippy pkgs.nickel ] ''
            ${self.packages.${pkgs.stdenv.hostPlatform.system}.check-nickel-configs}/bin/check-nickel-configs
            export ANIMUS_STD_EVAL_SKIP_NICKEL=1
            sh scripts/cairn/check-std-evaluation-host --root .
          '';

          evaluation-composition-conformance = cargoCheck "animus-evaluation-composition-conformance" [ pkgs.b3sum pkgs.clippy pkgs.nickel ] ''
            ${self.packages.${pkgs.stdenv.hostPlatform.system}.check-nickel-configs}/bin/check-nickel-configs
            export ANIMUS_EVAL_CONFORMANCE_SKIP_NICKEL=1
            export ANIMUS_EVAL_CONFORMANCE_SKIP_CONNECTED=1
            sh scripts/cairn/check-evaluation-composition-conformance --root .
            cargo check -p animus --no-default-features --features evaluation-conformance --locked
          '';

          host-control-extraction = cargoCheck "animus-host-control-extraction" [ ] ''
            ${self.packages.${pkgs.stdenv.hostPlatform.system}.check-nickel-configs}/bin/check-nickel-configs
            sh scripts/cairn/check-host-control-extraction --root .
          '';

          spacewasm-compat = cargoCheck "animus-spacewasm-compat" [ pkgs.clippy pkgs.wasm-tools pkgs.b3sum pkgs.lld ] ''
            ${self.packages.${pkgs.stdenv.hostPlatform.system}.check-nickel-configs}/bin/check-nickel-configs
            export ANIMUS_SPACEWASM_SKIP_NICKEL=1
            sh scripts/cairn/check-spacewasm-compat --root .
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

          rlm-kernel = cargoCheck "animus-rlm-kernel" [
            pkgs.b3sum
            pkgs.diffutils
            pkgs.nickel
          ] ''
            sh scripts/cairn/check-rlm-kernel --root .
          '';

          nickel-policy-contracts = cargoCheck "animus-nickel-policy-contracts" [
            pkgs.nickel
            pkgs.b3sum
          ] ''
            sh scripts/cairn/check-nickel-policy-contracts --root .
          '';

          continual-harness = cargoCheck "animus-continual-harness" [
            pkgs.nickel
            pkgs.b3sum
            pkgs.diffutils
          ] ''
            sh scripts/cairn/check-continual-harness --root .
          '';

          daemon-workers = cargoCheck "animus-daemon-workers" [
            pkgs.nickel
            pkgs.diffutils
          ] ''
            sh scripts/cairn/check-daemon-workers --root .
          '';

          messaging-receipts = cargoCheck "animus-messaging-receipts" [
            pkgs.nickel
            pkgs.diffutils
          ] ''
            sh scripts/cairn/check-messaging-receipts --root .
          '';

          persistent-goals = cargoCheck "animus-persistent-goals" [
            pkgs.nickel
            pkgs.diffutils
          ] ''
            sh scripts/cairn/check-persistent-goals --root .
          '';

          schedule-reliability = cargoCheck "animus-schedule-reliability" [
            pkgs.nickel
            pkgs.diffutils
          ] ''
            sh scripts/cairn/check-schedule-reliability --root .
          '';

          session-event-replay = cargoCheck "animus-session-event-replay" [
            pkgs.nickel
            pkgs.diffutils
            pkgs.clippy
          ] ''
            sh scripts/cairn/check-session-event-replay --root .
          '';

          execution-backend-selection = cargoCheck "animus-execution-backend-selection" [
            pkgs.nickel
            pkgs.diffutils
            pkgs.clippy
          ] ''
            sh scripts/cairn/check-execution-backend-selection --root .
          '';

          provider-video-payloads = cargoCheck "animus-provider-video-payloads" [
            pkgs.nickel
            pkgs.diffutils
            pkgs.clippy
          ] ''
            sh scripts/cairn/check-provider-video-payloads --root .
          '';

          terminal-image-protocols = cargoCheck "animus-terminal-image-protocols" [
            pkgs.nickel
            pkgs.diffutils
            pkgs.clippy
          ] ''
            sh scripts/cairn/check-terminal-image-protocols --root .
          '';

          terminal-video-playback = cargoCheck "animus-terminal-video-playback" [
            pkgs.nickel
            pkgs.diffutils
            pkgs.clippy
          ] ''
            sh scripts/cairn/check-terminal-video-playback --root .
          '';

          model-runtime-profiles = cargoCheck "animus-model-runtime-profiles" [
            pkgs.nickel
            pkgs.b3sum
            pkgs.diffutils
          ] ''
            sh scripts/cairn/check-model-runtime-profiles --root .
          '';

          agent-role-profiles = cargoCheck "animus-agent-role-profiles" [
            pkgs.nickel
            pkgs.b3sum
            pkgs.diffutils
          ] ''
            sh scripts/cairn/check-agent-role-profiles --root .
          '';

          aeneas-proof-campaign = cargoCheck "animus-aeneas-proof-campaign" [
            pkgs.nickel
            pkgs.b3sum
            pkgs.diffutils
          ] ''
            sh scripts/cairn/check-aeneas-proof-campaign --root .
          '';

          authoritative-agent-completion = cargoCheck "animus-authoritative-agent-completion" [
            pkgs.nickel
            pkgs.b3sum
            pkgs.diffutils
          ] ''
            export CAIRN_COMPLETION_MODE=saved
            sh scripts/cairn/check-authoritative-agent-completion --root .
          '';

          active-turn-recovery = cargoCheck "animus-active-turn-recovery" [
            pkgs.b3sum
            pkgs.clippy
            pkgs.nickel
          ] ''
            sh scripts/cairn/check-active-turn-recovery --root .
          '';

          conversation-projection = cargoCheck "animus-conversation-projection" [
            pkgs.b3sum
            pkgs.clippy
            pkgs.nickel
          ] ''
            sh scripts/cairn/check-conversation-projection --root .
          '';

          session-branch-explorer = cargoCheck "animus-session-branch-explorer" [
            pkgs.b3sum
            pkgs.clippy
            pkgs.nickel
          ] ''
            sh scripts/cairn/check-session-branch-explorer --root .
          '';

          interrupted-tool-repair = cargoCheck "animus-interrupted-tool-repair" [
            pkgs.b3sum
            pkgs.nickel
          ] ''
            sh scripts/cairn/check-interrupted-tool-repair --root .
          '';

          provider-retry = cargoCheck "animus-provider-retry" [
            pkgs.b3sum
            pkgs.clippy
            pkgs.nickel
          ] ''
            sh scripts/cairn/check-provider-retry --root .
          '';

          session-tree-export = cargoCheck "animus-session-tree-export" [
            pkgs.b3sum
            pkgs.clippy
            pkgs.diffutils
            pkgs.nickel
          ] ''
            sh scripts/cairn/check-session-tree-export --root .
          '';

          bounded-execution-evaluation = cargoCheck "animus-bounded-execution-evaluation" [
            pkgs.b3sum
            pkgs.nickel
          ] ''
            sh scripts/cairn/check-bounded-execution-evaluation --root .
          '';

          application-host-binding = cargoCheck "animus-application-host-binding" [
            pkgs.b3sum
            pkgs.clippy
            pkgs.diffutils
            pkgs.nickel
          ] ''
            sh scripts/cairn/check-application-host-binding --root .
          '';

          sandlock-host = cargoCheck "animus-sandlock-host" [
            pkgs.b3sum
            pkgs.clippy
          ] ''
            unset ANIMUS_SANDLOCK_LINUX_LIVE
            sh scripts/cairn/check-sandlock-host --root .
          '';

          agent-guardrail = cargoCheck "animus-agent-guardrail" [
            pkgs.b3sum
            pkgs.bubblewrap
            pkgs.clippy
            pkgs.nickel
          ] ''
            export ANIMUS_GUARDRAIL_LINUX_LIVE=1
            export ANIMUS_GUARDRAIL_BWRAP=${pkgs.bubblewrap}/bin/bwrap
            sh scripts/cairn/check-agent-guardrail --root .
          '';

          durable-session-store = cargoCheck "animus-durable-session-store" [
            pkgs.b3sum
            pkgs.clippy
            pkgs.diffutils
            pkgs.nickel
          ] ''
            sh scripts/cairn/check-durable-session-store --root .
          '';

          coding-session-host = cargoCheck "animus-coding-session-host" [
            pkgs.b3sum
          ] ''
            sh scripts/cairn/check-coding-session-host --root .
          '';

          agent-control = cargoCheck "animus-agent-control" [
            pkgs.b3sum
          ] ''
            sh scripts/cairn/check-agent-control --root .
          '';

          acp-stdio-host = cargoCheck "animus-acp-stdio-host" [
            pkgs.b3sum
            pkgs.clippy
          ] ''
            sh scripts/cairn/check-acp-stdio-host --root .
          '';

          acp-nominal-identities = cargoCheck "animus-acp-nominal-identities" [
            pkgs.b3sum
            pkgs.clippy
            pkgs.nickel
            pkgs.rustfmt
          ] ''
            sh scripts/cairn/check-acp-nominal-identities --root .
          '';

          lean4-loop-fabric = cargoCheck "animus-lean4-loop-fabric" [
            pkgs.b3sum
            pkgs.clippy
            pkgs.gcc
            pkgs.lean4
            pkgs.nickel
          ] ''
            sh scripts/cairn/check-lean4-loop-fabric --root .
          '';

          session-authority-profile = cargoCheck "animus-session-authority-profile" [
            pkgs.b3sum
            pkgs.diffutils
            pkgs.nickel
          ] ''
            sh scripts/cairn/check-session-authority-profile --root .
          '';

          runtime-authority-seal = cargoCheck "animus-runtime-authority-seal" [
            pkgs.b3sum
            pkgs.clippy
            pkgs.rustfmt
          ] ''
            sh scripts/cairn/check-runtime-authority-seal --root .
          '';

          rejected-attempt-context = cargoCheck "animus-rejected-attempt-context" [
            pkgs.b3sum
            pkgs.clippy
            pkgs.nickel
            pkgs.rustfmt
          ] ''
            sh scripts/cairn/check-rejected-attempt-context --root .
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

          codebase-graph = cargoCheck "animus-codebase-graph" [ pkgs.nickel pkgs.b3sum ] ''
            sh scripts/cairn/check-codebase-graph --root .
          '';

          mesh-provider = cargoCheck "animus-mesh-provider" [ pkgs.clippy pkgs.nickel ] ''
            ${self.packages.${pkgs.stdenv.hostPlatform.system}.check-nickel-configs}/bin/check-nickel-configs
            export ANIMUS_MESH_SKIP_NICKEL=1
            sh scripts/cairn/check-mesh-provider --root .
          '';

          branchfs-host = cargoCheck "animus-branchfs-host" [ pkgs.clippy ] ''
            sh scripts/cairn/check-branchfs-host
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
            lean4
            lld
            wasm-tools
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
