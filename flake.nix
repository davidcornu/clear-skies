{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-23.05";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
      inputs.rust-overlay.follows = "rust-overlay";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs =
    { self
    , flake-utils
    , crane
    , nixpkgs
    , rust-overlay
    }: flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;

          overlays = [
            (import rust-overlay)
          ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        systemBuildInputs =
          if pkgs.stdenv.isDarwin
          then
            [ pkgs.darwin.apple_sdk.frameworks.Security ]
          else
            [ ];

        nameAndVersion = craneLib.crateNameFromCargoToml { cargoToml = ./weather-server/Cargo.toml; };
      in
      rec
      {
        packages.default = craneLib.buildPackage (nameAndVersion // {
          src = craneLib.cleanCargoSource (craneLib.path ./.);

          nativeBuildInputs = systemBuildInputs;
          cargoExtraArgs = "--package weather-server";
          doCheck = false;

          RUST_BACKTRACE = 1;
        });

        packages.container = pkgs.dockerTools.buildLayeredImage {
          name = "weather-server";
          tag = "latest";
          contents = [ pkgs.cacert packages.default ];
          config.Cmd = [
            "${packages.default}/bin/weather-server"
            "--bind-addr"
            "0.0.0.0:8080"
          ];
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [ rustToolchain ] ++ systemBuildInputs;
          buildInputs = with pkgs; [ ];
        };
      }
    );
}
