{
  description = "Custom Bevy Halo game and development profiles";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
      pkgsFor = system: import nixpkgs { inherit system; };
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      version = cargoToml.workspace.package.version;
      rustPlatformFor = pkgs: pkgs.rustPlatform;

      commonNativeBuildInputs = pkgs: [
        pkgs.pkg-config
        pkgs.rustc
        pkgs.cargo
        pkgs.clippy
        pkgs.rustfmt
      ];

      linuxLibraries = pkgs: [
        pkgs.wayland
        pkgs.libxkbcommon
        pkgs.vulkan-loader
      ];

      platformBuildInputs = pkgs: if pkgs.stdenv.hostPlatform.isLinux then linuxLibraries pkgs else [ ];

      platformShellHook =
        pkgs:
        if pkgs.stdenv.hostPlatform.isLinux then
          ''
            export WINIT_UNIX_BACKEND=wayland
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath (linuxLibraries pkgs)}:/run/opengl-driver/lib''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
            if [ -d /run/opengl-driver/share/vulkan/icd.d ]; then
              export VK_DRIVER_FILES="$(find /run/opengl-driver/share/vulkan/icd.d -name '*_icd.x86_64.json' -type f -print | paste -sd: -)"
            fi
          ''
        else
          ''
            export WGPU_BACKEND=metal
          '';
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = pkgsFor system;
          rustPlatform = rustPlatformFor pkgs;
          haloGame = rustPlatform.buildRustPackage {
            pname = "halo-game";
            inherit version;
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = platformBuildInputs pkgs;
            cargoBuildFlags = [
              "-p"
              "halo-game"
            ];
            cargoTestFlags = [
              "-p"
              "halo-game"
            ];
            doCheck = true;
            meta = {
              description = "Custom Halo-inspired game built with Bevy";
              license = with pkgs.lib.licenses; [
                mit
                asl20
              ];
              mainProgram = "halo-game";
              platforms = [
                "x86_64-linux"
                "aarch64-darwin"
              ];
            };
          };
        in
        {
          default = haloGame;
          halo-game = haloGame;
        }
      );

      apps = forAllSystems (system: {
        default = {
          type = "app";
          program = "${self.packages.${system}.halo-game}/bin/halo-game";
        };
        halo-game = {
          type = "app";
          program = "${self.packages.${system}.halo-game}/bin/halo-game";
        };
      });

      devShells = forAllSystems (
        system:
        let
          pkgs = pkgsFor system;
        in
        {
          default = pkgs.mkShell {
            packages = commonNativeBuildInputs pkgs;
            buildInputs = platformBuildInputs pkgs;
            shellHook = platformShellHook pkgs;
          };
        }
      );

      checks = forAllSystems (system: {
        package = self.packages.${system}.halo-game;
      });

      formatter = forAllSystems (system: (pkgsFor system).nixfmt);
    };
}
