import { invoke } from "@tauri-apps/api/core";

export enum Method {
  GET = "get",
  POST = "post",
}

export const invokeLcu = async <T>(method: Method, uri: string, body: string = ""): Promise<T | null> => {
  return await invoke<T | null>("invoke_lcu", { method, uri, body });
};
