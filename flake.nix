{
  description = "Dev environment for Rust";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };in {
      formatter.${system} = pkgs.nixpkgs-format;
      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = [ pkgs.rustup ];
      };
    };
}
