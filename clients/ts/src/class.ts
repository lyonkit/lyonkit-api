import type { $Fetch } from 'ohmyfetch'
import { $fetch } from 'ohmyfetch'
import { joinURL } from 'ufo'
import type {
  BlokInput,
  BlokOutput, BlokPatchInput,
  ImageInput,
  ImageOutput,
  PageInput,
  PageOutput,
  PageOutputWithBloks, PostInput,
  PostOutput, QuoteInput,
  QuoteOutput,
} from './index'

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

  public async listPosts(): Promise<PostOutput[]> {
    return this.fetch('/post')
  }

  public async getPost(postId: number): Promise<PostOutput> {
    return this.fetch(`/post/${postId}`)
  }

  public async getPostBySlug(postSlug: string): Promise<PostOutput> {
    return this.fetch(`/post/s/${postSlug}`)
  }

  // QUOTES

  public async listQuotes(): Promise<QuoteOutput[]> {
    return this.fetch('/quote')
  }

  public async getQuote(quoteId: number): Promise<PostOutput> {
    return this.fetch(`/quote/${quoteId}`)
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

  public async patchBlok({ blokId, patch }: { blokId: number; patch: BlokPatchInput }): Promise<BlokOutput> {
    return this.fetch(`/blok/${blokId}`, { method: 'PATCH', body: patch })
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

  // QUOTE

  public async createQuote(quote: QuoteInput): Promise<QuoteOutput> {
    return this.fetch('/quote', { method: 'POST', body: quote })
  }

  public async updateQuote({ quoteId, update }: { quoteId: number; update: QuoteInput }): Promise<QuoteOutput> {
    return this.fetch(`/quote/${quoteId}`, { method: 'PUT', body: update })
  }

  public async deleteQuote(postId: number): Promise<QuoteOutput> {
    return this.fetch(`/quote/${postId}`, { method: 'DELETE' })
  }

  // GIT JSON FILE
  public async getGitJsonFile<T = any>(path: string): Promise<T> {
    return this.fetch(`/git/json-file/${path}`)
  }

  public async updateGitJsonFile<T = any, U = any>(path: string, update: U): Promise<T> {
    return this.fetch(`/git/json-file/${path}`, {
      method: 'PUT',
      body: update,
    })
  }
}
