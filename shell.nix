{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ ];
  buildInputs = with pkgs; [ ];
  inputsFrom = with pkgs; [ ];
  hardeningDisable = [ "all" ];
  shellHook = ''

      export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.wayland}/lib:${pkgs.libxkbcommon}/lib:${pkgs.vulkan-loader}/lib
  '';
}
