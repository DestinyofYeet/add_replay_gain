{ 
  rustPlatform,
  lib,
  fetchFromGitHub,
  pkgs,
}: 

rustPlatform.buildRustPackage rec {
  pname = "add_replay_gain_to_files";
  version = "1.5";

  src = fetchFromGitHub {
    owner = "DestinyofYeet";
    repo = "add_replay_gain";
    rev = "548e79a0277085993a594f441457a849eed91cf8";
    hash = "sha256-n8MGTUJMm2pjLf6nURt77cznHNYLpKtHI1j9x65wK/o=";
  };

  cargoHash = "sha256-WyG0/qAhMf6BGm2ykKF954RvB9hkWXnT4DNqICLSKzs=";

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
