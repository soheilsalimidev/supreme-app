import { createHighlighterCore } from 'shiki/core'
import { createOnigurumaEngine } from 'shiki/engine/oniguruma'

export const highlighter = createHighlighterCore({
  themes: [
    import('shiki/themes/tokyo-night.mjs')
  ],
  langs: [
    import('shiki/langs/json.mjs'),
  ],
  engine: createOnigurumaEngine(import('shiki/wasm'))
})
