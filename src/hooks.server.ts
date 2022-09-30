import type { Handle } from '@sveltejs/kit'

// export const handle: Handle = async ({ event, resolve }) => {
//   // disable SSR for all pages
//   // return await resolve(event, { ssr: false })
// }

export const handle: Handle = async ({ event, resolve }) => {
  // if (event.url.pathname.startsWith('/custom')) {
  //   return new Response('custom response')
  // }
  return await resolve(event)
}
