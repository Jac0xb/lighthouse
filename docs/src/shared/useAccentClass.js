import { useEffect } from "react"

export function useAccentClass(product) {
  useEffect(() => {
    document.querySelector('body').classList.forEach((className) => {
      if (className.startsWith('accent-')) {
        document.querySelector('body').classList.remove(className)
      }
    })
    if (product?.className) {
      document.querySelector('body').classList.add(product?.className)
    }
  })
}
