import { List, ListItemButton, ListItemText } from "@mui/material";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Webview } from "@tauri-apps/api/webview";
import React, { useEffect, useState } from "react";

interface HistoryItem {
  id: number;
  value: string;
}

const App = () => {
  const [history, setHistory] = useState<HistoryItem[]>([]);
  const [selected, setSelected] = useState(0);

  const getHistory = async (): Promise<HistoryItem[]> => {
    return await invoke("history");
  };

  useEffect(() => {
    const unsub = listen("history-updated", () => {
      getHistory()
        .then((items) => setHistory(items))
        .catch(console.error);
    });

    return () => {
      unsub.then(() => console.log("unsubscribed from `history-update`"));
    };
  }, []);

  const onKeyDown = async (e: React.KeyboardEvent) => {
    switch (e.key) {
      case "Escape":
        await Webview.getCurrent().window.hide();
        break;
      case "j":
      case "ArrowDown":
        setSelected(Math.min(selected + 1, history.length - 1));
        break;
      case "k":
      case "ArrowUp":
        setSelected(Math.max(selected - 1, 0));
        break;
    }
  };

  const paste = async (id: number) => {
    await Webview.getCurrent().window.hide();
    try {
      await invoke("paste", { id });
    } catch (e) {
      console.debug(e);
    }
  };

  return (
    <List onKeyDown={onKeyDown} dense>
      {history.map(({ id, value }, i) => {
        const itemSelected = selected === i;
        return (
          <ListItemButton
            key={id}
            selected={itemSelected}
            autoFocus={itemSelected}
            onClick={() => paste(id)}
            divider
          >
            <ListItemText
              primary={value}
              primaryTypographyProps={{ noWrap: true }}
            />
          </ListItemButton>
        );
      })}
    </List>
  );
};

export default App;
