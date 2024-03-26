import clsx from 'clsx'
import { LogoWithName } from './Logo'
import { products as allProducts } from './index'
import Link from 'next/link'
import { Grid } from './Grid'

export function MobileAppGrid({
  onClick,
  withoutFallback,
  className,
  menuItem,
  ...props
}) {
  console.log('menuItem', menuItem)
  const products = allProducts

  const hub = products.find((product) => product.name === 'Metaplex')

  return (
    <ul className={clsx(['grid grid-flow-row gap-5', className])} {...props}>
      <li key={hub.path}>
        <Link
          href={`/${hub.path}`}
          className="block rounded-lg p-3 hover:bg-slate-50 hover:dark:bg-slate-700 sticky"
          onClick={onClick}
        >
          <LogoWithName product={hub}></LogoWithName>
        </Link>
        <hr />
      </li>
      
    <div className='overflow-y-auto'>
      {products
        .filter((product) => product.name != 'Metaplex')
        .map((product) => (
          <li key={product.path}>
            <Link
              href={`/${product.path}`}
              className="block rounded-lg p-3 hover:bg-slate-50 hover:dark:bg-slate-700"
              onClick={onClick}
            >
              <LogoWithName product={product}></LogoWithName>
            </Link>
          </li>
        ))}
        </div>
    </ul>
  )
}

export function MarkdocGrid() {
  return (
    <div className="not-prose">
      <Grid
        className="relative md:grid-flow-col md:grid-cols-4 md:grid-rows-4"
        withoutFallback
      />
    </div>
  )
}
