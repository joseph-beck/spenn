/* prettier-ignore-start */

/* eslint-disable */

// @ts-nocheck

// noinspection JSUnusedGlobalSymbols

// This file is auto-generated by TanStack Router

import { createFileRoute } from '@tanstack/react-router'

// Import Routes

import { Route as rootRoute } from './routes/__root'

// Create Virtual Routes

const SpendingLazyImport = createFileRoute('/spending')()
const MacsLazyImport = createFileRoute('/macs')()
const AboutLazyImport = createFileRoute('/about')()
const IndexLazyImport = createFileRoute('/')()
const SpendingUuidLazyImport = createFileRoute('/spending/$uuid')()

// Create/Update Routes

const SpendingLazyRoute = SpendingLazyImport.update({
  path: '/spending',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/spending.lazy').then((d) => d.Route))

const MacsLazyRoute = MacsLazyImport.update({
  path: '/macs',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/macs.lazy').then((d) => d.Route))

const AboutLazyRoute = AboutLazyImport.update({
  path: '/about',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/about.lazy').then((d) => d.Route))

const IndexLazyRoute = IndexLazyImport.update({
  path: '/',
  getParentRoute: () => rootRoute,
} as any).lazy(() => import('./routes/index.lazy').then((d) => d.Route))

const SpendingUuidLazyRoute = SpendingUuidLazyImport.update({
  path: '/spending/$uuid',
  getParentRoute: () => rootRoute,
} as any).lazy(() =>
  import('./routes/spending_.$uuid.lazy').then((d) => d.Route),
)

// Populate the FileRoutesByPath interface

declare module '@tanstack/react-router' {
  interface FileRoutesByPath {
    '/': {
      preLoaderRoute: typeof IndexLazyImport
      parentRoute: typeof rootRoute
    }
    '/about': {
      preLoaderRoute: typeof AboutLazyImport
      parentRoute: typeof rootRoute
    }
    '/macs': {
      preLoaderRoute: typeof MacsLazyImport
      parentRoute: typeof rootRoute
    }
    '/spending': {
      preLoaderRoute: typeof SpendingLazyImport
      parentRoute: typeof rootRoute
    }
    '/spending/$uuid': {
      preLoaderRoute: typeof SpendingUuidLazyImport
      parentRoute: typeof rootRoute
    }
  }
}

// Create and export the route tree

export const routeTree = rootRoute.addChildren([
  IndexLazyRoute,
  AboutLazyRoute,
  MacsLazyRoute,
  SpendingLazyRoute,
  SpendingUuidLazyRoute,
])

/* prettier-ignore-end */
