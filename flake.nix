{
  description = "Web API for the Corn configuration language";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, naersk, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naerskLib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naerskLib.buildPackage ./.;

        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt rustPackages.clippy ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };

        nixConfig = {
          extra-substituters = [ "https://cache.garnix.io" ];
          extra-trusted-public-keys =
            [ "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g=" ];
        };
      }
    );
}
