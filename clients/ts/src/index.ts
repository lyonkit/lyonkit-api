import type { $Fetch } from 'ohmyfetch'
import { $fetch } from 'ohmyfetch'
import { joinURL } from 'ufo'

export interface ImageInput {
  alt?: string | null | undefined
  image: Blob | File
}

export interface ImageOutput {
  id: number
  publicUrl: string
  lazyImage: {
    id: number
    publicUrl: string
    alt: string | null
    createdAt: string
    updatedAt: string
  }
  alt: string | null
  createdAt: string
  updatedAt: string
}

export interface PageInput {
  title: string
  description?: string | undefined | null
  path: string
}
export interface PageOutput {
  id: number
  title: string
  description: string | null
  namespace: string
  path: string
  createdAt: string
  updatedAt: string
}
export interface PageOutputWithBloks extends PageOutput {
  bloks: BlokOutput[]
}

export interface BlokInput {
  pageId: number
  componentId: string
  props: Record<string, any>
  priority?: number | null | undefined
}
export interface BlokPatchInput {
  pageId?: number | undefined
  componentId?: string | undefined
  props?: Record<string, any> | undefined
  priority?: number | undefined
}
export interface BlokOutput {
  id: number
  pageId: number
  componentId: string
  props: Record<string, any>
  priority: number
  createdAt: string
  updatedAt: string
}

export interface PostInput {
  title: string
  description: string
  slug: string
  body: any
}
export interface PostOutput {
  id: number
  title: string
  description: string | null
  slug: string
  namespace: string
  body: any
  createdAt: string
  updatedAt: string
}

export interface QuoteInput {
  author: string
  message: string
}
export interface QuoteOutput {
  id: number
  namespace: string
  author: string
  message: string
  createdAt: string
  updatedAt: string
}

export type Locales = Record<string, any>

export interface LocaleOutput {
  id: number
  namespace: string
  lang: String
  createdAt: String
  updatedAt: String
}

interface LyonkitClientOptions { endpoint?: string; apiKey: string }

export function createLyonkitReadonlyApiClient({ endpoint = 'https://lyonkit.leo-coletta.fr', apiKey }: LyonkitClientOptions) {
  const fetchClient: $Fetch = $fetch.create({
    baseURL: joinURL(endpoint, '/api'),
    headers: {
      'x-api-key': apiKey,
    },
  })

  // IMAGES

  async function listImages(): Promise<ImageOutput[]> {
    return fetchClient('/image')
  }

  // PAGES

  async function listPages(): Promise<PageOutput[]> {
    return fetchClient('/page')
  }

  async function getPage(path: string): Promise<PageOutputWithBloks> {
    return fetchClient(`/page/wb${path}`)
  }

  // BLOKS

  async function getBlok(blokId: number): Promise<BlokOutput> {
    return fetchClient(`/blok/${blokId}`)
  }

  // POSTS

  async function listPosts(): Promise<PostOutput[]> {
    return fetchClient('/post')
  }

  async function getPost(postId: number): Promise<PostOutput> {
    return fetchClient(`/post/${postId}`)
  }

  async function getPostBySlug(postSlug: string): Promise<PostOutput> {
    return fetchClient(`/post/s/${postSlug}`)
  }

  // QUOTES

  async function listQuotes(): Promise<QuoteOutput[]> {
    return fetchClient('/quote')
  }

  async function getQuote(quoteId: number): Promise<QuoteOutput> {
    return fetchClient(`/quote/${quoteId}`)
  }

  // LOCALES
  async function getLocales(): Promise<Locales> {
    return fetchClient('/locale')
  }

  return {
    fetchClient,
    listImages,
    listPages,
    getPage,
    getBlok,
    listPosts,
    getPost,
    getPostBySlug,
    listQuotes,
    getQuote,
    getLocales,
  }
}

export function createLyonkitWriteApiClient(options: LyonkitClientOptions) {
  const { fetchClient, ...readonlyMethods } = createLyonkitReadonlyApiClient(options)

  // IMAGES

  async function createImage({ image, alt }: ImageInput): Promise<ImageOutput> {
    const form = new FormData()
    form.set('image', image)
    return fetchClient('/image', { params: { alt }, method: 'POST', body: form })
  }

  async function deleteImage(imageId: number): Promise<ImageOutput> {
    return fetchClient(`/image/${imageId}`, { method: 'DELETE' })
  }

  // PAGES

  async function createPage(page: PageInput): Promise<PageOutput> {
    return fetchClient('/page', { method: 'POST', body: page })
  }

  async function updatePage({ pageId, update }: { pageId: number; update: PageInput }): Promise<PageOutput> {
    return fetchClient(`/page/${pageId}`, { method: 'PUT', body: update })
  }

  async function deletePage(pageId: number): Promise<PageOutput> {
    return fetchClient(`/page/${pageId}`, { method: 'DELETE' })
  }

  // BLOKS

  async function createBlok(blok: BlokInput): Promise<BlokOutput> {
    return fetchClient('/blok', { method: 'POST', body: blok })
  }

  async function updateBlok({ blokId, update }: { blokId: number; update: BlokInput }): Promise<BlokOutput> {
    return fetchClient(`/blok/${blokId}`, { method: 'PUT', body: update })
  }

  async function patchBlok({ blokId, patch }: { blokId: number; patch: BlokPatchInput }): Promise<BlokOutput> {
    return fetchClient(`/blok/${blokId}`, { method: 'PATCH', body: patch })
  }

  async function deleteBlok(blokId: number): Promise<BlokOutput> {
    return fetchClient(`/blok/${blokId}`, { method: 'DELETE' })
  }

  // POST

  async function createPost(post: PostInput): Promise<PostOutput> {
    return fetchClient('/post', { method: 'POST', body: post })
  }

  async function updatePost({ postId, update }: { postId: number; update: PostInput }): Promise<PostOutput> {
    return fetchClient(`/post/${postId}`, { method: 'PUT', body: update })
  }

  async function deletePost(postId: number): Promise<PostOutput> {
    return fetchClient(`/post/${postId}`, { method: 'DELETE' })
  }

  // QUOTE

  async function createQuote(quote: QuoteInput): Promise<QuoteOutput> {
    return fetchClient('/quote', { method: 'POST', body: quote })
  }

  async function updateQuote({ quoteId, update }: { quoteId: number; update: QuoteInput }): Promise<QuoteOutput> {
    return fetchClient(`/quote/${quoteId}`, { method: 'PUT', body: update })
  }

  async function deleteQuote(postId: number): Promise<QuoteOutput> {
    return fetchClient(`/quote/${postId}`, { method: 'DELETE' })
  }

  // GIT JSON FILE
  async function getGitJsonFile<T = any>(path: string): Promise<T> {
    return fetchClient(`/git/json-file/${path}`)
  }

  async function updateGitJsonFile<T = any, U = any>(path: string, update: U): Promise<T> {
    return fetchClient(`/git/json-file/${path}`, {
      method: 'PUT',
      body: update,
    })
  }

  // LOCALES
  async function updateLocale(lang: string, messages: any): Promise<LocaleOutput> {
    return fetchClient(`/locale/${lang}`, {
      method: 'PUT',
      body: messages,
    })
  }

  return {
    fetchClient,
    ...readonlyMethods,
    createImage,
    deleteImage,
    createPage,
    updatePage,
    deletePage,
    createBlok,
    updateBlok,
    patchBlok,
    deleteBlok,
    createPost,
    updatePost,
    deletePost,
    createQuote,
    updateQuote,
    deleteQuote,
    getGitJsonFile,
    updateGitJsonFile,
    updateLocale,
  }
}
