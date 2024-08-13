import { FormControlLabel, Stack, Switch } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import { Config } from "../model/Config";

const Prefs = () => {
  const [config, setConfig] = useState<Config | null>(null);

  useEffect(() => {
    const unsubscribe = listen("config-updated", () => {
      invoke<Config>("get_config", {})
        .then((c) => {
          setConfig(c);
        })
        .catch(console.error);
    });

    return () => {
      unsubscribe
        .then(() => console.log("unsubscribed from config"))
        .catch(console.error);
    };
  }, []);

  const saveConfig = (config: Config) => {
    invoke("set_config", { config }).catch(console.error);
  };

  const handleAutostartChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    saveConfig({ ...config!!, autostart: e.target.checked });
  };

  if (!config) {
    return <div>Loading...</div>;
  }

  return (
    <Stack gap={1} alignItems="center">
      <FormControlLabel
        control={
          <Switch checked={config.autostart} onChange={handleAutostartChange} />
        }
        label={"Launch at system startup"}
      />
    </Stack>
  );
};

export default Prefs;
