import { Listbox } from '@headlessui/react'
import clsx from 'clsx'
import { createContext, useContext } from 'react'

import { useDialect } from '@/components/DialectContext'
import { Icon } from '@/components/icons'
import { Tag } from '@markdoc/markdoc'

const LocalDialectContext = createContext({
  localDialect: '',
  setLocalDialect: () => {},
})

export function DialectSwitcher({ children, title, dialects }) {
  const { dialect, setDialect } = useDialect()
  const hasMatchingDialect = dialects.some(({ id }) => id === dialect)
  const localDialect =
    !hasMatchingDialect && dialects.length > 0 ? dialects[0].id : dialect
  const localDialectTitle =
    dialects.find(({ id }) => id === localDialect)?.title ?? localDialect

  return (
    <LocalDialectContext.Provider
      value={{ localDialect, setLocalDialect: setDialect }}
    >
      <div className="totem">
        <div className="not-prose flex flex-wrap items-center gap-4 px-4 py-3">
          {title && <p className="text-slate-400">{title}</p>}
          <Listbox
            value={localDialect}
            onChange={setDialect}
            as="div"
            className="relative ml-auto"
          >
            <Listbox.Button className="flex items-center gap-1 rounded-[0.625rem] bg-slate-800/60 px-2 py-0.5 text-sm text-slate-400 dark:bg-slate-900/40">
              <span>{localDialectTitle}</span>
              <Icon icon="SolidChevronUpDown" className="h-4 w-4" />
            </Listbox.Button>
            <Listbox.Options className="absolute right-0 top-full mt-2 w-max space-y-1 rounded-xl bg-white p-3 text-sm font-medium shadow-md shadow-black/5 ring-1 ring-black/5 dark:bg-slate-800 dark:ring-white/5">
              {dialects.map((dialect) => (
                <Listbox.Option
                  key={dialect.id}
                  value={dialect.id}
                  className={({ active, selected }) =>
                    clsx(
                      'flex cursor-pointer select-none items-center rounded-[0.625rem] px-2 py-1',
                      {
                        'text-accent-500': selected,
                        'text-slate-900 dark:text-white': active && !selected,
                        'text-slate-700 dark:text-slate-400':
                          !active && !selected,
                        'bg-slate-100 dark:bg-slate-900/40': active,
                      }
                    )
                  }
                >
                  {dialect.title}
                </Listbox.Option>
              ))}
            </Listbox.Options>
          </Listbox>
        </div>
        <div className="overflow-hidden rounded-b-xl">{children}</div>
      </div>
    </LocalDialectContext.Provider>
  )
}

export function Dialect({ children, id }) {
  const { localDialect } = useContext(LocalDialectContext)
  if (localDialect !== id) return null
  return <>{children}</>
}

export function transformDialectSwitcherTag(node, config) {
  const attributes = node.transformAttributes(config)
  const children = node
    .transformChildren(config)
    .filter((child) => child.name === 'Dialect')
  const dialects = children.map((dialect) => dialect.attributes)

  return new Tag(this.render, { ...attributes, dialects }, children)
}
