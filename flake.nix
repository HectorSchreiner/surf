{
  description = "Dev shell with Postman, npm, Docker, and automated startup tasks";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";

  outputs = { self, nixpkgs }: {
    devShells.default = let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in pkgs.mkShell {
      name = "dev-env";

      buildInputs = [
        pkgs.postman
        pkgs.nodejs
        pkgs.docker
        pkgs.cargo-watch
        pkgs.rustc
        pkgs.cargo
      ];

      shellHook = ''
        echo "Starting Docker"
        sudo docker compose -f compose.dev.yaml up -d

        echo "Starting cargo watch in backend/"
        cd backend
        exec cargo watch -x 'run --features=docs'
      '';
    };
  };
}
