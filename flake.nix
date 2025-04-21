{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        lib = pkgs.lib;
        pname = "web-utils";
      in
      {
        packages.${pname} = pkgs.rustPlatform.buildRustPackage rec {
          inherit pname;
          version = "1.0.1";

          src = pkgs.lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
        packages.default = self.packages.${system}.${pname};
      }
    );
}
