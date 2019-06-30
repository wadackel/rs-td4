import { createContext } from "react";

export type CpuContextValues = {
  rom: number[];
  register: number[];
  port: number[];
};

export const CpuContext = createContext<CpuContextValues>({
  rom: [],
  register: [],
  port: []
});
