import { Grid } from '@/components/products/Grid'
import { Popover, Transition } from '@headlessui/react'

export function SwitcherPopover({ children, menuItem, ...props }) {
  console.log('menuItem', menuItem)
  return (
    <Popover {...props}>
      {children}
      {/* <Transition
        enter="transition ease-out duration-200"
        enterFrom="opacity-0 translate-y-1"
        enterTo="opacity-100 translate-y-0"
        leave="transition ease-in duration-150"
        leaveFrom="opacity-100 translate-y-0"
        leaveTo="opacity-0 translate-y-1"
      > */}
      <Popover.Panel className="absolute z-10 m-auto mt-4 ">
        {({ close }) => (
          <div className="fixed left-0 w-full">
            <div className="m-auto w-full max-w-[600px]  overflow-hidden  rounded-lg bg-white p-4 shadow-xl ring-1 ring-black ring-opacity-5 dark:border dark:border-slate-600 dark:bg-neutral-800">
              <Grid
                className="relative md:grid-flow-row md:grid-cols-2"
                onClick={close}
                menuItem={menuItem}
              />
            </div>
          </div>
        )}
      </Popover.Panel>
      {/* </Transition> */}
    </Popover>
  )
}
