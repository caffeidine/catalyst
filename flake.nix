{
  description = "Flake for the catalyst Rust CLI";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [
        "x86_64-linux"

        # TODO: Should work for other arch, but haven't tried.
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    in {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "catalyst";
            version = "0.2.9";
            src = pkgs.lib.cleanSource ./.;
            cargoLock.lockFile = ./Cargo.lock;

            # Nix builds are sandboxed, tests requiring internet do not work
            # TODO: use wiremock or httpmock instead
            cargoTestFlags = [
              "--"
              "--skip" "tests::test_get_request"
              "--skip" "tests::test_post_request"
              "--skip" "tests::test_with_params"
            ];

            meta = with pkgs.lib; {
              description = "A lightweight API testing tool";
              homepage = "https://github.com/caffeidine/catalyst";
              license = licenses.mpl20;
            };
            mainProgram = "catalyst";
          };
        }
      );

      apps = forAllSystems (system:
        let
          pkg = self.packages.${system}.default;
        in {
          default = {
            type = "app";
            program = "${pkg}/bin/catalyst";
          };
        }
      );

      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in {
          default = pkgs.mkShell {
            buildInputs = [
              pkgs.rustc
              pkgs.cargo
              pkgs.clippy
            ];
          };
        }
      );
    };
}
