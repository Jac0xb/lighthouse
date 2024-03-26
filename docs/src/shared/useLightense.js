import Lightense from 'lightense-images'
import { useEffect } from "react"

export function useLightense() {
  useEffect(() => Lightense('img:not(.no-lightense),.lightense'))
}
