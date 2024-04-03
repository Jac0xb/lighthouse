import { useEffect, useState } from 'react'
import Link from 'next/link'
import { useRouter } from 'next/router'
import { Dialog } from '@headlessui/react'

import { LogoWithName } from '@/components/products/Logo'
import { Navigation } from '@/components/Navigation'
import { Sections } from '@/components/products/Sections'
import { ComputerDesktopIcon } from '@heroicons/react/24/outline'

function MenuIcon(props) {
  return (
    <svg
      aria-hidden="true"
      viewBox="0 0 24 24"
      fill="none"
      strokeWidth="2"
      strokeLinecap="round"
      {...props}
    >
      <path d="M4 7h16M4 12h16M4 17h16" />
    </svg>
  )
}

function CloseIcon(props) {
  return (
    <svg
      aria-hidden="true"
      viewBox="0 0 24 24"
      fill="none"
      strokeWidth="2"
      strokeLinecap="round"
      {...props}
    >
      <path d="M5 5l14 14M19 5l-14 14" />
    </svg>
  )
}

export function MobileNavigation({ page }) {
  let router = useRouter()
  let [isOpen, setIsOpen] = useState(false)

  useEffect(() => {
    if (!isOpen) return

    function onRouteChange() {
      setIsOpen(false)
    }

    router.events.on('routeChangeComplete', onRouteChange)
    router.events.on('routeChangeError', onRouteChange)

    return () => {
      router.events.off('routeChangeComplete', onRouteChange)
      router.events.off('routeChangeError', onRouteChange)
    }
  }, [router, isOpen])

  return (
    <>
      <button
        type="button"
        onClick={() => setIsOpen(true)}
        className="relative"
        aria-label="Open navigation"
      >
        <MenuIcon className="h-6 w-6 stroke-slate-500" />
      </button>
      <Dialog
        open={isOpen}
        onClose={setIsOpen}
        className="bg-netural-900/50 fixed inset-0 z-50 flex items-start overflow-y-auto pr-10 backdrop-blur lg:hidden"
        aria-label="Navigation"
      >
        <Dialog.Panel className="min-h-full w-full max-w-sm border-r border-slate-600 bg-white px-4 pb-12 pt-5 dark:bg-neutral-800 sm:px-6">
          <div className="flex items-center">
            <button
              type="button"
              onClick={() => setIsOpen(false)}
              aria-label="Close navigation"
            >
              <CloseIcon className="h-6 w-6 stroke-slate-500" />
            </button>
            <Link href="/" className="ml-6" aria-label="Home page">
              <LogoWithName product={page.product} />
            </Link>
          </div>
          {page.product.sections && page.product.sections.length > 1 && (
            <Sections
              className="-ml-2 mt-6 flex flex-col gap-2"
              sections={page.product.sections}
              activeSectionId={page.activeSection?.id}
            />
          )}
          <Navigation
            product={page.product}
            navigation={page.activeSection?.navigation ?? []}
            className="mt-12 px-1"
          />
        </Dialog.Panel>
      </Dialog>
    </>
  )
}
