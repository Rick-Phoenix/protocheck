import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]): string {
  return twMerge(clsx(inputs));
}

type ClassDictionary = Record<
  string,
  ClassItem
>;

type ClassItem = string | string[] | Boolean | { [key: string]: ClassItem };

function recursiveGetClass(dict: ClassDictionary): string[] {
  const classes: string[] = [];

  const pairs = Object.entries(dict);
  for (const [key, val] of pairs) {
    if (val === true) {
      classes.push(key);
    } else if (typeof val === "string") {
      classes.push(val);
    } else if (Array.isArray(val)) {
      for (const item of val) {
        classes.push(item);
      }
    } else if (typeof val === "object" && val) {
      classes.push(...recursiveGetClass(val as { [key: string]: ClassItem }));
    }
  }

  return classes;
}

export function cd(dict: ClassDictionary, ...flatOpts: string[]): string[] {
  const output = recursiveGetClass(dict);

  if (flatOpts) {
    output.push(...flatOpts);
  }

  return output;
}

export function fcd(dict: ClassDictionary, ...flatOpts: ClassValue[]): string {
  return twMerge(cd(dict), clsx(flatOpts));
}

export function fcc(callback: () => ClassDictionary) {
  return fcd(callback());
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, "child"> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any }
  ? Omit<T, "children">
  : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & {
  ref?: U | null;
};
