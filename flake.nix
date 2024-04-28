{
  description = "Web API for the Corn configuration language";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, naersk, utils }:
    utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs { inherit system; };
          naerskLib = pkgs.callPackage naersk { };
        in
        {
          packages = rec {
            corn-api = naerskLib.buildPackage ./.;
            default = corn-api;
          };
        }) // {
      nixosModules.default = { config, pkgs, ... }:
        let
          cfg = config.services.corn-api;
          lib = pkgs.lib;
          defaultPkg = self.packages.${pkgs.hostPlatform.system}.default;
        in
        {
          options.services.corn-api = {
            enable = lib.mkEnableOption "Web API for Corn configuration language";

            package = lib.mkOption {
              type = lib.types.package;
              default = defaultPkg;
              description = "The package to use";
            };

            host = lib.mkOption {
              type = lib.types.str;
              description = "The hostname to listen on";
              default = "127.0.0.1";
            };

            port = lib.mkOption {
              type = lib.types.port;
              description = "The port number to listen on";
              default = 5050;
            };
          };

          config = let pkg = cfg.package; in {
            systemd.services.corn-api = lib.mkIf cfg.enable {
              description = "Web API for Corn configuration language";
              documentation = [ "https://github.com/corn-config/corn-api" ];

              serviceConfig = {
                ExecStart = "${pkg}/bin/corn-api";
                Restart = "on-failure";
              };

              environment.HOST = cfg.host;
              environment.PORT = toString cfg.port;

              wantedBy = [ "multi-user.target" ];
            };
          };
        };

      devShell = with nixpkgs; mkShell {
        buildInputs = [ cargo rustc rustfmt rustPackages.clippy ];
        RUST_SRC_PATH = rustPlatform.rustLibSrc;
      };

      nixConfig = {
        extra-substituters = [ "https://cache.garnix.io" ];
        extra-trusted-public-keys =
          [ "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g=" ];
      };
    };
}
