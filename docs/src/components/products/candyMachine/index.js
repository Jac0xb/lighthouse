import { documentationSection, referencesSection } from '@/shared/sections'
import { Hero } from './Hero'
import { Logo } from './Logo'

export const candyMachine = {
  name: 'Lighthouse',
  headline: '',
  description: 'The Assertion Protocol.',
  navigationMenuCatergory: '',
  path: '',
  // logo: Logo,
  github: 'https://github.com/Jac0xb/lighthouse',
  className: 'accent-teal',
  heroes: [{ path: '/', component: Hero }],
  sections: [
    {
      ...documentationSection(''),
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
              title: 'SPL-Token Mint',
              href: '/assert/spl-token-mint',
            },
            {
              title: 'SPL-Token Account',
              href: '/assert/spl-token-account',
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
        // {
        //   title: 'Old',
        //   links: [
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //     // { title: 'Old', href: '/old' },
        //   ],
        // },
        // {
        //   title: 'Features',
        //   links: [
        //     {
        //       title: 'Candy Machine Settings',
        //       href: '/settings',
        //     },
        //     { title: 'Managing Candy Machines', href: '/manage' },
        //     { title: 'Inserting Items', href: '/insert-items' },
        //     { title: 'Candy Guards', href: '/guards' },
        //     { title: 'Guard Groups', href: '/guard-groups' },
        //     {
        //       title: 'Special Guard Instructions',
        //       href: '/guard-route',
        //     },
        //     { title: 'Minting', href: '/mint' },
        //     { title: 'Programmable NFTs', href: '/pnfts' },
        //   ],
        // },
        // {
        //   title: 'Available Guards',
        //   links: [
        //     {
        //       title: 'Address Gate',
        //       href: '/guards/address-gate',
        //     },
        //     { title: 'Allocation', href: '/guards/allocation' },
        //     { title: 'Allow List', href: '/guards/allow-list' },
        //     { title: 'Bot Tax', href: '/guards/bot-tax' },
        //     { title: 'End Date', href: '/guards/end-date' },
        //     {
        //       title: 'Freeze Sol Payment',
        //       href: '/guards/freeze-sol-payment',
        //     },
        //     {
        //       title: 'Freeze Token Payment',
        //       href: '/guards/freeze-token-payment',
        //     },
        //     { title: 'Gatekeeper', href: '/guards/gatekeeper' },
        //     { title: 'Mint Limit', href: '/guards/mint-limit' },
        //     { title: 'NFT Burn', href: '/guards/nft-burn' },
        //     { title: 'NFT Gate', href: '/guards/nft-gate' },
        //     { title: 'NFT Payment', href: '/guards/nft-payment' },
        //     {
        //       title: 'Program Gate',
        //       href: '/guards/program-gate',
        //     },
        //     {
        //       title: 'Redeemed Amount',
        //       href: '/guards/redeemed-amount',
        //     },
        //     { title: 'Sol Payment', href: '/guards/sol-payment' },
        //     { title: 'Start Date', href: '/guards/start-date' },
        //     {
        //       title: 'Third Party Signer',
        //       href: '/guards/third-party-signer',
        //     },
        //     { title: 'Token Burn', href: '/guards/token-burn' },
        //     { title: 'Token Gate', href: '/guards/token-gate' },
        //     {
        //       title: 'Token Payment',
        //       href: '/guards/token-payment',
        //     },
        //     {
        //       title: 'Token2022 Payment',
        //       href: '/guards/token2022-payment',
        //     },
        //   ],
        // },
        // {
        //   title: 'Custom Guards',
        //   links: [
        //     {
        //       title: 'Writing a Custom Guard',
        //       href: '/custom-guards/writing-custom-guards',
        //     },
        //     {
        //       title: 'Generating Client',
        //       href: '/custom-guards/generating-client',
        //     },
        //   ],
        // },
      ],
    },
    {
      id: 'sugar',
      title: 'Sugar',
      icon: 'SolidCake',
      href: `/sugar`,
      navigation: [
        {
          title: 'Introduction',
          links: [
            { title: 'Overview', href: '/sugar' },
            {
              title: 'Installation',
              href: '/sugar/installation',
            },
            {
              title: 'Getting Started',
              href: '/sugar/getting-started',
            },
          ],
        },
        {
          title: 'Working with Sugar',
          links: [
            {
              title: 'Configuration File',
              href: '/sugar/configuration',
            },
            {
              title: 'Cache file',
              href: '/sugar/cache',
            },
          ],
        },
        {
          title: 'Commands',
          links: [
            { title: 'airdrop', href: '/sugar/commands/airdrop' },
            { title: 'bundlr', href: '/sugar/commands/bundlr' },
            {
              title: 'collection',
              href: '/sugar/commands/collection',
            },
            { title: 'config', href: '/sugar/commands/config' },
            { title: 'deploy', href: '/sugar/commands/deploy' },
            { title: 'freeze', href: '/sugar/commands/freeze' },
            { title: 'guard', href: '/sugar/commands/guard' },
            { title: 'hash', href: '/sugar/commands/hash' },
            { title: 'launch', href: '/sugar/commands/launch' },
            { title: 'mint', href: '/sugar/commands/mint' },
            { title: 'reveal', href: '/sugar/commands/reveal' },
            { title: 'show', href: '/sugar/commands/show' },
            { title: 'sign', href: '/sugar/commands/sign' },
            { title: 'update', href: '/sugar/commands/update' },
            { title: 'upload', href: '/sugar/commands/upload' },
            {
              title: 'validate',
              href: '/sugar/commands/validate',
            },
            { title: 'verify', href: '/sugar/commands/verify' },
            {
              title: 'withdraw',
              href: '/sugar/commands/withdraw',
            },
          ],
        },
        {
          title: 'References',
          links: [
            {
              title: 'Bring Your Own Uploader',
              href: '/sugar/bring-your-own-uploader',
            },
          ],
        },
      ],
    },
    // { ...referencesSection('candy-machine') },
    /*
    {
      ...recipesSection('candy-machine'),
      navigation: [
        {
          title: 'How to create a Candy Machine',
          links: [
            { title: 'Part 1 (Sugar)', href: '/recipes/todo' },
            { title: 'Part 2 (Umi)', href: '/recipes/todo' },
            { title: 'Part 2 (JS SDK)', href: '/recipes/todo' },
          ],
        },
      ],
    },
    { ...changelogSection('candy-machine') },
    */
  ],
}
