export function to_snake_case(str: string) {
  return str
    .replace(/([A-Z])/g, (match) => `_${match.toLowerCase()}`)
    .replace(/^_/, '');
}
