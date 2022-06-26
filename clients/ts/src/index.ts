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
export interface PageOuput {
  id: number
  title: string
  description: string | null
  namespace: string
  path: string
  createdAt: string
  updatedAt: string
}
export interface PageOuputWithBloks extends PageOuput {
  bloks: BlokOuput[]
}

export interface BlokInput {
  pageId: number
  componentId: string
  props: Record<string, any>
  priority?: number | null | undefined
}
export interface BlokOuput {
  id: number
  pageId: number
  componentId: string
  props: Record<string, any>
  priority: number
  createdAt: string
  updatedAt: string
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

  public async listPages(): Promise<PageOuput[]> {
    return this.fetch('/page')
  }

  public async getPage(path: string): Promise<PageOuputWithBloks> {
    return this.fetch('/page/wb', { params: { path } })
  }

  // BLOKS

  public async getBlok(blokId: number): Promise<BlokOuput> {
    return this.fetch(`/blok/${blokId}`)
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

  public async createPage(page: PageInput): Promise<PageOuput> {
    return this.fetch('/page', { method: 'POST', body: page })
  }

  public async updatePage({ pageId, update }: { pageId: number; update: PageInput }): Promise<PageOuput> {
    return this.fetch(`/page/${pageId}`, { method: 'PUT', body: update })
  }

  public async deletePage(pageId: number): Promise<PageOuput> {
    return this.fetch(`/page/${pageId}`, { method: 'DELETE' })
  }

  // BLOKS

  public async createBlok(blok: BlokInput): Promise<BlokOuput> {
    return this.fetch('/blok', { method: 'POST', body: blok })
  }

  public async updateBlok({ blokId, update }: { blokId: number; update: BlokInput }): Promise<BlokOuput> {
    return this.fetch(`/blok/${blokId}`, { method: 'PUT', body: update })
  }

  public async deleteBlok(blokId: number): Promise<BlokOuput> {
    return this.fetch(`/blok/${blokId}`, { method: 'DELETE' })
  }
}
