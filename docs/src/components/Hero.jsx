import Image from 'next/image'

import { Button } from '@/components/Button'
import blurCyanImage from '@/images/blur-cyan.png'
import blurIndigoImage from '@/images/blur-indigo.png'
import clsx from 'clsx'

export function Hero({
  page,
  title,
  description,
  subDescription,
  primaryCta,
  secondaryCta,
  light1Off,
  light2Off,
  light3Off,
  children,
}) {
  title = title ?? page.product.name
  description = description ?? page.product.description
  primaryCta = primaryCta ?? {
    title: 'Get started',
    href: `/${page.product.path}/getting-started`,
  }
  secondaryCta =
    secondaryCta ??
    (page.product.github
      ? {
          title: 'View on GitHub',
          href: page.product.github,
        }
      : undefined)

  return (
    <div className="overflow-hidden dark:-mb-32 dark:mt-[-7rem] dark:bg-neutral-900 dark:pb-32 dark:pt-[7rem] dark:lg:mt-[-7.25rem] dark:lg:pt-[7.25rem]">
      <div className="py-16 sm:px-2 lg:relative lg:px-0 lg:py-20">
        <div
          className={clsx(
            'mx-auto grid max-w-2xl grid-cols-1 items-center px-4 lg:max-w-8xl lg:grid-cols-2 lg:px-8 xl:px-12',
            { 'gap-x-8 gap-y-16 xl:gap-x-16': children }
          )}
        >
          <div className="relative z-10 md:text-center lg:text-left">
            {!light1Off && (
              <Image
                className="no-lightense absolute bottom-full right-full -mb-56 -mr-72 opacity-25"
                src={blurCyanImage}
                alt=""
                width={830}
                height={830}
                unoptimized
                priority
              />
            )}
            <div className="relative">
              <p className=" inline bg-gradient-to-r from-indigo-200 via-accent-400 to-indigo-200 bg-clip-text font-display text-5xl tracking-tight text-transparent">
                {title}
              </p>
              <p className="mt-3 text-2xl tracking-tight text-slate-400">
                {description}
              </p>
              {subDescription && (
                <p className="mt-2 text-base tracking-tight text-slate-400">
                  {subDescription}
                </p>
              )}
              <div className="mt-8 flex gap-4 md:justify-center lg:justify-start">
                <Button href={primaryCta.href}>{primaryCta.title}</Button>
                {secondaryCta && (
                  <Button href={secondaryCta.href} variant="secondary">
                    {secondaryCta.title}
                  </Button>
                )}
              </div>
            </div>
          </div>
          <div className="relative lg:static xl:pl-10">
            <div className="relative">
              {!light2Off && (
                <Image
                  className="no-lightense absolute -right-64 -top-64 opacity-10"
                  src={blurCyanImage}
                  alt=""
                  width={530}
                  height={530}
                  unoptimized
                  priority
                />
              )}
              {!light3Off && (
                <Image
                  className="no-lightense absolute -bottom-40 -right-44 opacity-10"
                  src={blurIndigoImage}
                  alt=""
                  width={567}
                  height={567}
                  unoptimized
                  priority
                />
              )}
              {children}
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
