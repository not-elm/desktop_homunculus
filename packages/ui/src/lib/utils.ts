import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export type SomeRequired<T, K extends keyof T> = Omit<T, K> & Required<Pick<T, K>>;