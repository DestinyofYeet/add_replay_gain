self: { lib, config, pkgs, ... }:

with lib;

let
  cfg = config.services.addReplayGain;
in {
  options = {
    services.addReplayGain = {
      enable = mkEnableOption "add replay gain to files";

      watchDirectory = mkOption {
        type = types.str;
        description = "The directory to watch for new files.";
      };

      metaFlacFlags = mkOption {
        type = types.str;
        default = "--add-replay-gain";
        description = "Flags to add to metaFlac";
      };

      mp3GainFlags = mkOption {
        type = types.str;
        default = "-a -k";
        description = "Flags to add to mp3Gain";
      };

      features = {
        add-replay-gain = mkOption {
          type = types.enum [ "true" "false" ];
          default = "true";
          description = "Wether to actually add replay gain to files";
        };
        remove-comments = mkOption {
          type = types.enum [ "true" "false" ];
          default = "true";
          description = "Wether to remove comments from files";
        };
      };

      user = mkOption {
        type = types.str;
        default = "add-replay-gain";
        description = "User to run as";
      };

      group = mkOption {
        type = types.str;
        default = "add-replay-gain";
        description = "Group to run as";
      };

      uptimeUrl = mkOption {
        type = types.str;
        default = null;
        description = "The url to GET every 60 seconds";
      };

      package = mkOption {
        type = types.package;
        default = self.packages.x86_64-linux.add-replay-gain;
        description = "The package to use";
      };

      extraSettings = mkOption {
        type = types.lines;
        default = "";
        description = "Additional settings to include in the config.toml.";
      };
    }; 
  };

  config = mkIf cfg.enable {
    systemd.services.add-replay-gain = let 
      config-file = pkgs.writeText "config.toml" ''
        [DEFAULT]
        watch_path = ${cfg.watchDirectory}

        [ENABLE]
        replay_gain = ${cfg.features.add-replay-gain}
        remove_comment = ${cfg.features.remove-comments}

        [FLAC]
        metaflac_bin = ${pkgs.flac}/bin/metaflac
        metaflac_flags = ${cfg.metaFlacFlags}

        [MP3]
        mp3gain_bin = ${pkgs.mp3gain}/bin/mp3gain
        mp3gain_flags = ${cfg.mp3GainFlags}

        [UPTIME]
        uptime_url = ${cfg.uptimeUrl}
        '';

    in {
      description = "Add Replay Gain to audio files";
      wantedBy = [ "multi-user.target" ];
      serviceConfig = {
        ExecStart = "${cfg.package}/bin/add_replay_gain_to_files --config ${config-file}";
        Restart = "on-failure";
        User = cfg.user;
        Group = cfg.group;
      };
    };
  };
}
