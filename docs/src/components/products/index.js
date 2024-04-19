import { lighthaus } from './lighthaus'

export const products = [lighthaus].sort((a, b) =>
  a.name.localeCompare(b.name)
)
