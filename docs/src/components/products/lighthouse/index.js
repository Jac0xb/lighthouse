import { Hero } from './Hero'
import { Logo } from './Logo'

export const lighthouse = {
  name: 'Lighthouse',
  headline: '',
  description: 'The Assertion Protocol.',
  navigationMenuCatergory: '',
  path: '',
  logo: Logo,
  github: 'https://github.com/Jac0xb/lighthouse',
  className: 'accent-teal',
  heroes: [{ path: '/', component: Hero }],
  sections: [
    {
      id: 'documentation',
      title: 'Documentation',
      icon: 'SolidBookOpen',
      href: ``,
      isFallbackSection: true,
      navigation: [
        {
          title: 'Introduction',
          links: [
            { title: 'Overview', href: '/' },
            {
              title: 'Getting Started',
              href: '/getting-started',
            },
          ],
        },
        {
          title: 'Features',
          links: [
            { title: 'Assert', href: '/assert' },
            { title: 'Write', href: '/write' },
          ],
        },
        {
          title: 'Assertions',
          links: [
            { title: 'Account Data', href: '/assert/account-data' },
            { title: 'Account Info', href: '/assert/account-info' },
            { title: 'Account Delta', href: '/assert/account-delta' },
            {
              title: 'Token Mint',
              href: '/assert/mint-account',
            },
            {
              title: 'Token Account',
              href: '/assert/token-account',
            },
            {
              title: 'Stake Account',
              href: '/assert/stake-account',
            },
            {
              title: 'Upgradeable Loader Account',
              href: '/assert/upgradeable-loader-account',
            },
            {
              title: 'Merkle Tree Account',
              href: '/assert/merkle-tree-account',
            },
          ],
        },
      ],
    },
  ],
}
