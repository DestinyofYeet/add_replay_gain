{ 
  rustPlatform,
  lib,
  fetchFromGitHub,
  pkgs,
}: 

rustPlatform.buildRustPackage rec {
  pname = "add_replay_gain_to_files";
  version = "1.6";

  src = fetchFromGitHub {
    owner = "DestinyofYeet";
    repo = "add_replay_gain";
    rev = "7c47e72cdef542b1f162c6fa926be98c2abf67b3";
    hash = "sha256-D3LsUZpu+MRlCyaNIIIsosrMmxQRysqX4oi5aAVDvPU=";
  };

  cargoHash = "sha256-oBoaUVjz2DpNvQzoADW9SoQ1vmYFqiAqbn2T5LzzPPs=";

  nativeBuildInputs = with pkgs; [
    pkg-config
    openssl.dev
  ];

  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

  meta = with lib; {
    description = "A tool to automatically add replay gain to mp3 and flac files";
    license = licenses.gpl3;
    # maintainers = [ maintainers.DestinyofYeet ];
    # platforms = platforms.all;
  };
}
