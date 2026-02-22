{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    # rebuilding packages with cache
    naersk.url = "github:nix-community/naersk";
    # ligthweight tool for different rust-toolchain versions
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # heavy but great choise of versions of rust-toolchain
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };

  outputs =
    {
      self,
      nixpkgs,
      naersk,
      fenix,
      rust-overlay,
    }:
    let
      # pkgs = nixpkgs.legacyPackages."x86_64-linux";
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        overlays = [ (import rust-overlay) ];
      };
      naerskLib = pkgs.callPackage naersk { };
      fenixLib = fenix.packages."x86_64-linux";
      rustToolchain = fenixLib.stable.toolchain;
    in
    {
      devShells."x86_64-linux".default = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo # Downloads your Rust project's dependencies and builds your project
          rustc # Safe, concurrent, practical language (wrapper script)
          rustfmt # Tool for formatting Rust code according to style guidelines
          clippy # Bunch of lints to catch common mistakes and improve your Rust code
          rust-analyzer # Modular compiler frontend for the Rust language

          # other dependencies
          glib # C library of programming buildings blocks
        ];

        nativeBuildInputs = with pkgs; [ pkg-config ];

        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

        # ensure cargo-installed binaries (like ~/.cargo/bin/rustlings) are on PATH inside nix develop
        shellHook = ''
          export PATH="$HOME/.cargo/bin:$PATH"
        '';
      };

      # packaging program at top-level outputs (not inside mkShell)
      # packages."x86_64-linux".default = pkgs.callPackage ./default.nix { };

      # packages."x86_64-linux".default = naerskLib.buildPackage {
      #   src = ./.;
      #   buildInputs = [ pkgs.glib ];
      #   nativeBuildInputs = [ pkgs.pkg-config ];
      # };
      packages.x86_64-linux.default =
        pkgs.makeRustPlatform
          {
            cargo = rustToolchain;
            rustc = rustToolchain;
          }
          .buildRustPackage
          {

          };
    };
}
