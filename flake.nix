{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
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
            [
              pkgs.darwin.apple_sdk.frameworks.Security
              pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
            ]
          else
            [ ];

        nameAndVersion = craneLib.crateNameFromCargoToml { cargoToml = ./weather-server/Cargo.toml; };

        staticFilter = path: _type: (builtins.match "^.*/weather-server/src/static/.*\\.(html|js|css)$" path) != null;
        filter = path: type: (craneLib.filterCargoSources path type) || (staticFilter path type);

        src = pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          inherit filter;
        };

        commonArgs = nameAndVersion // {
          inherit src;
          cargoExtraArgs = "--package weather-server";
          nativeBuildInputs = systemBuildInputs;

          RUST_BACKTRACE = 1;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in
      rec
      {
        packages.default = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });

        packages.container = pkgs.dockerTools.streamLayeredImage {
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
          buildInputs = with pkgs; [ nodejs yarn cargo-insta cargo-outdated ];
        };
      }
    );
}
