{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixpkgs-unstable;
    rust-overlay = {
      url = github:oxalica/rust-overlay;
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "aarch64-darwin";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rust-bin.stable.latest.default
          rust-analyzer
          cargo
          cargo-watch
          cargo-edit
          cargo-audit
          cargo-expand
          cargo-flamegraph
          clippy
          rustfmt
          just
          openssl
          pkg-config
          darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        shellHook = ''
            echo "🦀 Rust development environment loaded!"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo new <project-name>  - Create a new Rust project"
            echo "  cargo watch -x run        - Run with auto-reload"
            echo "  cargo expand              - Expand macros"
            echo "  cargo flamegraph         - Generate flamegraph"
            echo ""
        '';
      };
    };
}
