import { createContext, useContext, useEffect, useState } from 'react'

export const DialectContext = createContext({
  dialect: '',
  setDialect: () => {},
})

export const useDialect = () => useContext(DialectContext)

export function DialectProvider({ children }) {
  const [dialect, setDialect] = useState('')
  useEffect(() => {
    setDialect(localStorage.getItem('dialect'))
  }, [])
  useEffect(() => {
    dialect && localStorage.setItem('dialect', dialect)
  }, [dialect])

  return (
    <DialectContext.Provider value={{ dialect, setDialect }}>
      {children}
    </DialectContext.Provider>
  )
}
