import { lighthouse } from './lighthouse'

export const products = [lighthouse].sort((a, b) =>
  a.name.localeCompare(b.name)
)
