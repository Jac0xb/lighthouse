import { Disclosure } from '@headlessui/react'
import clsx from 'clsx'

import { Icon } from '@/components/icons'

export function Totem({ children }) {
  return <div className="totem overflow-hidden">{children}</div>
}

export function TotemAccordion({ children, title }) {
  return (
    <Disclosure>
      {({ open }) => (
        <div>
          <Disclosure.Button className="flex w-full items-center justify-between px-4 py-3 hover:bg-slate-800">
            {title}
            <Icon
              icon="ChevronRight"
              className={clsx(
                'h-5 w-5 transition',
                open ? 'rotate-90 transform' : ''
              )}
            />
          </Disclosure.Button>
          <Disclosure.Panel>{children}</Disclosure.Panel>
        </div>
      )}
    </Disclosure>
  )
}

export function TotemProse({ children }) {
  return <div>{children}</div>
}
