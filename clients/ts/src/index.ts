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
  title: string,
  description: string,
  body: any,
}
export interface PostOutput {
  id: number,
  title: string,
  description: string | null,
  namespace: string,
  body: any,
  created_at: string,
  updated_at: string,
}

export class LyonkitReadonlyApiClient {
  public readonly endpoint: string = 'https://lyonkit.leo-coletta.fr'
  protected readonly apiKey: string
  protected readonly fetch: $Fetch

  constructor(params: { endpoint?: string; apiKey: string }) {
    if (params.endpoint)
      this.endpoint = params.endpoint

    this.apiKey = params.apiKey
    this.fetch = $fetch.create({
      baseURL: joinURL(this.endpoint, '/api'),
      headers: {
        'x-api-key': this.apiKey,
      },
    })
  }

  // IMAGES

  public async listImages(): Promise<ImageOutput[]> {
    return this.fetch('/image')
  }

  // PAGES

  public async listPages(): Promise<PageOutput[]> {
    return this.fetch('/page')
  }

  public async getPage(path: string): Promise<PageOutputWithBloks> {
    return this.fetch(`/page/wb${path}`)
  }

  // BLOKS

  public async getBlok(blokId: number): Promise<BlokOutput> {
    return this.fetch(`/blok/${blokId}`)
  }

  // POSTS

  public async listPosts(): Promise<PostOutput> {
    return this.fetch('/post')
  }

  public async getPost(postId: number): Promise<PostOutput> {
    return this.fetch(`/post/${postId}`)
  }
}

export class LyonkitWriteApiClient extends LyonkitReadonlyApiClient {
  // IMAGES

  public async createImage({ image, alt }: ImageInput): Promise<ImageOutput> {
    const form = new FormData()
    form.set('image', image)
    return this.fetch('/image', { params: { alt }, method: 'POST', body: form })
  }

  public async deleteImage(imageId: number): Promise<ImageOutput> {
    return this.fetch(`/image/${imageId}`, { method: 'DELETE' })
  }

  // PAGES

  public async createPage(page: PageInput): Promise<PageOutput> {
    return this.fetch('/page', { method: 'POST', body: page })
  }

  public async updatePage({ pageId, update }: { pageId: number; update: PageInput }): Promise<PageOutput> {
    return this.fetch(`/page/${pageId}`, { method: 'PUT', body: update })
  }

  public async deletePage(pageId: number): Promise<PageOutput> {
    return this.fetch(`/page/${pageId}`, { method: 'DELETE' })
  }

  // BLOKS

  public async createBlok(blok: BlokInput): Promise<BlokOutput> {
    return this.fetch('/blok', { method: 'POST', body: blok })
  }

  public async updateBlok({ blokId, update }: { blokId: number; update: BlokInput }): Promise<BlokOutput> {
    return this.fetch(`/blok/${blokId}`, { method: 'PUT', body: update })
  }

  public async deleteBlok(blokId: number): Promise<BlokOutput> {
    return this.fetch(`/blok/${blokId}`, { method: 'DELETE' })
  }

  // POST

  public async createPost(post: PostInput): Promise<PostOutput> {
    return this.fetch('/post', { method: 'POST', body: post })
  }

  public async updatePost({ postId, update }: { postId: number; update: PostInput }): Promise<PostOutput> {
    return this.fetch(`/post/${postId}`, { method: 'PUT', body: update })
  }

  public async deletePost(postId: number): Promise<PostOutput> {
    return this.fetch(`/post/${postId}`, { method: 'DELETE' })
  }
}
