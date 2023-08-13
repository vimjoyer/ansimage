{
  description = "ansimage flake";

  inputs =
    {
      nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};

      buildInputs = with pkgs; [
        rustc
        rustfmt
        rust-analyzer
        clippy
        cargo
        cargo-edit
        cargo-watch
        clang
      ];

    in
    {
      devShells.${system}.default =
        pkgs.mkShell
          {
            buildInputs = buildInputs;
          };

      packages.${system}.default =
        pkgs.rustPlatform.buildRustPackage {
          name = "ansimg";
          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
        };
    };
}
