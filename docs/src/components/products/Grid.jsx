import clsx from 'clsx'
import { LogoWithName } from './Logo'
import { products as allProducts } from './index'
import Link from 'next/link'

export function Grid({
  onClick,
  withoutFallback,
  className,
  menuItem,
  ...props
}) {
  console.log('menuItem', menuItem)
  const products =  allProducts.filter(
        (product) => menuItem === product.navigationMenuCatergory
      )
    
  return (
    <ul className={clsx(['grid grid-flow-row gap-3', className])} {...props}>
      {products.map((product) => (
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
