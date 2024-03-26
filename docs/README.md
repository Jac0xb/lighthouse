# Metaplex Developer Hub

## Getting started

To get started with this app, first install the npm dependencies:

```bash
pnpm install
cp .env.example .env.local
```

Next, run the development server:

```bash
pnpm run dev
```

Finally, open [http://localhost:3000](http://localhost:3000) in your browser to view the website.

## Customizing

You can start editing the app by modifying the files in the `/src` folder. The site will auto-update as you edit these files.

## Global search

By default this template uses [Algolia DocSearch](https://docsearch.algolia.com) for the global search. DocSearch is free for open-source projects, and you can sign up for an account on their website. Once your DocSearch account is ready, update the following [environment variables](https://nextjs.org/docs/basic-features/environment-variables) in your project with the values provided by Algolia:

```
NEXT_PUBLIC_DOCSEARCH_APP_ID=
NEXT_PUBLIC_DOCSEARCH_API_KEY=
NEXT_PUBLIC_DOCSEARCH_INDEX_NAME=
```
