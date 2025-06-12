{
  description = "Dev shell with Postman, npm, Docker";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";

  outputs = { self, nixpkgs }: let
    systems = [ "x86_64-linux" "aarch64-linux" ];
    forAllSystems = f:
      builtins.listToAttrs (map (system: { name = system; value = f system; }) systems);
  in {
    devShells = forAllSystems (system: let
      pkgs = import nixpkgs { 
        inherit system; 
        config.allowUnfree = true;
        };
    in {
      default = pkgs.mkShell {
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
          echo "Composing docker for your lazy ass"
          sudo docker compose -f compose.dev.yaml up -d
        '';
      };
    });
  };
}
