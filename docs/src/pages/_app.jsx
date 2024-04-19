import Head from 'next/head'
import Script from 'next/script'

import { Layout } from '@/components/Layout'
import { DialectProvider } from '@/components/DialectContext'
import { usePage } from '@/shared/usePage'

import '@/styles/tailwind.css'
import 'focus-visible'
import 'reactflow/dist/base.css'

// Add Prism components.
import { Prism } from 'prism-react-renderer'
;(typeof global !== 'undefined' ? global : window).Prism = Prism
require('prismjs/components/prism-rust')

export default function App({ Component, pageProps }) {
  const page = usePage(pageProps)
  console.log({ page })

  return (
    <>
      <Head>
        <title>{page.metaTitle}</title>
        <meta property="og:title" content={page.metaTitle} />
        <meta name="twitter:title" content={page.metaTitle} />
        <meta name="twitter:card" content="summary_large_image" />
        <meta property="twitter:domain" content="http://lighthaus.voyage/" />
        <meta
          property="og:image"
          content="https://lighthaus.voyage/assets/lighthaus_widescreen.png"
        />
        <meta
          name="twitter:image"
          content="https://lighthaus.voyage/assets/lighthaus_widescreen.png"
        />

        {page.description && (
          <>
            <meta name="description" content={page.description} />
            <meta property="og:description" content={page.description} />
            <meta name="twitter:description" content={page.description} />
          </>
        )}
      </Head>

      <DialectProvider>
        <Layout page={page}>
          <Component {...pageProps} />
        </Layout>
      </DialectProvider>
    </>
  )
}
