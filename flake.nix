{
  description = "RUST FLAKE RUST FLAKE";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
  let
    system = "x86_64-linux";
    libPath = with pkgs; lib.makeLibraryPath [
      libGL
      libxkbcommon
      wayland
    ];
    pkgs = nixpkgs.legacyPackages.${system};
  in 
  with pkgs;
  {
    devShells.${system}.default = mkShell rec {
      buildInputs = [
        cargo
        rustc
        rust-analyzer
        libxkbcommon
        wayland xorg.libX11 xorg.libXcursor xorg.libXrandr xorg.libXi
        alsa-lib
        shaderc directx-shader-compiler

        libGL
        vulkan-headers vulkan-loader
        vulkan-tools vulkan-tools-lunarg
        vulkan-extension-layer
        vulkan-validation-layers
      ];

      LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
    };
  };
}
