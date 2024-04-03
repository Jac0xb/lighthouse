import Link from 'next/link'
import clsx from 'clsx'

const styles = {
  primary:
    'rounded-md bg-accent-300 py-2 px-4 text-sm font-semibold text-slate-900 hover:bg-accent-200 focus:outline-none focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent-300/50 active:bg-accent-500',
  secondary:
    'rounded-md py-2 px-4 text-sm font-medium text-white hover:bg-slate-800 focus:outline-none focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-white/50 active:text-slate-400',
}

export function Button({ variant = 'primary', className, href, ...props }) {
  className = clsx(styles[variant], className)

  return href ? (
    <Link href={href} className={className} {...props} />
  ) : (
    <button className={className} {...props} />
  )
}
