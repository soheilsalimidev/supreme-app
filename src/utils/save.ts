import { useAppSettingStore } from "@/stores/appSetting";
import { invoke } from "@tauri-apps/api/core";
import { save, open } from "@tauri-apps/plugin-dialog";
import { notify } from "notiwind";

export const saveAppAsFile = async () => {
  const appDate = useAppSettingStore();
  let savePath = appDate.savePath;
  try {
    if (!savePath) {
      const selected = await save({
        filters: [
          {
            name: "iapp",
            extensions: ["iapp"],
          },
        ],
      });
      if (!selected) {
        return;
      }
      appDate.savePath = selected;
      savePath = selected + ".iapp";
    }
    await invoke("save_config", {
      config: appDate.appInfo,
      path: savePath,
    });
    notify(
      {
        group: "generic",
        title: "settings save",
        text: "your settings saved in " + savePath,
        type: "info",
      },
      3000,
    );
  } catch (error) {
    notify(
      {
        group: "generic",
        title: "settings saving failed",
        text: "error" + error,
        type: "warning",
      },
      3000,
    );
  }
};

export const importFromTheAppFile = async () => {
  const appDate = useAppSettingStore();
  let savePath = appDate.savePath;
  try {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "IApp",
          extensions: ["iapp"],
        },
      ],
    });
    if (selected === null) {
      throw "you didnt select";
    }
    const config = await invoke<[string, string]>("import_config", {
      path: selected,
    });
    appDate.appInfo = JSON.parse(config[0]);
    appDate.appInfo.paths.forEach((val, index) => {
      appDate.appInfo.paths[index].path = config[1] + "/" + val.path;
    });
    notify(
      {
        group: "generic",
        title: "settings save",
        text: "your settings saved in " + savePath,
        type: "info",
      },
      3000,
    );
  } catch (error) {
    console.log(error);
    notify(
      {
        group: "generic",
        title: "settings saving failed",
        text: "error" + error,
        type: "warning",
      },
      3000,
    );
  }
};
