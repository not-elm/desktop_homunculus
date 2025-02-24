export const isProduction = import.meta.env.PROD

export const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))